use crate::utils::{read_file_all, parse_from_vec_u8, fill_0_u8};
use super::entry::Entry;

use byteorder::{BigEndian, ReadBytesExt};

pub fn is_index(buf: Vec<u8>) -> bool {
    String::from_utf8(buf).unwrap() == "DIRC".to_string()
}

// indexを生成する過程で使う可能性があるので定義しておく
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

    pub fn make_object_from_path(path: &str) -> Result<Self, Box<std::error::Error>> {
        let buf = read_file_all(path)?;
        let (identifer, header_and_body) = buf.split_at(4);

        assert!(is_index(identifer.to_vec()));

        let (header, mut _body) = header_and_body.split_at(8);
        let (version, entry_number) = Self::analyze_header(header)?;
        let mut entries: Vec<Entry> = Vec::new();

        let mut c: u32 = 0;

        loop {
            let (mut ctime_sec, body) = _body.split_at(4);
            let (mut ctime_nano, body) = body.split_at(4);
            let (mut mtime_sec, body) = body.split_at(4);
            let (mut mtime_nano, body) = body.split_at(4);
            let (mut dev, body) = body.split_at(4);
            let (mut inode, body) = body.split_at(4);
            let (mut mode, body) = body.split_at(4);
            let (mut uid, body) = body.split_at(4);
            let (mut guid, body) = body.split_at(4);
            let (mut size, body) = body.split_at(4);
            let (sha1, body) = body.split_at(20);

            let ctime_sec = ctime_sec.read_u32::<BigEndian>()? as i64;
            let ctime_nano = ctime_nano.read_u32::<BigEndian>()?;
            let mtime_sec = mtime_sec.read_u32::<BigEndian>()? as i64;
            let mtime_nano = mtime_nano.read_u32::<BigEndian>()?;
            let dev = dev.read_u32::<BigEndian>()?;
            let mode_u32 = mode.read_u32::<BigEndian>()?;
            let mode = format!("{:o}", mode_u32);
            let inode = inode.read_u32::<BigEndian>()?;
            let uid = uid.read_u32::<BigEndian>()?;
            let guid = guid.read_u32::<BigEndian>()?;
            let size = size.read_u32::<BigEndian>()?;
            let sha1 = sha1.iter().map(|x| fill_0_u8(*x)).collect::<String>();

            let (name_length, body) = body.split_at(2);
            let name_length = parse_from_vec_u8(name_length) as usize;
            let (name, body) = body.split_at(name_length);
            let name = std::str::from_utf8(name)?.to_string();
            let (_padding, body) = body.split_at(8-(6+name_length)%8);
            _body = body;

            let entry = Entry::new(ctime_sec, ctime_nano, mtime_sec, mtime_nano, dev, inode, mode, uid, guid, size, sha1, name);
            entries.push(entry);
            c += 1;

            if c == entry_number {
                break;
            }
        }
        Ok(Self::new(version, entry_number, entries))
    }

    pub fn list_files(&self) -> String {
        self.entries.iter().fold("".to_string(), |text, e| format!("{}{}\n", text, e.name))
    }

    pub fn list_files_verbose(&self) -> String {
        self.entries.iter().fold("".to_string(), |text, e| format!("{}{}\n", text, e.get_file_verbose()))
    }
}

impl Index {
    fn analyze_header(header: &[u8]) -> Result<(u32, u32), Box<std::error::Error>> {
        let (mut version_bytes, mut entry_number_bytes) = header.split_at(4);

        let version = version_bytes.read_u32::<BigEndian>()?;
        let entry_number = entry_number_bytes.read_u32::<BigEndian>()?;
        Ok((version, entry_number))
    }
}
