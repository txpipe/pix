use image::{imageops, GenericImage, GenericImageView, RgbaImage};

use nft_gen::{
    cli::Commands,
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
            let (mut x, mut y) = (0, 0);

            let config = Config::new(&args.config);

            let mut traits = Traits::new();

            for entry in config
                .path
                .read_dir()
                .expect("path is not a directory")
                .filter(|x| x.as_ref().unwrap().path().is_dir())
            {
                if let Ok(entry) = entry {
                    let mut trait_list = Vec::new();

                    let dir_path = entry.path();

                    for rarity in dir_path
                        .read_dir()
                        .expect("trait must be a folder")
                        .filter(|x| x.as_ref().unwrap().path().is_dir())
                        .filter(|x| {
                            RARITIES
                                .iter()
                                .any(|y| x.as_ref().unwrap().path().ends_with(y))
                        })
                    {
                        if let Ok(rarity) = rarity {
                            let rarity_path = rarity.path();

                            for trait_file in rarity_path
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
                                })
                            {
                                if let Ok(trait_file) = trait_file {
                                    let trait_path = trait_file.path();

                                    let image = image::open(trait_path).expect("file not found");

                                    let (width, height) = image.dimensions();

                                    x = width;
                                    y = height;

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

                    traits.insert(dir_path, trait_list);
                }
            }

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
