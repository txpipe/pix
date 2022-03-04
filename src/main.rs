use std::{collections::HashSet, fs, path::Path, process, time::Duration};

use anyhow::{anyhow, Context};
use dialoguer::{theme::ColorfulTheme, Confirm, Password};
use image::RgbaImage;
use indicatif::ProgressBar;
use rayon::prelude::*;
use serde_json::{Map, Value};

use pix::{
    cli::Commands,
    config::{
        create_global_config_paths, AppConfig, GlobalConfig, NftMakerGlobalConfig,
        NftMakerLocalConfig,
    },
    layers::Layers,
    metadata,
    nft_maker::{
        CreateProjectRequest, MetadataPlaceholder, NftFile, NftMakerClient, UploadNftRequest,
    },
    utils,
};

const OUTPUT: &str = "output";

fn main() -> anyhow::Result<()> {
    let cmds = Commands::new();

    let output = Path::new(OUTPUT);

    match cmds {
        Commands::Auth => {
            let (_, global_config_path) = create_global_config_paths()?;

            let apikey = Password::new()
                .with_prompt("NFT Maker API Key")
                .interact()?;

            let global_config = GlobalConfig {
                nft_maker: Some(NftMakerGlobalConfig { apikey }),
            };

            let contents = serde_json::to_string_pretty(&global_config)?;

            fs::write(&global_config_path, contents)?;
        }
        Commands::Clean => utils::clean(output)?,

        Commands::Gen(args) => {
            let config = AppConfig::new(&args.config)?;
            let progress = ProgressBar::new(config.amount as u64);

            let mut layers = Layers::default();

            layers.load(&config)?;

            let mut fail_count = 0;

            let mut uniques = HashSet::new();

            let mut count = 1;

            while count <= config.amount {
                let unique = layers.create_unique();

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

            utils::clean(output)?;

            fs::create_dir(output)?;

            uniques
                .into_iter()
                .enumerate()
                .collect::<Vec<(usize, String)>>()
                .par_iter()
                .for_each(|(mut count, unique_str)| {
                    if config.start_at_one {
                        count += 1
                    }

                    let mut base = RgbaImage::new(layers.width, layers.height);

                    let unique = unique_str
                        .split(':')
                        .map(|index| index.parse::<usize>().unwrap());

                    let mut trait_info = Map::new();

                    let folder_name = output.join(format!("{}#{}", config.name, count));
                    fs::create_dir(&folder_name).expect("failed to created a folder for an NFT");

                    for (index, trait_list) in unique.zip(&layers.data) {
                        let nft_trait = &trait_list[index];

                        trait_info.insert(
                            nft_trait.layer.to_owned(),
                            Value::String(nft_trait.name.to_owned()),
                        );

                        utils::merge(&mut base, &nft_trait.image);
                    }

                    let nft_image_path = folder_name.join(format!("{}#{}.png", config.name, count));
                    let attributes_path =
                        folder_name.join(format!("{}#{}.json", config.name, count));
                    let metadata_path = folder_name.join("metadata.json");

                    base.save(nft_image_path).expect("failed to create image");

                    let attributes = serde_json::to_string_pretty(&trait_info)
                        .expect("failed to create attributes");

                    fs::write(attributes_path, attributes).expect("failed to create attributes");

                    let meta = metadata::build_with_attributes(&config, trait_info, count);

                    fs::write(metadata_path, meta).expect("failed to create metadata");

                    progress.inc(1);
                });

            progress.finish();
        }

        Commands::Metadata(args) => {
            let config = AppConfig::new(&args.config)?;

            let template = metadata::build_template(&config);

            println!("{}", template);
        }

        Commands::New { name } => {
            let global_config = GlobalConfig::new()?;

            let root_dir = Path::new(&name);

            if root_dir.exists() {
                return Err(anyhow!("{} already exists", root_dir.display()));
            }

            fs::create_dir(root_dir)?;

            let mut app_config = AppConfig::prompt()?;

            let config_file_path = root_dir.join("pix.json");

            let images_path = root_dir.join(&app_config.path);

            fs::create_dir(&images_path)?;

            for layer in &app_config.layers {
                let layer_path = images_path.join(layer);

                fs::create_dir(&layer_path)?;
            }

            if let Some(nft_maker_config) = global_config.nft_maker {
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("would you like to create a new project on https://nft-maker.io?")
                    .default(false)
                    .interact()?
                {
                    let metadata = metadata::build_template(&app_config);

                    let body = CreateProjectRequest::new(
                        &app_config,
                        metadata,
                        "2022-02-20T00:00:00.988Z".to_string(),
                    );

                    let nft_maker = NftMakerClient::new(nft_maker_config.apikey)?;

                    let data = nft_maker.create_project(&body)?;

                    app_config.nft_maker = Some(NftMakerLocalConfig {
                        apikey: "".to_string(),
                        nft_project_id: data.project_id,
                    })
                }
            }

            let contents = serde_json::to_string_pretty(&app_config)?;

            fs::write(config_file_path, contents)?;

            println!("\nDone! ✅ To get started:\n");
            println!("cd {}", &name);
            println!("and add some traits into the images/ directory 🚀");
        }
        Commands::Upload(args) => {
            if !output.exists() {
                return Err(anyhow!("no output found, try running gen first"));
            }

            let config = AppConfig::new(&args.config)?;

            if let Some(nft_maker_config) = config.nft_maker {
                let nft_maker = NftMakerClient::new(nft_maker_config.apikey)?;

                let output_dir = output
                    .read_dir()
                    .with_context(|| format!("{} is not a folder", output.display()))?
                    .map(|dir| dir.unwrap().path());

                let progress = ProgressBar::new(config.amount as u64);

                for nft_path in output_dir {
                    let nft_name = nft_path.file_name().unwrap().to_str().unwrap();

                    let split_name: Vec<&str> = nft_name.split('#').collect();

                    let number = split_name[1];

                    let nft_file_path = nft_path.join(format!("{}.png", nft_name));

                    let nft_attributes_file_path = nft_path.join(format!("{}.json", nft_name));

                    let nft_attributes_file = fs::File::open(&nft_attributes_file_path)?;

                    let nft = fs::read(&nft_file_path)?;

                    let nft_attributes = serde_json::from_reader(&nft_attributes_file)?;

                    if let Value::Object(attributes) = nft_attributes {
                        let nft_base64 = base64::encode(nft);

                        let metadata_placeholder: Vec<MetadataPlaceholder> = attributes
                            .values()
                            .enumerate()
                            .map(|(index, attr)| {
                                if let Value::String(attr) = attr {
                                    MetadataPlaceholder {
                                        name: Some(format!("attribute{}", index)),
                                        value: Some(attr.to_owned()),
                                    }
                                } else {
                                    eprintln!("attribute values should be strings");

                                    process::exit(1);
                                }
                            })
                            .collect();

                        let body = UploadNftRequest {
                            asset_name: Some(format!("{}{}", config.name, number)),
                            preview_image_nft: NftFile {
                                mimetype: Some(String::from("image/png")),
                                description: None,
                                displayname: Some(format!(
                                    "{} #{}",
                                    config.display_name.as_ref().unwrap_or(&config.name),
                                    number
                                )),
                                file_from_IPFS: None,
                                file_froms_url: None,
                                file_from_base64: Some(nft_base64),
                                metadata_placeholder,
                            },
                            subfiles: vec![],
                            metadata: None,
                        };

                        let _data = nft_maker
                            .upload_nft(nft_maker_config.nft_project_id, &body)
                            .expect("failed to upload nft");

                        progress.inc(1);

                        std::thread::sleep(Duration::from_millis(10));
                    } else {
                        return Err(anyhow!("failed to read nft attributes"));
                    }
                }

                progress.finish();
            } else {
                return Err(anyhow!("please provide an nft_maker config to upload"));
            }
        }
    }

    Ok(())
}
