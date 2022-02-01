use std::{collections::HashSet, fs, path::Path, process, time::Duration};

use anyhow::{anyhow, Context};
use directories_next::ProjectDirs;
use image::RgbaImage;
use indicatif::ProgressBar;
use rayon::prelude::*;

use nft_gen::{
    cli::Commands,
    config::AppConfig,
    nft_maker::{NftFile, NftMakerClient, UploadNftRequest},
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
            let config = AppConfig::new(&args.config)?;
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

            let config = AppConfig::new(&args.config)?;

            if let Some(nft_maker_config) = config.nft_maker {
                let (tx, rx) = std::sync::mpsc::sync_channel(config.amount);

                let nft_maker = NftMakerClient::new(nft_maker_config.apikey);

                let output_dir = output
                    .read_dir()
                    .with_context(|| format!("{} is not a folder", output.display()))?;

                // let total = output_dir.count();

                let uploader = std::thread::spawn(move || {
                    for (index, nft_file) in output_dir.enumerate() {
                        let nft = image::open(nft_file.unwrap().path())
                            .expect("failed to load nft image");

                        let nft_base64 = base64::encode(nft.to_bytes());

                        let body = UploadNftRequest {
                            asset_name: Some(String::from("BasedBears")),
                            preview_image_nft: NftFile {
                                mimetype: Some(String::from("image/png")),
                                description: None,
                                displayname: Some(format!("BasedBear#{}", index)),
                                file_from_IPFS: None,
                                file_froms_url: None,
                                file_from_base64: Some(nft_base64),
                                metadata_placeholder: vec![],
                            },
                            subfiles: vec![],
                            metadata: None,
                        };

                        let _data = nft_maker
                            .upload_nft(nft_maker_config.nft_project_id, &body)
                            .expect("failed to upload nft");

                        tx.send(index).expect("failed to send progress");

                        std::thread::sleep(Duration::from_millis(10));
                    }
                });

                let progress_bar = std::thread::spawn(move || {
                    let progress = ProgressBar::new(config.amount as u64);

                    loop {
                        let index = rx.recv().expect("unable to receive index");

                        if index < config.amount {
                            progress.inc(1);
                        }

                        if index == config.amount - 1 {
                            break;
                        }
                    }

                    progress.finish();
                });

                uploader
                    .join()
                    .map_err(|_| anyhow!("error uploading nfts"))?;

                progress_bar
                    .join()
                    .map_err(|_| anyhow!("error with progress bar"))?;
            } else {
                eprintln!("Error: please provide an nft_maker config to upload");

                process::exit(1);
            }
        }
    }

    Ok(())
}
