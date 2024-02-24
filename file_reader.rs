use std::fs;
use std::io;

pub fn read_file(file_path: &str) -> Vec<u8> {
    match fs::read(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            Vec::new()
        }
    }
}
