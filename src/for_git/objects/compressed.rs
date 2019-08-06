use crate::utils::{decompress_zlib, read_file_all};

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
