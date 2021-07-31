use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use syn::Lit;

const ASSETS_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets");

fn read_file<P: AsRef<Path> + Display>(path: P) -> String {
    let error_string = format!("Failed to open file {}", path);
    let mut buf = String::new();
    File::open(path)
        .expect(&error_string)
        .read_to_string(&mut buf)
        .expect(&error_string);
    buf
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum Manifest {
    Static {
        #[serde(rename = "image")]
        image_path: String,
        scale: f32,
    },
    Animated {
        #[serde(rename = "images")]
        image_paths: Vec<String>,
        fps: f64,
        scale: f32,
    },
}

#[proc_macro]
pub fn load_texture(tokens: TokenStream) -> TokenStream {
    let path_lit: Lit = syn::parse(tokens).expect("Expected path");
    let path_str = match path_lit {
        Lit::Str(lit_str) => lit_str.value(),
        _ => panic!("Path must be a string literal"),
    };

    let dir_full_path = format!("{}/textures/{}", ASSETS_DIR, path_str);
    let manifest_full_path = format!("{}/texture.toml", dir_full_path);

    let manifest: Manifest = toml::from_str(&read_file(&manifest_full_path))
        .expect("Failed to parse manifest file");

    match manifest {
        Manifest::Static { image_path, scale } => {
            quote! {
                crate::graphics::texture::Texture::new_static(
                    ::ggez::graphics::Image::from_bytes(
                        ctx,
                        include_bytes!(stringify!(#image_path)),
                    ).expect("Failed to decode image from bytes"),
                    #scale,
                )
            }
        }
        Manifest::Animated {
            image_paths,
            fps,
            scale,
        } => {
            let frame_interval_secs = fps.recip();
            quote! {
                crate::graphics::texture::Texture::new_animated(
                    vec![
                        #(
                            ::ggez::graphics::Image::from_bytes(
                                ctx,
                                include_bytes!(concat!(#dir_full_path, "/", #image_paths)),
                            ).expect("Failed to decode image from bytes"),
                        )*
                    ],
                    ::std::time::Duration::from_secs_f64(#frame_interval_secs),
                    #scale,
                )
            }
        }
    }
    .into()
}
