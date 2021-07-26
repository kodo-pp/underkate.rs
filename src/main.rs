mod app;
mod graphics;
mod screen;
mod ui_event;
mod overworld;

fn main() {
    match app::run() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("An error occurred! {}", e);
            eprintln!("Debug information: {:?}", e);
        }
    }
}
