use std::{fs::File, path::PathBuf};

use serde::Deserialize;

use crate::cli::Mode;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mode: Mode,
    pub amount: usize,
    pub path: PathBuf,
    pub order: Vec<String>,
}

impl Config {
    pub fn new(file_name: &str) -> Self {
        let config_file = File::open(file_name).expect("error opening config file");

        let config: Self =
            serde_json::from_reader(config_file).expect("failed to load config file");

        config
    }
}
