use crate::utils::read_file_all;
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

    // TODO: 生成過程のテストを書く
    pub fn make_object_from_path(path: &str) -> Result<Self, Box<std::error::Error>> {
        let buf = read_file_all(path)?;
        let (identifer, header_and_body) = buf.split_at(4);

        assert!(is_index(identifer.to_vec()));

        let (header, _body) = header_and_body.split_at(8);
        let (version, entry_number) = Self::analyze_header(header)?;
        let mut entries: Vec<Entry> = Vec::new();

        let mut c: u32 = 0;

        loop {
            let (mut ctime_sec, _body) = _body.split_at(4);
            let (mut ctime_nano, _body) = _body.split_at(4);
            let (mut mtime_sec, _body) = _body.split_at(4);
            let (mut mtime_nano, _body) = _body.split_at(4);
            let (mut dev, _body) = _body.split_at(4);
            let (mut inode, _body) = _body.split_at(4);
            let (mut mode, _body) = _body.split_at(4);
            let (mut uid, _body) = _body.split_at(4);
            let (mut guid, _body) = _body.split_at(4);
            let (mut size, _body) = _body.split_at(4);
            let (sha1, _body) = _body.split_at(20);

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
            let sha1 = sha1.iter().map(|x| format!("{:o}", x)).collect::<String>();

            let (mut name_length, _body) = _body.split_at(2);
            let name_length = name_length.read_u32::<BigEndian>()? as usize;
            let (name, _body) = _body.split_at(name_length);
            let name = std::str::from_utf8(name)?.to_string();
            let (_padding, _body) = _body.split_at(8-(6+name_length)%8);

            println!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", ctime_sec, ctime_nano, mtime_sec, mtime_nano, dev, inode, mode, uid, guid, size, sha1, name, (8-(6+name_length)%8));

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


#[cfg(test)]
mod tests {
    #[test]
    fn test_convert_byte_to_number() {
        
    }
}