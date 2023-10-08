mod scanner;

use fastnbt::from_bytes;
use itertools::Itertools;
use scanner::blocks::{BlockDescriptor, BlockEntity};
use serde::Deserialize;
use std::{
    cmp::Ordering,
    collections::HashSet,
    env,
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use fastanvil::*;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use wildmatch::WildMatch;

use crate::scanner::{bounds::Bounds, points::Point, scan::ScanResult, veins::find_veins};

fn main() {
    _ = match try_main() {
        Ok(_) => return,
        Err(err) if err.kind() == std::io::ErrorKind::BrokenPipe => return,
        Err(err) => writeln!(std::io::stderr(), "{}", err),
    };
}

fn try_main() -> std::result::Result<(), io::Error> {
    let mut stdout = io::stdout();

    let default_home = "0,8,0".to_owned();
    let default_threshold = "100,8,100".to_owned();

    let args = env::args().collect::<Vec<String>>();

    let dim_path = PathBuf::from(args[1].clone());
    let search_block = args[2].clone();
    let home_str = args
        .get(3)
        .unwrap_or(&default_home)
        .split(',')
        .collect_vec(); // .unwrap_or(&"0,0,0".to_owned()).split(",").collect_vec();
    let threshold_str = args
        .get(4)
        .unwrap_or(&default_threshold)
        .split(',')
        .collect_vec();

    let home = Point {
        x: home_str[0].parse().unwrap(),
        y: home_str[1].parse().unwrap(),
        z: home_str[2].parse().unwrap(),
    };

    let threshold = Point {
        x: threshold_str[0].parse().unwrap(),
        y: threshold_str[1].parse().unwrap(),
        z: threshold_str[2].parse().unwrap(),
    };

    let boundary = Bounds::from_point(&home).expand(&threshold);

    let block_matcher = WildMatch::new(&search_block);
    writeln!(stdout, "Looking for {}", search_block)?;

    let loader = RegionFileLoader::new(dim_path.join("region"));
    let regions = loader.list().unwrap();
    let regions_len = regions.len();
    writeln!(stdout, "Scanning {} regions", regions_len)?;

    // Search for the specified block in each region
    let results = regions
        .into_par_iter()
        .map(|(rx, rz)| {
            let mut file = loader.region(rx, rz).unwrap().unwrap();


            locate_in_region(block_matcher.clone(), &mut file, rx.0 as i64, rz.0 as i64)
        })
        .filter_map(Option::Some)
        .collect::<Vec<ScanResult>>();

    // Summarize the results
    let summary = ScanResult::combine(results.into_iter()).replacing_blocks(|blocks| {
        blocks
            .into_iter()
            .filter(|block| boundary.contains(&block.pos()))
            .sorted()
    });

    let all_block_types =
        HashSet::<String>::from_iter(summary.found.iter().map(|b| b.name.clone()));

    writeln!(stdout, "Scanned {} regions", summary.regions)?;
    writeln!(stdout, "Scanned {} chunks", summary.chunks)?;
    writeln!(stdout, "Scanned {} sections", summary.sections)?;
    writeln!(stdout, "Scanned {} blocks", summary.blocks)?;
    writeln!(stdout, "Types: {}", all_block_types.into_iter().join(", "))?;

    writeln!(
        stdout,
        "Found {} blocks of {}",
        summary.found.len(),
        search_block
    )?;

    // Match blocks to their entities

    for desc in &summary.found {
        let block_pos = Point {
            x: desc.x,
            y: desc.y,
            z: desc.z,
        };
        let _distance = block_pos.distance_to(&home);

        // writeln!(stdout, "  {} at {}, {}, {} -> {} blocks away", desc.name, desc.x, desc.y, desc.z, distance)?;

        if let Some(entity) = &desc.enity {
            if entity.id != "minecraft:mob_spawner" {
                continue;
            }

            writeln!(
                stdout,
                "    spawn delay: {}-{} ticks",
                entity.min_spawn_delay.unwrap(),
                entity.max_spawn_delay.unwrap()
            )?;
            writeln!(
                stdout,
                "    spawn count: {} @ {} blocks",
                entity.spawn_count.unwrap(),
                entity.spawn_range.unwrap()
            )?;
            writeln!(stdout, "    max entities: {}", entity.max_entities.unwrap())?;
            writeln!(
                stdout,
                "    activation range: {} blocks",
                entity.activation_range.unwrap()
            )?;
        }
    }

    // writeln!(stdout, "Looking for veins")?;
    let veins = find_veins(summary.found);

    for (num, vein) in veins
        .sorted_by(|a, b| {
            let a_dist = a.center.distance_to(&home);
            let b_dist = b.center.distance_to(&home);
            let a_count = a.blocks.len();
            let b_count = b.blocks.len();

            Ordering::Equal
                .then_with(|| a_dist.cmp(&b_dist))
                .then_with(|| b_count.cmp(&a_count))
        })
        .enumerate()
    {
        writeln!(
            stdout,
            "Vein {} ({} blocks, {}m away): {}",
            num,
            vein.blocks.len(),
            vein.center.distance_to(&home),
            vein.bounds
        )?;
    }

    Ok(())
}

/// A Minecraft chunk.
#[derive(Deserialize, Debug)]
pub struct ChunkEntityContainer {
    #[serde(rename = "block_entities")]
    pub block_entities: Vec<BlockEntity>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SpawnData {
    #[serde(rename = "entity")]
    pub entity: SpawnEntity,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SpawnEntity {
    #[serde(rename = "id")]
    pub id: String,
}

fn locate_in_region(name: WildMatch, region: &mut Region<File>, rx: i64, rz: i64) -> ScanResult {
    let chunk_coords = (0..32i64).flat_map(|z| (0..32i64).map(move |x| (x, z)));

    let mut result = ScanResult {
        regions: 1,
        chunks: 0,
        sections: 0,
        blocks: 0,
        found: vec![],
    };

    for (cx, cz) in chunk_coords {
        let chunk = region.read_chunk(cx as usize, cz as usize);
        let chunk = match chunk {
            Ok(Some(data)) => Some(data),
            _ => None,
        };

        let matching_entities = chunk
            .clone()
            .and_then(|data| from_bytes::<ChunkEntityContainer>(&data).ok())
            .map_or(vec![], |container| container.block_entities)
            .into_iter()
            .filter(|entity| name.matches(&entity.id))
            .map(move |entity| BlockDescriptor {
                name: entity.id.clone(),
                x: entity.x,
                y: entity.y,
                z: entity.z,
                enity: Some(entity),
            })
            .collect_vec();

        result.found.extend(matching_entities);

        // let blaze_spawners = block_entities
        //     .into_iter()
        //     .filter(|entity| {
        //         return match entity.spawn_data {
        //             Some(ref spawn_data) => {
        //                 match spawn_data.entity.id.as_str() {
        //                     "minecraft:blaze" => true,
        //                     _ => false,
        //                 }
        //             },
        //             None => false,
        //         }
        //     })
        //     .map(|entity| {
        //         BlockDescriptor {
        //             name: "minecraft:blaze_spawner".to_string(),
        //             x: entity.x,
        //             y: entity.y,
        //             z: entity.z,
        //             enity: Some(entity),
        //         }
        //     })
        //     .collect_vec();

        // if blaze_spawners.len() > 0 {
        //     println!("blaze_spawners: {:?}", block_entities);
        // }

        // result.found.extend(blaze_spawners);

        let chunk = chunk.and_then(|data| match JavaChunk::from_bytes(&data) {
            Ok(JavaChunk::Post18(chunk)) => Some(chunk),
            _ => None,
        });

        let tower = chunk.and_then(|chunk| chunk.sections);
        if tower.is_none() {
            continue;
        }

        let mut has_non_air = false;

        let tower = tower.unwrap();
        for section in tower.sections() {
            let palette = section.block_states.palette();
            let is_air = palette.len() == 1 && palette.get(0).unwrap().name() == "minecraft:air";
            let iter = section.block_states.try_iter_indices();
            let sy = (section.y() as i64) * 16;
            has_non_air = has_non_air || !is_air;

            if iter.is_none() {
                continue;
            }

            // Skip air sections completely
            if is_air {
                continue;
            }

            result.sections += 1;

            for (i, block_index) in iter.unwrap().enumerate() {
                let block = palette.get(block_index).unwrap();

                // Skip air blocks
                if block.name() == "minecraft:air" {
                    continue;
                }

                result.blocks += 1;

                if !name.matches(block.name()) {
                    continue;
                }

                let x = (i & 0x000F) as i64;
                let y = ((i & 0x0F00) >> 8) as i64;
                let z = ((i & 0x00F0) >> 4) as i64;

                // 512 blocks per region + 16 blocks per chunk + x
                // Normalized

                result.found.push(BlockDescriptor {
                    name: normalize_name(block.name()),
                    x: rx * 512 + cx * 16 + x,
                    y: sy + y,
                    z: rz * 512 + cz * 16 + z,
                    enity: None,
                });
            }
        }

        if has_non_air {
            result.chunks += 1;
        }
    }

    result
}

fn normalize_name(name: &str) -> String {
    name
        .trim_end_matches("_stone")
        .trim_end_matches("_kivi")
        .trim_end_matches("_deepslate")
        .replace("deepslate_", "")
        .replace("aethersteel:aetherslate_", "minecraft:")
}
