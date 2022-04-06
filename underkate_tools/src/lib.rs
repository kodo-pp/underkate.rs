mod args;
mod codegen;
mod common;
mod file;
mod pass_map;
mod room;
mod rust_script;
mod texture;

use crate::args::{parse_args, parse_list_args};
use proc_macro::TokenStream;

const ASSETS_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets");

#[proc_macro]
pub fn load_texture(tokens: TokenStream) -> TokenStream {
    let args = parse_args(tokens).unwrap();
    texture::load_texture(&args)
}

#[proc_macro]
pub fn load_room(tokens: TokenStream) -> TokenStream {
    let args = parse_args(tokens).unwrap();
    room::load_room(&args)
}

#[proc_macro]
pub fn load_pass_map(tokens: TokenStream) -> TokenStream {
    let args = parse_args(tokens).unwrap();
    pass_map::load_pass_map(&args)
}

#[proc_macro]
pub fn load_rust_script(tokens: TokenStream) -> TokenStream {
    let args = parse_args(tokens).unwrap();
    rust_script::load_rust_script(&args)
}

#[proc_macro]
pub fn load_assets(tokens: TokenStream) -> TokenStream {
    let args = parse_list_args(tokens);
    codegen::generate_resource_storage_code(
        args.resource_specs
            .iter()
            .map(|(ref name, ref resource_type)| (name as &str, *resource_type)),
    )
    .into()
}
