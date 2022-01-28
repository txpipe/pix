use std::{collections::HashSet, fs, path::Path, process};

use anyhow::Context;
use image::RgbaImage;

use nft_gen::{
    cli::Commands,
    config::Config,
    traits::{self, Features},
    utils,
};

const OUTPUT: &str = "output";

fn main() -> anyhow::Result<()> {
    let cmds = Commands::new();

    let output = Path::new(OUTPUT);

    match cmds {
        Commands::Clean => utils::clean(output)?,
        Commands::New(_args) => {
            todo!("implement new command")
        }
        Commands::Gen(args) => {
            let config = Config::new(&args.config)?;

            let features = Features::load_features(&config)?;

            let Features {
                initial_width,
                initial_height,
                ..
            } = features;

            utils::clean(output)?;

            if !output.exists() {
                fs::create_dir(output).context("creating output directory")?;
            }

            let layers = features.layers(&config)?;

            let mut fail_count = 0;

            let mut uniques = HashSet::new();

            let mut count = 1;

            while count <= config.amount {
                let unique = traits::create_unique(&layers);

                let unique_str = unique
                    .iter()
                    .map(|n| n.to_string()) // map every integer to a string
                    .collect::<Vec<String>>()
                    .join("");

                if uniques.contains(&unique_str) {
                    fail_count += 1;

                    if fail_count > config.tolerance {
                        println!(
                            "You need more features or traits to generate {}",
                            config.amount
                        );

                        process::exit(1);
                    }

                    continue;
                }

                let mut base = RgbaImage::new(initial_width, initial_height);

                for (index, trait_list) in unique.iter().zip(&layers) {
                    utils::merge(&mut base, &trait_list[*index].image);
                }

                let output_file = format!("{}/{}.png", OUTPUT, count);

                base.save(&output_file)
                    .with_context(|| format!("failed to generate {}", output_file))?;

                uniques.insert(unique_str);

                count += 1;
            }
        }
    }

    Ok(())
}
