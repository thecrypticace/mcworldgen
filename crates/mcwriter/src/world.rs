use std::path::PathBuf;
use walkdir::WalkDir;

use crate::dimension::Dimension;

pub struct World {
    pub path: PathBuf,
    pub dimensions: Vec<Dimension>,
}

impl World {
    pub fn load(path: PathBuf) -> World {
        World {
            path: path.clone(),
            dimensions: Self::load_dimensions(path.clone()),
        }
    }

    fn load_dimensions(path: PathBuf) -> Vec<Dimension> {
        let mut dimensions = vec![];

        dimensions.append(&mut Self::load_dimensions_vanilla(path.clone()));
        dimensions.append(&mut Self::load_dimensions_modded(path.clone()));

        return dimensions;
    }

    fn load_dimensions_vanilla(path: PathBuf) -> Vec<Dimension> {
        return vec![
            // Dimension::vanilla(path.clone(), "Overworld"),
            // Dimension::vanilla(path.clone().join("DIM-1"), "The Nether"),
            Dimension::vanilla(path.clone().join("DIM1"), "The End"),
        ]
    }

    fn load_dimensions_modded(path: PathBuf) -> Vec<Dimension> {
        let mut dimensions = vec![];

        // Load modded dimensions from world/dimensions/*/*
        let walker = WalkDir::new(path.join("dimensions"))
            .max_depth(2)
            .into_iter()
            .filter_entry(|e| {
                return e.file_type().is_dir()
                    && !e.file_name().to_str().map_or(false, |s| s.starts_with("."))
            });

        for entry in walker {
            let entry = entry.unwrap();
            let path = entry.path();

            let mod_name = path.parent().unwrap().file_name().unwrap().to_str().unwrap();
            let dim_name = path.file_name().unwrap().to_str().unwrap();

            dimensions.push(Dimension::modded(path.to_path_buf(), mod_name, dim_name));
        }

        dimensions
    }
}
