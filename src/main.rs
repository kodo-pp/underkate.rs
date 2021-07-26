mod app;

fn main() {
    match app::run() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("An error occurred! {}", e);
            eprintln!("Debug information: {:?}", e);
        }
    }
}
