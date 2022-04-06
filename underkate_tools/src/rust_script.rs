use crate::args::Args;
use crate::file::read_file;
use crate::ASSETS_DIR;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Deserialize)]
struct Manifest {
    file: String,
}

pub fn load_rust_script(args: &Args) -> TokenStream {
    let dir_full_path = format!("{}/{}", ASSETS_DIR, args.path);
    let manifest_full_path = format!("{}/script.toml", dir_full_path);

    let manifest: Manifest =
        toml::from_str(&read_file(&manifest_full_path)).expect("Failed to parse manifest file");

    let rust_code = read_file(format!("{}/{}", dir_full_path, manifest.file));
    let rust_code_tokens: TokenStream2 = rust_code.parse().unwrap();
    let module_name = format_ident!(
        "script_{}",
        blake3::hash(args.path.as_bytes()).to_hex().as_str(),
    );

    let result = quote! {{
        mod #module_name {
            mod script {
                #rust_code_tokens
            }

            pub fn main(
                script_handle: crate::script::ScriptHandle,
                context: crate::game_context::GameContext,
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()>>> {
                Box::pin(script::main(script_handle, context))
            }
        }

        crate::script::rust_script::RustScript::new(#module_name::main)
    }};
    result.into()
}
