use std::io;

use image::{imageops, GenericImageView};

use nft_gen::cli::Args;

fn main() -> Result<(), io::Error> {
    let args = Args::new();

    let mut image1 = image::open("example/image1.png").unwrap();
    let image2 = image::open("example/image2.png").unwrap();

    let (x_1, y_1) = image1.dimensions();
    let (x_2, y_2) = image2.dimensions();

    imageops::overlay(&mut image1, &image2, x_1 - x_2, y_1 - y_2);

    image1.save("example/output.png").unwrap();

    println!("{:#?}", args);

    Ok(())
}
