use crate::args::Args;
use crate::file::read_file;
use crate::ASSETS_DIR;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Manifest {
    background: String,
    pass_map: String,
    initial_player_states: HashMap<String, PlayerState>,
    scripts: Scripts,
}

#[derive(Deserialize)]
struct Scripts {
    init: Option<String>,
}

#[derive(Deserialize)]
struct PlayerState {
    x: f32,
    y: f32,
    direction: Direction,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

pub fn load_room(args: &Args) -> TokenStream {
    let dir_full_path = format!("{}/rooms/{}", ASSETS_DIR, args.path);
    let manifest_full_path = format!("{}/room.toml", dir_full_path);

    let manifest: Manifest =
        toml::from_str(&read_file(&manifest_full_path)).expect("Failed to parse manifest file");

    let background_path = manifest.background;
    let pass_map_path = manifest.pass_map;
    let initial_player_states_keys: Vec<_> = manifest.initial_player_states.keys().collect();
    let initial_player_states_values_x: Vec<_> = manifest
        .initial_player_states
        .values()
        .map(|s| s.x)
        .collect();
    let initial_player_states_values_y: Vec<_> = manifest
        .initial_player_states
        .values()
        .map(|s| s.y)
        .collect();
    let initial_player_states_values_dir: Vec<_> = manifest
        .initial_player_states
        .values()
        .map(|s| {
            format_ident!(
                "{}",
                match s.direction {
                    Direction::Backward => "Backward",
                    Direction::Forward => "Forward",
                    Direction::Left => "Left",
                    Direction::Right => "Right",
                }
            )
        })
        .collect();

    let init_script = if let Some(script) = manifest.scripts.init {
        quote! { Some(#script) }
    } else {
        quote! { None }
    };

    (quote! {
        crate::overworld::room::PartialCreationParams {
            background_path: ::std::string::String::from(#background_path),
            pass_map_path: ::std::string::String::from(#pass_map_path),
            initial_player_states: vec![
                #(
                    (
                        ::std::string::String::from(#initial_player_states_keys),
                        (
                            crate::geometry::OverworldPoint::<f32>::from([
                                #initial_player_states_values_x,
                                #initial_player_states_values_y,
                            ]),
                            crate::overworld::move_trait::Direction::#initial_player_states_values_dir,
                        )
                    )
                ),*
            ].into_iter().collect(),
            init_script: #init_script,
        }
    })
    .into()
}
