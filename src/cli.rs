use std::path::PathBuf;

use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long, default_value = "10000")]
    pub amount: usize,

    #[clap(short, long, default_value = "images")]
    pub path: PathBuf,

    #[clap(short, long, arg_enum, default_value = "simple")]
    pub mode: Mode,
}

#[derive(Clone, Debug, ArgEnum)]
pub enum Mode {
    Simple,
    Advanced,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}
