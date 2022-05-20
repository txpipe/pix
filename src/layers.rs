use std::path::PathBuf;

use anyhow::{anyhow, Context};
use image::{DynamicImage, GenericImageView};
use rand::Rng;

use crate::{cli::Mode, config::LayerConfig};

const RARITIES: [&str; 5] = ["common", "uncommon", "rare", "mythical", "legendary"];

#[derive(Debug, Clone)]
pub struct Trait {
    pub layer: String,
    pub name: String,
    pub weight: u32,
    pub image: Option<DynamicImage>,
}

#[derive(Default)]
pub struct Layers {
    pub data: Vec<Vec<Trait>>,
    pub width: u32,
    pub height: u32,
}

impl Layers {
    pub fn load(
        &mut self,
        mode: Mode,
        layers: Vec<LayerConfig>,
        path: PathBuf,
    ) -> anyhow::Result<()> {
        let mut data = Vec::new();

        let layer_paths = layers
            .iter()
            .map(|layer| (layer, path.join(layer.name.clone())))
            .filter(|(_, path)| path.is_dir());

        for (layer, layer_path) in layer_paths {
            let mut trait_list = Vec::new();

            match mode {
                Mode::Advanced => {
                    let trait_paths = layer_path
                        .read_dir()
                        .with_context(|| format!("{} is not a folder", layer_path.display()))?
                        .map(|dir| dir.unwrap().path())
                        .filter(|path| path.is_file())
                        .filter(|path| matches!(path.extension(), Some(ext) if ext == "png"));

                    for trait_path in trait_paths {
                        let image = image::open(&trait_path).with_context(|| {
                            format!("failed to load image {}", trait_path.display())
                        })?;

                        let (width, height) = image.dimensions();

                        if self.width == 0 && self.height == 0 {
                            self.width = width;
                            self.height = height;
                        }

                        let file_name = trait_path
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();

                        if file_name.contains('#') {
                            let parts: Vec<&str> = file_name.split('#').collect();

                            let name = parts[0];

                            let weight = parts[1].parse().with_context(|| {
                                format!("{} is not a parsable number", parts[1])
                            })?;

                            trait_list.push(Trait {
                                layer: layer.display_name.as_ref().unwrap_or(&layer.name).clone(),
                                name: name.to_owned(),
                                image: Some(image),
                                weight,
                            })
                        } else {
                            return Err(anyhow!("{} is missing `#weight`", file_name));
                        }
                    }
                }
                Mode::Simple => {
                    let rarity_paths = layer_path
                        .read_dir()
                        .with_context(|| format!("{} is not a folder", layer_path.display()))?
                        .map(|dir| dir.unwrap().path())
                        .filter(|path| path.is_dir())
                        .filter(|path| RARITIES.iter().any(|rarity| path.ends_with(rarity)));

                    for rarity_path in rarity_paths {
                        let rarity_name = rarity_path
                            .file_name()
                            .with_context(|| {
                                format!("could not get rarity name for {}", rarity_path.display())
                            })?
                            .to_str();

                        let trait_paths = rarity_path
                            .read_dir()
                            .with_context(|| format!("{} is not a folder", rarity_path.display()))?
                            .map(|dir| dir.unwrap().path())
                            .filter(|path| path.is_file())
                            .filter(|path| matches!(path.extension(), Some(ext) if ext == "png"));

                        for trait_path in trait_paths {
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
                                layer: layer.display_name.as_ref().unwrap_or(&layer.name).clone(),
                                name,
                                image: Some(image),
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

            if let Some(weight) = layer.none {
                trait_list.push(Trait {
                    layer: layer.display_name.as_ref().unwrap_or(&layer.name).clone(),
                    name: "None".to_string(),
                    weight,
                    image: None,
                })
            }

            data.push(trait_list);
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
