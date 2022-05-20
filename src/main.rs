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
        NftMakerLocalConfig, NftMakerNetwork, NftProjectId,
    },
    layers::Layers,
    metadata,
    nft_maker::{CreateProjectRequest, MetadataPlaceholder, NftMakerClient},
    rarity::Rarity,
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

            let (layer_sets, unique_sets) = if let Some(sets) = &config.sets {
                let mut layer_sets = Vec::new();

                let mut unique_sets = Vec::new();

                let mut offset = config.amount;

                let sets_total = sets.iter().fold(0, |acc, set| acc + set.amount);

                if sets_total != config.amount {
                    return Err(anyhow!("amount in sets must equal the total amount"));
                }

                for (set_index, set) in sets.iter().enumerate() {
                    let mut layers = Layers::default();

                    layers.load(
                        config.mode,
                        &config.layers,
                        config.path.join(set.name.clone()),
                    )?;

                    let mut fail_count = 0;

                    let mut uniques = HashSet::new();

                    let mut count = 1;

                    while count <= set.amount {
                        let unique = layers.create_unique(&config.layers, &set.name);

                        if uniques.contains(&unique) {
                            fail_count += 1;

                            if fail_count > config.tolerance {
                                println!(
                                    "You need more features or traits to generate {}",
                                    set.amount
                                );

                                process::exit(1);
                            }

                            continue;
                        }

                        uniques.insert(unique);

                        count += 1;
                    }

                    layer_sets.push(layers);

                    offset -= set.amount;

                    unique_sets.push((uniques, set_index, offset));
                }

                (layer_sets, unique_sets)
            } else {
                let mut layers = Layers::default();

                layers.load(config.mode, &config.layers, config.path)?;

                let mut fail_count = 0;

                let mut uniques = HashSet::new();

                let mut count = 1;

                while count <= config.amount {
                    let unique = layers.create_unique(&config.layers, "");

                    if uniques.contains(&unique) {
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

                    uniques.insert(unique);

                    count += 1;
                }

                (vec![layers], vec![(uniques, 0, 0)])
            };

            utils::clean(output)?;

            fs::create_dir(output)?;

            // Calculate rarity
            let mut rarity = Rarity::new(config.amount);

            for (uniques, set_index, _) in &unique_sets {
                for unique in uniques {
                    for (index, trait_list) in unique.iter().zip(&layer_sets[*set_index].data) {
                        let nft_trait = &trait_list[*index];

                        rarity.count_trait(&nft_trait.layer, &nft_trait.name);
                    }
                }
            }

            // Generate the images
            unique_sets
                .par_iter()
                .for_each(|(uniques, set_index, offset)| {
                    let layers = &layer_sets[*set_index];

                    uniques
                        .iter()
                        .enumerate()
                        .collect::<Vec<(usize, &Vec<usize>)>>()
                        .par_iter()
                        .for_each(|(mut count, unique)| {
                            if config.start_at_one {
                                count += 1
                            }

                            let mut base = RgbaImage::new(layers.width, layers.height);

                            let mut trait_info = Map::new();

                            let folder_name =
                                output.join(format!("{}#{}", config.name, count + offset));
                            fs::create_dir(&folder_name)
                                .expect("failed to created a folder for an NFT");

                            for (index, trait_list) in unique.iter().zip(&layers.data) {
                                let nft_trait = &trait_list[*index];

                                trait_info.insert(
                                    nft_trait.layer.to_owned(),
                                    Value::String(nft_trait.name.to_owned()),
                                );

                                if let Some(image) = &nft_trait.image {
                                    utils::merge(&mut base, image);
                                }
                            }

                            let nft_image_path =
                                folder_name.join(format!("{}#{}.png", config.name, count + offset));
                            let attributes_path = folder_name.join(format!(
                                "{}#{}.json",
                                config.name,
                                count + offset
                            ));
                            let metadata_path = folder_name.join("metadata.json");

                            base.save(nft_image_path).expect("failed to create image");

                            let attributes = serde_json::to_string_pretty(&trait_info)
                                .expect("failed to create attributes");

                            fs::write(attributes_path, attributes)
                                .expect("failed to create attributes");

                            let meta = metadata::build_with_attributes(
                                trait_info,
                                config.policy_id.clone(),
                                config.name.clone(),
                                config.display_name.as_ref(),
                                config.extra.clone(),
                                count + offset,
                            );

                            fs::write(metadata_path, meta).expect("failed to create metadata");

                            progress.inc(1);
                        });
                });

            let rarity_path = output.join("rarity.json");

            let rarity_data = serde_json::to_string_pretty(&rarity.data)?;

            fs::write(rarity_path, rarity_data)?;

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
                let layer_path = images_path.join(&layer.name);

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

                    let nft_maker =
                        NftMakerClient::new(nft_maker_config.apikey, NftMakerNetwork::Mainnet)?;

                    let data = nft_maker.create_project(&body)?;

                    app_config.nft_maker = Some(NftMakerLocalConfig {
                        network: NftMakerNetwork::Mainnet,
                        apikey: "".to_string(),
                        nft_project_id: NftProjectId::Id(data.project_id),
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
                let nft_maker =
                    NftMakerClient::new(nft_maker_config.apikey, nft_maker_config.network)?;

                let output_dir = output
                    .read_dir()
                    .with_context(|| format!("{} is not a folder", output.display()))?
                    .map(|dir| dir.unwrap().path())
                    .filter(|path| path.is_dir());

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

                        nft_maker.upload_nft(
                            &nft_maker_config.nft_project_id,
                            format!("{}{}", config.name, number),
                            String::from("image/png"),
                            format!(
                                "{} #{}",
                                config.display_name.as_ref().unwrap_or(&config.name),
                                number
                            ),
                            nft_base64,
                            metadata_placeholder,
                        )?;

                        progress.inc(1);

                        std::thread::sleep(Duration::from_micros(100));
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
