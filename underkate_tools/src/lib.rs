mod file;
mod pass_map;
mod room;
mod texture;
mod rust_script;

use proc_macro::TokenStream;

const ASSETS_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets");

#[proc_macro]
pub fn load_texture(tokens: TokenStream) -> TokenStream {
    texture::load_texture(tokens)
}

#[proc_macro]
pub fn load_room(tokens: TokenStream) -> TokenStream {
    room::load_room(tokens)
}

#[proc_macro]
pub fn load_pass_map(tokens: TokenStream) -> TokenStream {
    pass_map::load_pass_map(tokens)
}

#[proc_macro]
pub fn load_rust_script(tokens: TokenStream) -> TokenStream {
    rust_script::load_rust_script(tokens)
}
