use rayon::prelude::*;
use std::{path::PathBuf, fs::File, collections::HashSet};
use fastanvil::{RegionFileLoader, RegionLoader, LoaderError, Chunk, RCoord};

use crate::region::ChunkRegion;

pub enum DimensionGroup {
    Vanilla,
    Modded{ group: String },
}

pub struct Dimension {
    pub group: DimensionGroup,
    pub name: String,
    pub path: PathBuf,

    pub regions: Vec<ChunkRegion>,
}

#[derive(Debug, Clone)]
pub struct DimensionLoadError(pub String);

impl From<LoaderError> for DimensionLoadError {
    fn from(err: LoaderError) -> Self {
        DimensionLoadError(err.to_string())
    }
}

impl Dimension {
    pub fn vanilla(path: PathBuf, name: &str) -> Dimension {
        Dimension {
            group: DimensionGroup::Vanilla,
            name: name.to_string(),
            path,
            regions: vec![],
        }
    }

    pub fn modded(path: PathBuf, mod_name: &str, name: &str) -> Dimension {
        Dimension {
            group: DimensionGroup::Modded { group: mod_name.to_string() },
            name: name.to_string(),
            path,
            regions: vec![],
        }
    }

    pub fn load_regions(&mut self) -> Result<(), DimensionLoadError> {
        if self.regions.len() == 0 {
            self.reload_regions()?;
        }

        Ok(())
    }

    pub fn reload_regions(&mut self) -> Result<(), DimensionLoadError> {
        let loader = RegionFileLoader::new(self.path.join("region"));
        let regions = loader.list()?;

        let regions = regions
            .into_iter()
            .map(|(rx, rz)| ChunkRegion::new(rx.0 as i32, rz.0 as i32))
            .par_bridge()
            .map(|mut region| {
                region.load_chunks(&loader)?;

                Ok(region)
            })
            .collect::<Vec<Result<ChunkRegion, DimensionLoadError>>>();

        // Throw the first error if there is one
        for region in &regions {
            match region {
                Err(err) => return Err(err.clone()),
                _ => {},
            }
        }

        // Unwrap the results and store them
        self.regions = regions.into_iter().map(|r| r.unwrap()).collect();

        Ok(())
    }

    pub fn block_names(&mut self) -> Result<impl Iterator<Item = String>, DimensionLoadError> {
        self.load_regions()?;

        let name_list = self.regions
            .iter()
            .flat_map(|region| region.names())

            // Unique them across all regions
            .collect::<HashSet<String>>()
            .into_iter();

        Ok(name_list)
    }
}
