use std::collections::HashMap;

use itertools::Itertools;
use partitions::PartitionVec;

use crate::scanner::bounds::Bounds;

use super::{blocks::BlockDescriptor, points::Point};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Vein {
    pub center: Point,
    pub bounds: Bounds,
    pub blocks: Vec<BlockDescriptor>,
}

impl Vein {
    pub fn from_iter(blocks: impl Iterator<Item = BlockDescriptor>) -> Self {
        Self {
            center: Point::ZERO,
            bounds: Bounds::ZERO,
            blocks: blocks.collect_vec(),
        }
    }

    pub fn with_computed_bounds(mut self) -> Self {
        self.bounds = self.compute_bounds();
        self.center = self.bounds.center();
        self
    }

    fn compute_bounds(&self) -> Bounds {
        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;
        let mut min_z = i64::MAX;
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;
        let mut max_z = i64::MIN;

        for block in &self.blocks {
            if block.x < min_x {
                min_x = block.x;
            }
            if block.y < min_y {
                min_y = block.y;
            }
            if block.z < min_z {
                min_z = block.z;
            }
            if block.x > max_x {
                max_x = block.x;
            }
            if block.y > max_y {
                max_y = block.y;
            }
            if block.z > max_z {
                max_z = block.z;
            }
        }

        Bounds {
            min: Point {
                x: min_x,
                y: min_y,
                z: min_z,
            },
            max: Point {
                x: max_x,
                y: max_y,
                z: max_z,
            },
        }
    }
}

impl Ord for Vein {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.blocks.len().cmp(&other.blocks.len())
    }
}

impl PartialOrd for Vein {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.cmp(other).into();
    }
}

pub fn find_veins(blocks: Vec<BlockDescriptor>) -> impl Iterator<Item = Vein> {
    let mut blocks = PartitionVec::from_iter(blocks.into_iter());

    let mut idx_to_pos = HashMap::new();
    let mut pos_to_idx = HashMap::new();
    let mut pos_to_name = HashMap::new();

    for (idx, block) in blocks.iter().enumerate() {
        idx_to_pos.insert(idx, block.pos());
        pos_to_idx.insert(block.pos(), idx);
        pos_to_name.insert(block.pos(), block.name.clone());
    }

    // Label every block using a union-find structure
    let mut pairs = vec![];

    for block in blocks.iter() {
        let first_index = pos_to_idx.get(&block.pos()).unwrap();

        let neighbors = block
            .pos()
            .neighbors()
            .flat_map(|pos| pos_to_name.get(&pos).map(|name| (pos, name)))
            .filter(|(_, name)| **name == block.name)
            .map(|(pos, _)| pos_to_idx.get(&pos).unwrap())
            .collect_vec();

        for second_index in neighbors {
            pairs.push((*first_index, *second_index));
        }
    }

    for (first_index, second_index) in pairs {
        blocks.union(first_index, second_index);
    }

    blocks
        .all_sets()
        .map(|vein_blocks| {
            Vein::from_iter(vein_blocks.into_iter().map(|(_, b)| b.clone())).with_computed_bounds()
        })
        .filter(|vein| vein.blocks.len() > 1)
        .into_iter()
        .collect_vec()
        .into_iter()
}
