use clap::{ArgEnum, Parser};
use serde::Deserialize;

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
    New(ConfigArgs),
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

#[derive(Clone, Debug, ArgEnum, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Simple,
    Advanced,
}
