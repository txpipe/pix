use std::io;

use image::{imageops, GenericImage, GenericImageView, RgbaImage};

use nft_gen::cli::Args;

fn main() -> Result<(), io::Error> {
    let args = Args::new();

    let image1 = image::open("example/image1.png").unwrap();

    let (width, height) = image1.dimensions();

    let mut base = RgbaImage::new(width, height);

    let image2 = image::open("example/image2.png").unwrap();

    merge(&mut base, &image1);
    merge(&mut base, &image2);

    base.save("example/output.png").unwrap();

    for n in 1..=args.amount {
        println!("{}", n);
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
