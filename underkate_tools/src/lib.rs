mod texture;
mod file;

use proc_macro::TokenStream;

const ASSETS_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets");

#[proc_macro]
pub fn load_texture(tokens: TokenStream) -> TokenStream {
    texture::load_texture(tokens)
}
