use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub amount: usize,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}
