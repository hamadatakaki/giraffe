use super::objects::{StoredObject, Index};

// Self::new(
//     csec, cnano, msec, mnano, dev, inode, 
//     mode, uid, guid, size, sha1, name
// )

pub fn cat_file(path: &str) -> Result<(), Box<std::error::Error>> { 
    let stored = StoredObject::make_object_from_path(path)?;
    print!("{}", stored.get_body());
    Ok(())
}

pub fn cat_file_verbose(path: &str) -> Result<(), Box<std::error::Error>> {
    let stored = StoredObject::make_object_from_path(path)?;
    println!("{}", stored.get_verbose());
    Ok(())
}

pub fn is_index(buf: Vec<u8>) -> bool {
    String::from_utf8(buf).unwrap() == "DIRC".to_string()
}

pub fn ls_files(path: &str) -> Result<(), Box<std::error::Error>> {
    let index = Index::make_object_from_path(path)?;
    for s in index.get_files_list() {
        println!("{}", s);
    }
    Ok(())
}