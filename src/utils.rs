use std::fs::File;
use std::io::Read;
use libflate::zlib;

pub fn read_file_all(path: &str) -> Result<Vec<u8>, Box<std::error::Error>> {
    // Read file
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    Ok(buf)
}

pub fn decompress_zlib(compressed: Vec<u8>) -> Result<Vec<u8>, Box<std::error::Error>> {
    let mut decoder = zlib::Decoder::new(&compressed[..])?;
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    Ok(decompressed)
}