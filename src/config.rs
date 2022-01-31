use std::path::PathBuf;

use anyhow::{anyhow, Context};
use config::{Config, File};
use directories_next::ProjectDirs;
use serde::Deserialize;

use crate::cli::Mode;

#[derive(Deserialize, Debug)]
pub struct GenConfig {
    pub mode: Mode,
    pub amount: usize,
    pub tolerance: usize,
    pub path: PathBuf,
    pub order: Vec<String>,
}

impl GenConfig {
    pub fn new(file_name: &str) -> anyhow::Result<Self> {
        let mut s = Config::default();

        s.merge(File::with_name(file_name).required(true))?;

        s.try_into()
            .map_err(|_| anyhow!("failed to load local config"))
    }
}

#[derive(Deserialize, Debug)]
pub struct NftMakerConfig {
    pub apikey: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalConfig {
    pub nft_maker: NftMakerConfig,
}

impl GlobalConfig {
    pub fn new(file_name: &str) -> anyhow::Result<Self> {
        let mut s = Config::default();

        let (_, global_path) = get_global_config_paths()?;

        if !global_path.exists() {
            return Err(anyhow!("make sure to run init"));
        }

        let global_file_name = global_path
            .to_str()
            .context("failed to load global config")?;

        s.merge(File::with_name(global_file_name).required(true))?;

        s.merge(File::with_name(file_name).required(false))?;

        s.try_into()
            .map_err(|_| anyhow!("failed to load global config"))
    }
}

pub fn get_global_config_paths() -> anyhow::Result<(PathBuf, PathBuf)> {
    let project = ProjectDirs::from("rs", "", "nft-gen").context("getting global config folder")?;

    let config_dir = project.config_dir().to_path_buf();
    let config_dir_str = config_dir
        .to_str()
        .context("getting global config file path")?;

    let path = [config_dir_str, "global.json"].iter().collect();

    Ok((config_dir, path))
}
