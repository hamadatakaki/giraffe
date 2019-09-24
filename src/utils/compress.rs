use libflate::zlib;
use std::io::Read;

pub fn decompress_zlib(compressed: Vec<u8>) -> std::io::Result<Vec<u8>> {
    let mut decoder = zlib::Decoder::new(&compressed[..])?;
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    Ok(decompressed)
}