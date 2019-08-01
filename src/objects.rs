use super::utils::{decompress_zlib, read_file_all};
use super::details::is_index;

use byteorder::{BigEndian, ReadBytesExt};

pub struct StoredObject {
    body: String,
    object_type: String,
    length: u32
}

impl StoredObject {
    pub fn new(body: String, obj_type: String, length: u32) -> Self {
        Self {
            body: body,
            object_type: obj_type,
            length: length
        }
    }

    pub fn make_object_from_path(path: &str) -> Result<Self, Box<std::error::Error>> {
        // Read file & decompress zlib
        let buf = read_file_all(path)?;
        let decompressed = decompress_zlib(buf)?;

        // extract file body.
        let index = match decompressed.windows(1).position(|w| w[0] == 0) {
            None => panic!("Not found separator literal '\\x00'"),
            Some(index) => index
        };
        let (header_bytes, body_bytes) = decompressed.split_at(index);
        let body = String::from_utf8(body_bytes.to_vec())?;

        // extract object-type and source length.
        let header = String::from_utf8(header_bytes.to_vec())?;
        let devided_header: Vec<&str> = header.split(" ").collect();
        assert!(devided_header.len()==2);
        let obj_type = devided_header[0].to_string();
        let length = devided_header[1].parse()?;

        Ok(Self::new(body, obj_type, length))
    }

    pub fn get_body(self) -> String {
        self.body
    }

    pub fn get_verbose(self) -> String {
        format!("object type: {}, length: {len:>3}\n{body}", self.object_type, len=self.length, body=self.body)
    }
}

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

    fn analyze_header(header: Vec<u8>) -> Result<(u32, u32), Box<std::error::Error>> {
        let (version_bytes, entry_number_bytes) = header.split_at(4);
        let version_str = std::str::from_utf8(version_bytes)?;
        let entry_number_str = std::str::from_utf8(entry_number_bytes)?;
        let version = version_str.parse()?;
        let entry_number = entry_number_str.parse()?;
        Ok((version, entry_number))
    }

    pub fn make_object_from_path(path: &str) -> Result<Self, Box<std::error::Error>> {
        let buf = read_file_all(path)?;
        let (identifer, header_and_body) = buf.split_at(4);

        assert!(is_index(identifer.to_vec()));

        let (header, _body) = header_and_body.split_at(8);
        let (version, entry_number) = Self::analyze_header(header.to_vec())?;
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

            println!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", ctime_sec, ctime_nano, mtime_sec, mtime_nano, dev, inode, mode, uid, guid, size, sha1, name);

            let entry = Entry::new(ctime_sec, ctime_nano, mtime_sec, mtime_nano, dev, inode, mode, uid, guid, size, sha1, name);
            entries.push(entry);
            c += 1;

            if c == entry_number {
                break;
            }
        }
        Ok(Self::new(version, entry_number, entries))
    }

    pub fn get_files_list(self) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();
        for e in self.entries {
            files.push(e.name);
        }
        files
    }
}

pub struct Entry {
    // ctime/mtime : 型は chrono に合わせる
    csec: i64,
    cnano: u32,
    msec: i64,
    mnano: u32,
    dev: u32,
    inode: u32,
    mode: String,
    uid: u32,
    guid: u32,
    size: u32,
    sha1: String,
    name: String
}

impl Entry {
    pub fn new(csec: i64, cnano: u32, msec: i64, mnano: u32, dev: u32, inode: u32, mode: String,
            uid: u32, guid: u32, size: u32, sha1: String, name: String) -> Self {
        Self {
            csec: csec, cnano: cnano, msec: msec, mnano: mnano, dev: dev, inode: inode,
            mode: mode, uid: uid, guid: guid, size: size, sha1: sha1, name: name
        }
    }
}