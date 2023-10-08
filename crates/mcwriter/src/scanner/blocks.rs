use serde::Deserialize;

use super::points::Point;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BlockDescriptor {
    pub name: String,
    pub x: i64,
    pub y: i64,
    pub z: i64,

    pub enity: Option<BlockEntity>,
}

impl BlockDescriptor {
    pub fn pos(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Ord for BlockDescriptor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.x < other.x {
            std::cmp::Ordering::Less
        } else if self.x > other.x {
            std::cmp::Ordering::Greater
        } else if self.y < other.y {
            std::cmp::Ordering::Less
        } else if self.y > other.y {
            std::cmp::Ordering::Greater
        } else if self.z < other.z {
            std::cmp::Ordering::Less
        } else if self.z > other.z {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for BlockDescriptor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmp(other).into()
    }
}

/// A Minecraft chunk.
#[derive(Deserialize, Debug)]
pub struct ChunkEntityContainer {
    #[serde(rename = "block_entities")]
    pub block_entities: Vec<BlockEntity>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct BlockEntity {
    pub id: String,
    pub x: i64,
    pub y: i64,
    pub z: i64,

    #[serde(rename = "MinSpawnDelay")]
    pub min_spawn_delay: Option<isize>,

    #[serde(rename = "MaxSpawnDelay")]
    pub max_spawn_delay: Option<isize>,

    #[serde(rename = "SpawnCount")]
    pub spawn_count: Option<isize>,

    #[serde(rename = "SpawnRange")]
    pub spawn_range: Option<isize>,

    #[serde(rename = "MaxNearbyEntities")]
    pub max_entities: Option<isize>,

    #[serde(rename = "RequiredPlayerRange")]
    pub activation_range: Option<isize>,

    #[serde(rename = "SpawnData")]
    pub spawn_data: Option<SpawnData>,
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
