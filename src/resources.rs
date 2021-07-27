use include_dir::{include_dir, Dir};
use std::path::Path;

const ASSETS: Dir = include_dir!("assets");

pub fn file<S: AsRef<Path>>(path: S) -> &'static [u8] {
    ASSETS.get_file(path).unwrap().contents()
}
