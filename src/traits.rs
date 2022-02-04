use std::collections::HashMap;

use anyhow::Context;
use image::{DynamicImage, GenericImageView};
use rand::Rng;

use crate::{cli::Mode, config::AppConfig};

const RARITIES: [&str; 5] = ["common", "uncommon", "rare", "mythical", "legendary"];

#[derive(Debug, Clone)]
pub struct Trait {
    pub layer: String,
    pub name: String,
    pub weight: u32,
    pub image: DynamicImage,
}

#[derive(Default)]
pub struct Layers {
    pub data: Vec<Vec<Trait>>,
    pub width: u32,
    pub height: u32,
}

impl Layers {
    pub fn load(&mut self, config: &AppConfig) -> anyhow::Result<()> {
        let mut layers_map = HashMap::new();

        let feature_paths = config
            .path
            .read_dir()
            .with_context(|| format!("{} is not a folder", config.path.display()))?
            .filter(|x| x.as_ref().unwrap().path().is_dir());

        for feature_dir in feature_paths {
            let feature_dir = feature_dir?;
            let feature_path = feature_dir.path();

            let layer_name = feature_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let mut trait_list = Vec::new();

            match config.mode {
                Mode::Advanced => todo!("implement advanced mode"),
                Mode::Simple => {
                    let rarity_paths = feature_path
                        .read_dir()
                        .with_context(|| format!("{} is not a folder", feature_path.display()))?
                        .filter(|x| x.as_ref().unwrap().path().is_dir())
                        .filter(|x| {
                            RARITIES
                                .iter()
                                .any(|y| x.as_ref().unwrap().path().ends_with(y))
                        });

                    for rarity_dir in rarity_paths {
                        let rarity_dir = rarity_dir?;
                        let rarity_path = rarity_dir.path();
                        let rarity_name = rarity_path
                            .file_name()
                            .with_context(|| {
                                format!("could not get rarity name for {}", rarity_path.display())
                            })?
                            .to_str();

                        let trait_paths = rarity_path
                            .read_dir()
                            .with_context(|| format!("{} is not a folder", rarity_path.display()))?
                            .filter(|x| x.as_ref().unwrap().path().is_file())
                            .filter(|x| x.as_ref().unwrap().path().extension().unwrap() == "png");

                        for trait_file in trait_paths {
                            let trait_file = trait_file?;
                            let trait_path = trait_file.path();

                            let image = image::open(&trait_path).with_context(|| {
                                format!("failed to load image {}", trait_path.display())
                            })?;

                            let (width, height) = image.dimensions();

                            if self.width == 0 && self.height == 0 {
                                self.width = width;
                                self.height = height;
                            }

                            let name = trait_path
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string();

                            trait_list.push(Trait {
                                layer: layer_name.clone(),
                                name,
                                image,
                                weight: match rarity_name {
                                    Some("common") => 70,
                                    Some("uncommon") => 50,
                                    Some("rare") => 20,
                                    Some("mythical") => 10,
                                    Some("legendary") => 5,
                                    _ => unreachable!(),
                                },
                            })
                        }
                    }
                }
            }

            layers_map.insert(feature_path, trait_list);
        }

        let mut data = Vec::new();

        for item in &config.attributes {
            let trait_list = layers_map.get(&config.path.join(item)).with_context(|| {
                format!("{} folder not found in {}", item, config.path.display())
            })?;

            data.push(trait_list.clone());
        }

        self.data = data;

        Ok(())
    }

    pub fn create_unique(&self) -> Vec<usize> {
        let mut random = Vec::new();

        let mut rng = rand::thread_rng();

        for trait_list in &self.data {
            let total_weight = trait_list.iter().fold(0, |acc, elem| acc + elem.weight);

            let random_num = rng.gen_range(0.0..1.0);

            let mut n = (random_num * total_weight as f64).floor();

            for (index, elem) in trait_list.iter().enumerate() {
                n -= elem.weight as f64;

                if n < 0.0 {
                    random.push(index);

                    break;
                }
            }
        }

        random
    }
}
