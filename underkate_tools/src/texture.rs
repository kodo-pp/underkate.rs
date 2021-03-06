use crate::args::Args;
use crate::file::read_file;
use crate::ASSETS_DIR;
use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;

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

pub fn load_texture(args: &Args) -> TokenStream {
    let dir_full_path = format!("{}/{}", ASSETS_DIR, args.path);
    let manifest_full_path = format!("{}/texture.toml", dir_full_path);

    let manifest: Manifest =
        toml::from_str(&read_file(&manifest_full_path)).expect("Failed to parse manifest file");

    match manifest {
        Manifest::Static { image_path, scale } => {
            quote! {
                crate::graphics::texture::Texture::new_static(
                    {
                        let mut image = ::ggez::graphics::Image::from_bytes(
                            ctx,
                            include_bytes!(concat!(#dir_full_path, "/", #image_path)),
                        ).expect("Failed to decode image from bytes");
                        image.set_filter(::ggez::graphics::FilterMode::Nearest);
                        image
                    },
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
                            {
                                let mut image = ::ggez::graphics::Image::from_bytes(
                                    ctx,
                                    include_bytes!(concat!(#dir_full_path, "/", #image_paths)),
                                ).expect("Failed to decode image from bytes");
                                image.set_filter(::ggez::graphics::FilterMode::Nearest);
                                image
                            },
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
