use std::io;

use image::imageops;

use nft_gen::cli::Args;

fn main() -> Result<(), io::Error> {
    let args = Args::new();

    let mut image1 = image::open("example/image1.png").unwrap();
    let image2 = image::open("example/image2.png").unwrap();

    imageops::overlay(&mut image1, &image2, 0, 0);

    image1.save("example/output.png").unwrap();

    println!("{:#?}", args);

    Ok(())
}
