use std::fs::File;
use std::io::Read;
use libflate::zlib;

struct StoredObject {
    body: String,
    object_type: String,
    length: u32
}

impl StoredObject {
    fn new(body: String, obj_type: String, length: u32) -> Self {
        Self {
            body: body,
            object_type: obj_type,
            length: length
        }
    }

    fn make_object_from_path(path: &str) -> Result<Self, Box<std::error::Error>> {
        // Read file
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf);

        // decompress zlib
        let mut decompressed = decompress_zlib(buf)?;

        // extract file body.
        let index = match decompressed.windows(1).position(|w| w[0] == 0) {
            None => panic!("Not found the separator literal '\\x00'"),
            Some(index) => index
        };
        let body_bytes = decompressed.split_off(index);
        let body = String::from_utf8(body_bytes)?;

        // extract object-type and source length.
        let header = String::from_utf8(decompressed)?;
        let devided_header: Vec<&str> = header.split(" ").collect();
        assert!(devided_header.len()==2);
        let obj_type = devided_header[0].to_string();
        let length = devided_header[1].parse()?;

        Ok(Self::new(body, obj_type, length))
    }
}

fn decompress_zlib(compressed: Vec<u8>) -> Result<Vec<u8>, Box<std::error::Error>> {
    let mut decoder = zlib::Decoder::new(&compressed[..])?;
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    Ok(decompressed)
}

pub fn cat_file(path: &str) -> Result<(), Box<std::error::Error>> { 
    let stored = StoredObject::make_object_from_path(path)?;
    print!("{}", stored.body);
    Ok(())
}

pub fn cat_file_verbose(path: &str) -> Result<(), Box<std::error::Error>> {
    let stored = StoredObject::make_object_from_path(path)?;
    println!("object type: {}, length: {len:>3}", stored.object_type, len=stored.length);
    print!("{}", stored.body);
    Ok(())
}