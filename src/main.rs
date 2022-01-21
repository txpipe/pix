use image::{imageops, GenericImage, GenericImageView, RgbaImage};

use nft_gen::{cli::Args, traits::Traits};

fn main() -> Result<(), String> {
    let args = Args::new();

    // let traits = Traits::new();

    let mut images = Vec::new();

    for entry in args.path.read_dir().expect("path is not a directory") {
        if let Ok(entry) = entry {
            let image = image::open(entry.path()).expect("file not found");

            images.push(image);
        }
    }

    let (x, y) = images[0].dimensions();

    let mut base = RgbaImage::new(x, y);

    for image in images {
        merge(&mut base, &image);
    }

    base.save("example/output.png").unwrap();

    Ok(())
}

fn merge<I, J>(bottom: &mut I, top: &J)
where
    I: GenericImage,
    J: GenericImageView<Pixel = I::Pixel>,
{
    imageops::overlay(bottom, top, 0, 0);
}
