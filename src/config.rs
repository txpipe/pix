use std::{fs::File, path::PathBuf};

use anyhow::Context;
use serde::Deserialize;

use crate::cli::Mode;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mode: Mode,
    pub amount: usize,
    pub tolerance: usize,
    pub path: PathBuf,
    pub order: Vec<String>,
}

impl Config {
    pub fn new(file_name: &str) -> anyhow::Result<Self> {
        let config_file = File::open(file_name).context("opening config file")?;

        let config: Self = serde_json::from_reader(config_file).context("loading config file")?;

        Ok(config)
    }
}
