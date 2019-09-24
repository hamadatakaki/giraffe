use std::fs;
use std::io::{Read, Write};
use std::path::Path;

pub fn read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    // Read file
    let mut file = fs::File::open(path)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    Ok(buf)
}

pub fn create_file(path: &Path, body: Vec<u8>) -> std::io::Result<()> {
    let parent = path.parent().unwrap();
    fs::create_dir_all(parent)?;
    let mut file = fs::File::create(path)?;
    file.write_all(&body)?;
    Ok(())
}
