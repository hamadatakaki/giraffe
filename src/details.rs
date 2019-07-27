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

    // fn to_string(&self) -> String {
    //     format!("<{type}: {len: >3}>", type=self.object_type, len=self.length)
    // }
}

fn make_object_from_path(path: &str) -> Result<StoredObject, Box<std::error::Error>> {
    // Read file
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);

    // decompress zlib
    let mut decoder = zlib::Decoder::new(&buf[..]).unwrap();
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();

    // extract file body.
    let nullchar: u8 = 0;
    let index = decompressed.windows(1).position(|w| w[0] == nullchar).unwrap();
    let body_bytes = decompressed.split_off(index);
    let body = String::from_utf8(body_bytes).unwrap();

    // extract object-type and source length.
    let header = String::from_utf8(decompressed).unwrap();
    let devided_header: Vec<&str> = header.split(" ").collect();
    let obj_type = devided_header[0].to_string();
    let length: u32 = devided_header[1].parse().unwrap();

    Ok(StoredObject::new(body, obj_type, length))
}

pub fn cat_file(path: &str) {
    let stored = make_object_from_path(path).unwrap();
    print!("{}", stored.body);
}

pub fn cat_file_verbose(path: &str) {
    let stored = make_object_from_path(path).unwrap();
    println!("object type: {}, length: {len:>3}", stored.object_type, len=stored.length);
    print!("{}", stored.body);
}