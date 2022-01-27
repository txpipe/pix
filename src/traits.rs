use std::{collections::HashMap, path::PathBuf};

use image::DynamicImage;

pub type FeaturesMap = HashMap<PathBuf, Vec<Trait>>;

#[derive(Debug)]
pub struct Trait {
    pub weight: u32,
    pub image: DynamicImage,
}
