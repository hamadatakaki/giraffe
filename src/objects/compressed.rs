use crate::utils::{decompress_zlib, read_file_all};

enum ObjectType {
    Blob,
    Tree,
    Commit
}

pub struct StoredObject {
    body: String,
    object_type: ObjectType,
    length: u32
}