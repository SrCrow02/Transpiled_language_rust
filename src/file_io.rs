use std::fs;
use std::io::{self, Write};

pub fn read_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

pub fn write_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())
}
