use std::{fs, path::Path};

use anyhow::Context;
use image::{imageops, GenericImage, GenericImageView};

pub fn merge<I, J>(bottom: &mut I, top: &J)
where
    I: GenericImage,
    J: GenericImageView<Pixel = I::Pixel>,
{
    imageops::overlay(bottom, top, 0, 0);
}

pub fn clean(output: &Path) -> anyhow::Result<()> {
    if output.exists() {
        fs::remove_dir_all(output)
            .with_context(|| format!("could not delete {}", output.display()))?
    }

    Ok(())
}
