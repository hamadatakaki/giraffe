use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::path::Path;
use crate::utils::{fill_0_for_index, read_file_all, normalize_name_length};
use super::compressed::GiraffeObject;

pub struct Blob {
    hash: String,
    name: String,
    body: Vec<u8>,
}

impl Blob {
    pub fn new(hash: String, name: String, body: Vec<u8>) -> Self {
        Self {
            hash: hash, name: name, body: body
        }
    }

    pub fn create_object(path: &Path) -> std::io::Result<Self> {
        let buf = read_file_all(path)?;
        let file_name = path.file_name().unwrap()
                            .to_str().unwrap();
        let mut hasher = Sha1::new();
        hasher.input(buf.as_slice());
        let hex = hasher.result_str();

        Ok(Blob::new(hex, String::from(file_name), buf))
    }
}

impl GiraffeObject for Blob {
    fn encode_to_object(&self) -> Vec<u8> {
        let obj_type = String::from("blob");
        let len = self.body.len();
        let head = format!("{} {}", obj_type, len);
        let mut obj_raw = head.into_bytes();
        obj_raw.push(0);
        obj_raw.append(self.body.clone().as_mut());
        obj_raw
    }

    fn encode_to_entry(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = self.hash.clone().into_bytes();
        vec.append(&mut normalize_name_length(self.name.len()));
        vec.append(&mut self.name.clone().into_bytes());
        vec.append(&mut fill_0_for_index(self.name.len()));
        vec
    }

    fn generate_path_string(&self) -> String {
        let mut path = self.hash.clone();
        path.insert_str(2, "/");
        format!("./experiment/.repo/objects/{}", path)
    }
}
