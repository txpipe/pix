use std::{collections::HashSet, fs, path::Path, process};

use image::{imageops, GenericImage, GenericImageView, RgbaImage};
use rand::Rng;

use nft_gen::{
    cli::{Commands, Mode},
    config::Config,
    traits::{FeaturesMap, Trait},
};

const RARITIES: [&str; 5] = ["common", "uncommon", "rare", "mythical", "legendary"];

const OUTPUT: &str = "output";

fn main() -> Result<(), String> {
    let cmds = Commands::new();

    let output = Path::new(OUTPUT);

    match cmds {
        Commands::Clean => clean(output),
        Commands::New(_args) => {
            todo!("implement new command")
        }
        Commands::Gen(args) => {
            let config = Config::new(&args.config);

            let mut features_map = FeaturesMap::new();

            let (x, y) = load_features(&mut features_map, &config);

            clean(output);

            if !output.exists() {
                fs::create_dir(output).expect("failed to create output directory");
            }

            let mut features = Vec::new();

            for item in config.order {
                let (_, trait_list) = features_map.get_key_value(&config.path.join(item)).unwrap();

                features.push(trait_list);
            }

            let mut fail_count = 0;

            let mut uniques = HashSet::new();

            let mut count = 1;

            while count <= config.amount {
                let unique = create_unique(&features);

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

                let mut base = RgbaImage::new(x, y);

                for (index, trait_list) in unique.iter().zip(&features) {
                    merge(&mut base, &trait_list[*index].image);
                }

                let output_file = format!("{}/{}.png", OUTPUT, count);

                base.save(output_file).unwrap();

                uniques.insert(unique_str);

                count += 1;
            }
        }
    }

    Ok(())
}

fn merge<I, J>(bottom: &mut I, top: &J)
where
    I: GenericImage,
    J: GenericImageView<Pixel = I::Pixel>,
{
    imageops::overlay(bottom, top, 0, 0);
}

fn clean(output: &Path) {
    if output.exists() {
        fs::remove_dir_all(output).expect("could not")
    }
}

fn create_unique(features: &Vec<&Vec<Trait>>) -> Vec<usize> {
    let mut random = Vec::new();

    let mut rng = rand::thread_rng();

    for trait_list in features {
        let total_weight = trait_list.iter().fold(0, |acc, elem| acc + elem.weight);

        let random_num = rng.gen_range(0.0..1.0);

        let mut n = (random_num * total_weight as f64).floor();

        for (index, elem) in trait_list.iter().enumerate() {
            n -= elem.weight as f64;

            if n < 0.0 {
                random.push(index);

                break;
            }
        }
    }

    random
}

fn load_features(features_map: &mut FeaturesMap, config: &Config) -> (u32, u32) {
    let (mut x, mut y) = (0, 0);

    let feature_paths = config
        .path
        .read_dir()
        .expect("path is not a directory")
        .filter(|x| x.as_ref().unwrap().path().is_dir());

    for feature_dir in feature_paths {
        let feature_dir = feature_dir.unwrap();
        let feature_path = feature_dir.path();

        let mut trait_list = Vec::new();

        match config.mode {
            Mode::Advanced => todo!("implement advanced mode"),
            Mode::Simple => {
                let rarity_paths = feature_path
                    .read_dir()
                    .expect("trait must be a folder")
                    .filter(|x| x.as_ref().unwrap().path().is_dir())
                    .filter(|x| {
                        RARITIES
                            .iter()
                            .any(|y| x.as_ref().unwrap().path().ends_with(y))
                    });

                for rarity_dir in rarity_paths {
                    let rarity_dir = rarity_dir.unwrap();
                    let rarity_path = rarity_dir.path();

                    let trait_paths = rarity_path
                        .read_dir()
                        .expect("rarity must be a directory")
                        .filter(|x| x.as_ref().unwrap().path().is_file())
                        .filter(|x| {
                            x.as_ref()
                                .unwrap()
                                .path()
                                .extension()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                == "png"
                        });

                    for trait_file in trait_paths {
                        let trait_file = trait_file.unwrap();
                        let trait_path = trait_file.path();

                        let image = image::open(trait_path).expect("file not found");

                        let (width, height) = image.dimensions();

                        if x == 0 && y == 0 {
                            x = width;
                            y = height;
                        }

                        trait_list.push(Trait {
                            image,
                            weight: match rarity_path.file_name().unwrap().to_str() {
                                Some("common") => 70,
                                Some("uncommon") => 50,
                                Some("rare") => 20,
                                Some("mythical") => 10,
                                Some("legendary") => 5,
                                _ => unreachable!(),
                            },
                        })
                    }
                }
            }
        }

        features_map.insert(feature_path, trait_list);
    }

    (x, y)
}
