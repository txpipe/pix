use image::{imageops, GenericImage, GenericImageView, RgbaImage};

use nft_gen::{
    cli::{Commands, Mode},
    config::Config,
    traits::{Trait, Traits},
};
use rand::Rng;

const RARITIES: [&str; 5] = ["common", "uncommon", "rare", "mythical", "legendary"];

fn main() -> Result<(), String> {
    let cmds = Commands::new();

    match cmds {
        Commands::New(_args) => {
            todo!("implement new command")
        }
        Commands::Gen(args) => {
            let config = Config::new(&args.config);

            let mut traits = Traits::new();

            let (x, y) = load_traits(&mut traits, &config);

            let mut base = RgbaImage::new(x, y);

            for item in config.order {
                let trait_list = traits.get(&config.path.join(item)).unwrap();

                let mut rng = rand::thread_rng();

                let index = rng.gen_range(0..trait_list.len());

                merge(&mut base, &trait_list[index].image);
            }

            base.save("output.png").unwrap();
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

fn load_traits(traits: &mut Traits, config: &Config) -> (u32, u32) {
    let (mut x, mut y) = (0, 0);

    let feature_paths = config
        .path
        .read_dir()
        .expect("path is not a directory")
        .filter(|x| x.as_ref().unwrap().path().is_dir());

    for feature_dir in feature_paths {
        let category_dir = feature_dir.unwrap();
        let category_path = category_dir.path();

        let mut trait_list = Vec::new();

        match config.mode {
            Mode::Advanced => todo!("implement advanced mode"),
            Mode::Simple => {
                let rarity_paths = category_path
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

        traits.insert(category_path, trait_list);
    }

    (x, y)
}
