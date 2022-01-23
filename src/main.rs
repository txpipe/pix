use image::{imageops, GenericImage, GenericImageView, RgbaImage};

use nft_gen::{
    cli::Args,
    traits::{SimpleTrait, Traits},
};

fn main() -> Result<(), String> {
    let args = Args::new();

    let (mut x, mut y) = (0, 0);

    let order = vec![
        "background",
        "eyes",
        "Base",
        "Stitch Color",
        "belly",
        "forehead",
        "Stuffing",
    ];

    let mut traits = Traits::new();

    for entry in args.path.read_dir().expect("path is not a directory") {
        if let Ok(entry) = entry {
            let mut simple_trait = SimpleTrait::new();

            let dir_path = entry.path();

            for rarity in dir_path
                .read_dir()
                .expect("trait must be a folder")
                .filter(|x| x.as_ref().unwrap().path().is_dir())
            {
                if let Ok(rarity) = rarity {
                    let rarity_path = rarity.path();

                    for trait_file in rarity_path.read_dir().expect("rarity must be a directory") {
                        if let Ok(trait_file) = trait_file {
                            let trait_path = trait_file.path();

                            let image = image::open(trait_path).expect("file not found");

                            let (width, height) = image.dimensions();

                            x = width;
                            y = height;

                            match rarity_path.file_name().unwrap().to_str() {
                                Some("common") => simple_trait.common.push(image),
                                Some("uncommon") => simple_trait.uncommon.push(image),
                                Some("rare") => simple_trait.rare.push(image),
                                Some("mythical") => simple_trait.mythical.push(image),
                                Some("legendary") => simple_trait.legendary.push(image),
                                _ => (),
                            }
                        }
                    }
                }
            }

            traits.insert(dir_path, simple_trait);
        }
    }

    let mut base = RgbaImage::new(x, y);

    for item in order {
        let simple_trait = traits.get(&args.path.join(item)).unwrap();

        merge(&mut base, &simple_trait.legendary[0]);
    }

    base.save("output2.png").unwrap();

    Ok(())
}

fn merge<I, J>(bottom: &mut I, top: &J)
where
    I: GenericImage,
    J: GenericImageView<Pixel = I::Pixel>,
{
    imageops::overlay(bottom, top, 0, 0);
}
