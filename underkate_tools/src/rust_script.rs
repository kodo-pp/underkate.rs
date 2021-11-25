use crate::file::read_file;
use crate::ASSETS_DIR;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use serde::Deserialize;
use syn::Lit;

#[derive(Deserialize)]
struct Manifest {
    file: String,
}

pub fn load_rust_script(tokens: TokenStream) -> TokenStream {
    let path_lit: Lit = syn::parse(tokens).expect("Expected path");
    let path_str = match path_lit {
        Lit::Str(lit_str) => lit_str.value(),
        _ => panic!("Path must be a string literal"),
    };

    let dir_full_path = format!("{}/scripts/{}", ASSETS_DIR, path_str);
    let manifest_full_path = format!("{}/script.toml", dir_full_path);

    let manifest: Manifest =
        toml::from_str(&read_file(&manifest_full_path)).expect("Failed to parse manifest file");

    let rust_code = read_file(format!("{}/{}", dir_full_path, manifest.file));
    let rust_code_tokens: TokenStream2 = rust_code.parse().unwrap();
    let module_name = format_ident!(
        "script_{}",
        blake3::hash(path_str.as_bytes()).to_hex().as_str(),
    );

    let result = quote! {{
        mod #module_name {
            #rust_code_tokens
        }

        crate::script::rust_script::RustScript::new(#module_name::main)
    }};
    result.into()
}
