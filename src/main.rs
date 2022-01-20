use std::io;

use image::{imageops, GenericImage, GenericImageView};

use nft_gen::cli::Args;

fn main() -> Result<(), io::Error> {
    let args = Args::new();

    let image1_path = args.path.join("image1.png");

    let mut image1 = image::open(image1_path).unwrap();

    let image2_path = args.path.join("image2.png");

    let image2 = image::open(image2_path).unwrap();

    merge(&mut image1, &image2);

    image1.save("example/output.png").unwrap();

    Ok(())
}

fn merge<I, J>(bottom: &mut I, top: &J)
where
    I: GenericImage,
    J: GenericImageView<Pixel = I::Pixel>,
{
    imageops::overlay(bottom, top, 0, 0);
}
