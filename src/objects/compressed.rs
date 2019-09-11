// use crate::utils::{decompress_zlib, read_file_all};

#[allow(dead_code)]
enum ObjectType {
    Blob,
    Tree,
    Commit
}

#[allow(dead_code)]
pub struct StoredObject {
    body: String,
    object_type: ObjectType,
    length: u32
}