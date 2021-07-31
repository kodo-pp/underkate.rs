mod app;
mod geometry;
mod graphics;
mod overworld;
mod resources;
mod screen;
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
