use std::fmt::Display;

use clap::{ArgEnum, Parser};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct NewCommand {
    /// The mode for processing attribute rarity
    #[clap(short, long, arg_enum, default_value = "simple")]
    pub mode: Mode,
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    /// Path to the projects config file
    #[clap(short, long, default_value = "pix.json")]
    pub config: String,
}

/// A CLI for managing NFT projects
#[derive(Parser, Debug)]
pub enum Commands {
    /// Provide your NFT Maker API Key to use globally
    Auth,
    /// Clean the output directory
    Clean,
    /// Generate an NFT collection
    Gen(ConfigArgs),
    /// Output metadata template that can be uploaded to nft-maker.io
    Metadata(ConfigArgs),
    /// Create a new project
    New { name: String },
    /// Upload an NFT collection to nft-maker.io
    Upload(ConfigArgs),
}

impl Default for Commands {
    fn default() -> Self {
        Self::new()
    }
}

impl Commands {
    pub fn new() -> Self {
        Commands::parse()
    }
}

#[derive(Clone, Copy, Debug, ArgEnum, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Simple,
    Advanced,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Simple
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Mode::*;

        match self {
            Simple => write!(f, "simple"),
            Advanced => write!(f, "advanced"),
        }
    }
}
