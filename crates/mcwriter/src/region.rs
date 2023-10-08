use std::fs::File;

use itertools::Itertools;
use fastanvil::{RegionLoader, CurrentJavaChunk, RCoord, Region, JavaChunk, RegionFileLoader};

use crate::dimension::DimensionLoadError;

pub struct ChunkRegion {
    pub x: i32,
    pub z: i32,
    pub chunks: Vec<CurrentJavaChunk>,
}

impl ChunkRegion {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
            chunks: vec![],
        }
    }

    pub fn names(&self) -> Vec<String> {
        return self.chunks.iter().flat_map(|chunk| names_in_chunk(chunk)).collect_vec();
    }

    pub fn load_chunks(&mut self, loader: &RegionFileLoader) -> Result<(), DimensionLoadError> {
        // Load file
        let anvil_region = loader.region(RCoord(self.x as isize), RCoord(self.z as isize));

        if anvil_region.is_none() {
            return Err(DimensionLoadError("Failed load region file".to_string()))
        }

        let mut anvil_region = anvil_region.unwrap();

        // Load chunks from the file
        self.load_chunks_from(&mut anvil_region);

        Ok(())
    }

    fn load_chunks_from(&mut self, region: &mut Region<File>) -> Vec<CurrentJavaChunk> {
        // Generate all possible chunk coordinates in a region
        // Z -> X because of the layout of the file results in sequential reads
        let coords = (0..32).flat_map(|z| (0..32).map(move |x| (x, z)));

        coords
            .into_iter()

            // Raad valid chunks from the file
            .map(|(x, z)| region.read_chunk(x, z))
            .flat_map(|res| res)
            .flat_map(|opt| opt)

            // Parse them (1.18+ only)
            .map(|data| JavaChunk::from_bytes(&data))
            .flat_map(|res| res)
            .flat_map(|chunk| match chunk {
                JavaChunk::Post18(raw) => Some(raw),
                _ => None,
            })
            .collect()
    }
}

fn names_in_chunk(chunk: &CurrentJavaChunk) -> impl Iterator<Item = String> + '_ {
    return chunk.sections.iter().flat_map(|tower| {
        tower.sections()
            .iter()
            .flat_map(|section| section.block_states.palette().iter())
            .map(|block| block.name().to_string())
            .collect_vec()
    })
}
