use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    process,
    time::Duration,
};

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

            if !output.exists() {
                fs::create_dir(output).context("creating output directory")?;
            }

            uniques
                .into_iter()
                .enumerate()
                .collect::<Vec<(usize, String)>>()
                .par_iter()
                .for_each(|(count, unique_str)| {
                    let mut base = RgbaImage::new(initial_width, initial_height);

                    let unique = unique_str
                        .split(':')
                        .map(|index| index.parse::<usize>().unwrap());

                    let mut trait_info = HashMap::new();

                    let folder_name = output.join(format!("{}#{}", config.name, count));
                    fs::create_dir(&folder_name).expect("failed to created a folder for an NFT");

                    for (index, trait_list) in unique.zip(&layers) {
                        let nft_trait = &trait_list[index];

                        trait_info.insert(&nft_trait.layer, &nft_trait.name);

                        utils::merge(&mut base, &nft_trait.image);
                    }

                    let nft_image_path = folder_name.join(format!("{}#{}.png", config.name, count));
                    let metadata_path = folder_name.join(format!("{}#{}.json", config.name, count));

                    base.save(nft_image_path).expect("failed to create image");

                    let metadata =
                        serde_json::to_string(&trait_info).expect("failed to create metadata");

                    fs::write(metadata_path, metadata).expect("failed to create metadata");

                    progress.inc(1);
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
                let nft_maker = NftMakerClient::new(nft_maker_config.apikey);

                let output_dir = output
                    .read_dir()
                    .with_context(|| format!("{} is not a folder", output.display()))?;

                let progress = ProgressBar::new(config.amount as u64);

                for (index, nft_file) in output_dir.enumerate() {
                    let nft =
                        image::open(nft_file.unwrap().path()).expect("failed to load nft image");

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

                    progress.inc(1);

                    std::thread::sleep(Duration::from_millis(10));
                }

                progress.finish();
            } else {
                eprintln!("Error: please provide an nft_maker config to upload");

                process::exit(1);
            }
        }
    }

    Ok(())
}
