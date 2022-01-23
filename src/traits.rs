use std::{collections::HashMap, path::PathBuf};

use image::DynamicImage;

pub type Traits = HashMap<PathBuf, SimpleTrait>;

#[derive(Debug)]
pub struct SimpleTrait {
    pub common: Vec<DynamicImage>,
    pub uncommon: Vec<DynamicImage>,
    pub rare: Vec<DynamicImage>,
    pub mythical: Vec<DynamicImage>,
    pub legendary: Vec<DynamicImage>,
}

impl SimpleTrait {
    pub fn new() -> Self {
        SimpleTrait {
            common: Vec::new(),
            uncommon: Vec::new(),
            rare: Vec::new(),
            mythical: Vec::new(),
            legendary: Vec::new(),
        }
    }
}
