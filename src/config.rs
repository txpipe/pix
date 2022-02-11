use std::io::Write;
use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context};
use config::{Config, File};
use directories_next::ProjectDirs;
use serde::Deserialize;

use crate::cli::Mode;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub name: String,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub copyright: Option<String>,
    pub mode: Mode,
    pub amount: usize,
    pub tolerance: usize,
    pub path: PathBuf,
    pub attributes: Vec<String>,
    pub nft_maker: Option<NftMakerLocalConfig>,
}

#[derive(Deserialize, Debug)]
pub struct NftMakerLocalConfig {
    pub apikey: String,
    pub nft_project_id: i32,
}

impl AppConfig {
    pub fn new(file_name: &str) -> anyhow::Result<Self> {
        let mut s = Config::default();

        let (_, global_path) = get_global_config_paths()?;

        let global_file_name = global_path
            .to_str()
            .context("failed to load global config")?;

        s.merge(File::with_name(global_file_name).required(true))?;

        s.merge(File::with_name(file_name).required(true))?;

        s.try_into()
            .map_err(|e| anyhow!("loading config\nReason: {}", e.to_string()))
    }
}

pub fn get_global_config_paths() -> anyhow::Result<(PathBuf, PathBuf)> {
    let project = ProjectDirs::from("rs", "", "pix").context("getting global config folder")?;

    let config_dir = project.config_dir().to_path_buf();
    let config_dir_str = config_dir
        .to_str()
        .context("getting global config file path")?;

    let path = [config_dir_str, "global.json"].iter().collect();

    Ok((config_dir, path))
}

pub fn create_global_config_paths() -> anyhow::Result<()> {
    let (global_config_dir, global_config_file) = get_global_config_paths()?;

    if !global_config_dir.exists() {
        fs::create_dir_all(global_config_dir)?;
    }

    if !global_config_file.exists() {
        let mut file = fs::File::create(global_config_file)?;

        file.write_all(b"{}")?;
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct NftMakerGlobalConfig {
    pub apikey: String,
}

#[derive(Deserialize, Debug)]
pub struct GlobalConfig {
    pub nft_maker: NftMakerGlobalConfig,
}
