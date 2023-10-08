use std::{path::PathBuf, str::FromStr, fs::File, collections::HashSet};
use itertools::Itertools;

use fastanvil::*;
use rayon::prelude::{ParallelIterator, IntoParallelIterator};

fn main() {
    let path = PathBuf::from_str("/Users/jordanpittman/Developer/projects/mcwriter/fixtures/the-other").unwrap();

    let names = names_in_dimension(path)
        .sorted()
        .join("\n");

    println!("{}", names);
}

fn names_in_dimension(path: PathBuf) -> impl Iterator<Item = String> {
    let loader = RegionFileLoader::new(path.join("region"));
    let regions = loader.list().unwrap();

    return regions
        .into_par_iter()

        // Load up every region in parallel
        .map(|(rx, rz)| loader.region(rx, rz).unwrap())

        // Collect the block types in it
        .flat_map_iter(|region| names_in_region(region))

        // Unique them across all regions
        .collect::<HashSet<String>>()

        .into_iter();
}

fn names_in_region(region: Region<File>) -> impl Iterator<Item = String> {
    let mut region = region;

    // Generate all possible chunk coordinates in a region
    // Z -> X because of the layout of the file results in sequential reads
    let coords = (0..32).flat_map(|z| (0..32).map(move |x| (x, z)));

    coords
        .into_iter()

        // Raad valid chunks from the file
        .map(move |(x, z)| region.read_chunk(x, z))
        .flat_map(|res| res)
        .flat_map(|opt| opt)

        // Parse them
        .map(|data| JavaChunk::from_bytes(&data))
        .flat_map(|res| res)
        .flat_map(|chunk| match chunk {
            JavaChunk::Post18(raw) => Some(raw),
            _ => None,
        })

        // Discover contained block types
        .flat_map(|chunk| names_in_chunk(chunk))
}

fn names_in_chunk(chunk: CurrentJavaChunk) -> impl Iterator<Item = String> {
    chunk.sections.into_iter().flat_map(|tower| {
        tower.sections()
            .into_iter()
            .flat_map(|section| section.block_states.palette().iter())
            .map(|block| block.name().to_string())
            .collect_vec()
    })
}
