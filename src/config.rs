use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context};
use config::{Config, File};
use dialoguer::{console::Term, theme::ColorfulTheme, Confirm, Input, Select};
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::cli::Mode;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AppConfig {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    pub mode: Mode,
    pub amount: usize,
    pub tolerance: usize,
    pub path: PathBuf,
    pub attributes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft_maker: Option<NftMakerLocalConfig>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct NftMakerLocalConfig {
    #[serde(skip_serializing_if = "String::is_empty")]
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

        s.merge(File::with_name(global_file_name).required(false))?;

        s.merge(File::with_name(file_name).required(true))?;

        s.try_into()
            .map_err(|e| anyhow!("loading config\nReason: {}", e.to_string()))
    }

    pub fn prompt() -> anyhow::Result<Self> {
        let name: String = Input::new()
            .with_prompt("enter asset name")
            .interact_text()?;

        let display_name: String = Input::new()
            .with_prompt("enter display name")
            .allow_empty(true)
            .interact_text()?;

        let display_name = if !display_name.is_empty() {
            Some(display_name)
        } else {
            None
        };

        let twitter: String = Input::new()
            .with_prompt("enter twitter url")
            .allow_empty(true)
            .interact_text()?;

        let twitter = if !twitter.is_empty() {
            Some(twitter)
        } else {
            None
        };

        let website: String = Input::new()
            .with_prompt("enter website url")
            .allow_empty(true)
            .interact_text()?;

        let website = if !website.is_empty() {
            Some(website)
        } else {
            None
        };

        let copyright: String = Input::new()
            .with_prompt("enter copyright")
            .allow_empty(true)
            .interact_text()?;

        let copyright = if !copyright.is_empty() {
            Some(copyright)
        } else {
            None
        };

        let items = vec![Mode::Simple, Mode::Advanced];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        let mode = selection.map_or(Mode::Simple, |index| items[index]);

        let amount: usize = Input::new().with_prompt("enter amount").interact_text()?;

        let mut attributes = Vec::new();

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("would you like to enter layers?")
            .default(true)
            .interact()?
        {
            loop {
                let attribute: String = Input::new().allow_empty(true).interact_text()?;

                if attribute.is_empty() {
                    break;
                }

                attributes.push(attribute);
            }
        }

        Ok(Self {
            name,
            display_name,
            twitter,
            website,
            copyright,
            mode,
            amount,
            tolerance: 50,
            path: "images".into(),
            attributes,
            nft_maker: None,
        })
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

pub fn create_global_config_paths() -> anyhow::Result<(PathBuf, PathBuf)> {
    let (global_config_dir, global_config_file) = get_global_config_paths()?;

    if !global_config_dir.exists() {
        fs::create_dir_all(&global_config_dir)?;
    }

    if !global_config_file.exists() {
        fs::write(&global_config_file, b"{}")?;
    }

    Ok((global_config_dir, global_config_file))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NftMakerGlobalConfig {
    pub apikey: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GlobalConfig {
    pub nft_maker: Option<NftMakerGlobalConfig>,
}

impl GlobalConfig {
    pub fn new() -> anyhow::Result<Self> {
        let mut s = Config::default();

        let (_, global_path) = get_global_config_paths()?;

        let global_file_name = global_path
            .to_str()
            .context("failed to load global config")?;

        s.merge(File::with_name(global_file_name).required(false))?;

        s.try_into()
            .map_err(|e| anyhow!("loading config\nReason: {}", e.to_string()))
    }
}
