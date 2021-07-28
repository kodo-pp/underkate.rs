use crate::graphics::texture::Texture;
use ggez::graphics::{FilterMode, Image};
use ggez::Context;
use include_dir::{include_dir, Dir};
use std::path::{Path, PathBuf};
use std::time::Duration;

const ASSETS: Dir = include_dir!("assets");

pub fn file<S: AsRef<Path>>(path: S) -> &'static [u8] {
    ASSETS.get_file(path).unwrap().contents()
}

pub fn texture_image<S: AsRef<Path>>(ctx: &mut Context, path: S) -> Image {
    let mut full_path = PathBuf::from("textures");
    full_path.push(path);

    let mut image = Image::from_bytes(ctx, file(full_path)).expect("Failed to load texture");
    image.set_filter(FilterMode::Nearest);
    image
}

pub fn static_texture(ctx: &mut Context, path: &str, scale_factor: f32) -> Texture {
    Texture::new_static(texture_image(ctx, path), scale_factor)
}

pub fn animated_texture(
    ctx: &mut Context,
    paths: &[&str],
    frame_interval: Duration,
    scale_factor: f32,
) -> Texture {
    Texture::new_animated(
        paths
            .iter()
            .copied()
            .map(|path| texture_image(ctx, path))
            .collect(),
        frame_interval,
        scale_factor,
    )
}
