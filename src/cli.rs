use clap::{ArgEnum, Parser};
use serde::Deserialize;

#[derive(Parser, Debug)]
pub struct NewCommand {
    #[clap(short, long, arg_enum, default_value = "simple")]
    pub mode: Mode,
}

#[derive(Parser, Debug)]
pub struct GenCommand {
    #[clap(short, long, default_value = "nft-gen.json")]
    pub config: String,
}

#[derive(Parser, Debug)]
pub enum Commands {
    New(NewCommand),
    Gen(GenCommand),
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
