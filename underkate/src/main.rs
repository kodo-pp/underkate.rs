mod app;
mod default_runtime;
mod dialog;
mod game_context;
mod geometry;
mod graphics;
mod handle;
mod overworld;
mod resources;
mod screen;
mod script;
mod ui_event;

fn main() {
    match app::run() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("An error occurred! {}", e);
            eprintln!("Debug information: {:?}", e);
        }
    }
}
