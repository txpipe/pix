use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    path::Path,
    process,
};

use anyhow::{anyhow, Context};
use directories_next::ProjectDirs;
use image::RgbaImage;
use indicatif::ProgressBar;
use rayon::prelude::*;

use nft_gen::{
    cli::Commands,
    config::{self, GenConfig, GlobalConfig},
    traits::{self, Features},
    utils,
};

const OUTPUT: &str = "output";

fn main() -> anyhow::Result<()> {
    let cmds = Commands::new();

    let output = Path::new(OUTPUT);

    match cmds {
        Commands::Clean => utils::clean(output)?,

        Commands::Gen(args) => {
            let config = GenConfig::new(&args.config)?;
            let progress = ProgressBar::new(config.amount as u64);

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
                    .join(":");

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

                uniques.insert(unique_str);

                count += 1;
            }

            uniques.par_iter().for_each(|unique_str| {
                let mut base = RgbaImage::new(initial_width, initial_height);

                let unique = unique_str
                    .split(':')
                    .map(|index| index.parse::<usize>().unwrap());

                for (index, trait_list) in unique.zip(&layers) {
                    utils::merge(&mut base, &trait_list[index].image);
                }

                let output_file = format!("{}/{}.png", OUTPUT, unique_str);

                let result = base
                    .save(&output_file)
                    .with_context(|| format!("failed to generate {}", output_file));

                match result {
                    Ok(()) => {
                        progress.inc(1);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
            });

            progress.finish();
        }

        Commands::Init => {
            let (global_config_dir, global_config_file) = config::get_global_config_paths()?;

            if !global_config_dir.exists() {
                fs::create_dir_all(global_config_dir)?;
            }

            if !global_config_file.exists() {
                let mut file = File::create(global_config_file)?;

                file.write_all(b"{}")?;
            } else {
                println!("Already initialized")
            }
        }

        Commands::New(_args) => {
            if let Some(project_dirs) = ProjectDirs::from("com", "3Based", "NFTGen") {
                dbg!(project_dirs.config_dir());
                // Lin: Some(/home/alice/.local/bin)
                // Win: None
                // Mac: None
            }
        }

        Commands::Upload(args) => {
            if !output.exists() {
                return Err(anyhow!("no output found, try running gen first"));
            }

            let config = GlobalConfig::new(&args.config)?;

            println!("{:?}", config);
        }
    }

    Ok(())
}
