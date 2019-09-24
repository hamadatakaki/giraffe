#[allow(dead_code)]
enum ObjectType {
    Blob,
    Tree,
    Commit
}

pub trait GiraffeObject {
    fn encode_to_object(&self) -> Vec<u8>;
    fn encode_to_entry(&self) -> Vec<u8>;
    fn generate_path_string(&self) -> String;
}
