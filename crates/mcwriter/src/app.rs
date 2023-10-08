use std::path::PathBuf;

use itertools::Itertools;

use crate::{world::World, dimension::DimensionLoadError};

pub struct App {
    pub world: World,
    pub dim_selected: usize,
}

impl App {
    pub fn new(world_path: PathBuf) -> App {
        App {
            world: World::load(world_path),
            dim_selected: 0,
        }
    }

    pub fn dim_next(&mut self) {
        self.dim_selected = (self.dim_selected + 1) % self.world.dimensions.len();
    }

    pub fn dim_previous(&mut self) {
        if self.dim_selected > 0 {
            self.dim_selected -= 1;
        } else {
            self.dim_selected = self.world.dimensions.len() - 1;
        }
    }

    pub fn dim_load_names(&mut self) -> Result<Vec<String>, DimensionLoadError> {
        // Get the selected dimension
        let dim = &mut self.world.dimensions[self.dim_selected];

        // Load its regions in parallel
        let names = dim.block_names()?;

        Ok(names.collect_vec())
    }
}
