use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::fmt::Display;

pub fn read_file<P: AsRef<Path> + Display>(path: P) -> String {
    let error_string = format!("Failed to open file {}", path);
    let mut buf = String::new();
    File::open(path)
        .expect(&error_string)
        .read_to_string(&mut buf)
        .expect(&error_string);
    buf
}
