use crate::utils::{parse_from_vec_u8, fill_0_u8};
use super::entry::{Entry, Mode};

use byteorder::{BigEndian, ReadBytesExt};

pub fn is_index(buf: Vec<u8>) -> bool {
    String::from_utf8(buf).unwrap() == "DIRC".to_string()
}

#[allow(dead_code)]
pub struct Index {
    version: u32,
    entry_number: u32,
    entries: Vec<Entry>
}

impl Index {
    pub fn new(version: u32, entry_number: u32, entries: Vec<Entry>) -> Self {
        Self {
            version: version,
            entry_number: entry_number,
            entries: entries
        }
    }

    pub fn make_object(buf: Vec<u8>) -> Result<Self, Box<std::error::Error>> {
        let (identifer, header_and_body) = buf.split_at(4);

        assert!(is_index(identifer.to_vec()));

        let (header, mut _body) = header_and_body.split_at(8);
        let (version, entry_number) = analyze_header(header)?;

        let mut entries: Vec<Entry> = Vec::new();
        let mut c: u32 = 0;

        loop {
            let (mut mode, body) = _body.split_at(4);
            let (sha1, body) = body.split_at(20);

            // TODO: Consider how to save object's mode.
            let mode_u32 = mode.read_u32::<BigEndian>()?;
            let mode = Mode::new(mode_u32, false);
            let sha1 = sha1.iter().map(|x| fill_0_u8(*x)).collect::<String>();

            let (name_length, body) = body.split_at(2);
            let name_length = parse_from_vec_u8(name_length) as usize;
            let (name, body) = body.split_at(name_length);
            let name = std::str::from_utf8(name)?.to_string();
            let (_padding, body) = body.split_at(8-(6+name_length)%8);
            _body = body;

            let entry = Entry::new(mode, sha1, name);
            entries.push(entry);
            c += 1;

            if c == entry_number {
                break;
            }
        }
        Ok(Self::new(version, entry_number, entries))
    }
}

fn analyze_header(header: &[u8]) -> Result<(u32, u32), Box<std::error::Error>> {
    let (mut version_bytes, mut entry_number_bytes) = header.split_at(4);

    let version = version_bytes.read_u32::<BigEndian>()?;
    let entry_number = entry_number_bytes.read_u32::<BigEndian>()?;
    Ok((version, entry_number))
}
