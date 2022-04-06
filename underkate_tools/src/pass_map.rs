use crate::file::read_file;
use crate::ASSETS_DIR;
use image::io::Reader as ImageReader;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Literal, TokenTree};
use quote::quote;
use serde::Deserialize;
use crate::args::Args;

#[derive(Deserialize)]
struct Manifest {
    #[serde(rename = "image")]
    image_path: String,
    scale: f32,
}

pub fn load_pass_map(args: &Args) -> TokenStream1 {
    let dir_full_path = format!("{}/{}", ASSETS_DIR, args.path);
    let manifest_full_path = format!("{}/pass-map.toml", dir_full_path);

    let manifest: Manifest =
        toml::from_str(&read_file(&manifest_full_path)).expect("Failed to parse manifest file");

    let image = ImageReader::open(format!("{}/{}", dir_full_path, manifest.image_path))
        .expect("Failed to open pass map image file")
        .decode()
        .expect("Failed to decode pass map image file")
        .into_luma8();
    let width = image.width() as usize;
    let height = image.height() as usize;
    let scale = manifest.scale;
    let image_bytes = &image as &[u8];
    let image_bytes_lit = TokenTree::Literal(Literal::byte_string(image_bytes));

    let result = quote! {
        crate::overworld::pass_map::BitmapPassMap::new(
            crate::overworld::pass_map::bitmap_pass_map::Bitmap::new(
                #width,
                #height,
                #image_bytes_lit,
            ),
            #scale,
        )
    };
    result.into()
}
