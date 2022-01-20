use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long, default_value = "10000")]
    pub amount: usize,

    #[clap(short, long, default_value = "images")]
    pub path: PathBuf,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}
