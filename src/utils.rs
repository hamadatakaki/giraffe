use std::fs::File;
use std::io::Read;
use std::path::Path;
use libflate::zlib;

pub fn read_file_all(path: &Path) -> Result<Vec<u8>, Box<std::error::Error>> {
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

pub fn parse_from_vec_u8(vec: &[u8]) -> u32 {
    let mut reverced_vec: Vec<u32> = Vec::new();
    for u in vec {
        reverced_vec.push(*u as u32);
    }

    let mut i = 0;
    let mut sum = 0;

    loop {
        let num: u32 = reverced_vec.pop().unwrap();
        sum += num * 8u32.pow(i);
        i += 1;

        if reverced_vec.is_empty() {
            break;
        }
    }

    sum
}

pub fn fill_0_u8(u: u8) -> String {
    format!("{:>02}", format!("{:x}", u)).replace(" ", "0")
}

pub fn normalize_name_length(name_length: usize) -> Vec<u8> {
    // 2byteに前方を0埋めしたlengthを突っ込む
    let mut vec = Vec::new();
    vec.push((name_length / 16) as u8);
    vec.push((name_length % 16) as u8);
    vec
}

pub fn fill_0_for_index(name_len: usize) -> Vec<u8> {
    let pad_num = 8 - (6+name_len)%8;
    let mut vec = Vec::new();
    vec.resize(pad_num, 0);
    vec
}
