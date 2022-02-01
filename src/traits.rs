use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use image::{DynamicImage, GenericImageView};
use rand::Rng;

use crate::{cli::Mode, config::AppConfig};

const RARITIES: [&str; 5] = ["common", "uncommon", "rare", "mythical", "legendary"];

pub struct Features {
    layers_map: HashMap<PathBuf, Vec<Trait>>,
    pub initial_width: u32,
    pub initial_height: u32,
}

impl Features {
    pub fn load_features(config: &AppConfig) -> anyhow::Result<Self> {
        let mut layers_map = HashMap::new();

        let (mut initial_width, mut initial_height) = (0, 0);

        let feature_paths = config
            .path
            .read_dir()
            .with_context(|| format!("{} is not a folder", config.path.display()))?
            .filter(|x| x.as_ref().unwrap().path().is_dir());

        for feature_dir in feature_paths {
            let feature_dir = feature_dir?;
            let feature_path = feature_dir.path();

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

                            if initial_width == 0 && initial_height == 0 {
                                initial_width = width;
                                initial_height = height;
                            }

                            trait_list.push(Trait {
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

        Ok(Self {
            layers_map,
            initial_width,
            initial_height,
        })
    }

    pub fn layers<'a>(&'a self, config: &AppConfig) -> anyhow::Result<Layers<'a>> {
        let mut layers = Vec::new();

        for item in &config.order {
            let trait_list = self
                .layers_map
                .get(&config.path.join(item))
                .with_context(|| {
                    format!("{} folder not found in {}", item, config.path.display())
                })?;

            layers.push(trait_list);
        }

        Ok(layers)
    }
}

pub type Layers<'a> = Vec<&'a Vec<Trait>>;

pub fn create_unique(features: &Layers) -> Vec<usize> {
    let mut random = Vec::new();

    let mut rng = rand::thread_rng();

    for trait_list in features {
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

#[derive(Debug)]
pub struct Trait {
    pub weight: u32,
    pub image: DynamicImage,
}
