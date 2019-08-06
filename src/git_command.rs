use super::for_git::objects::compressed::StoredObject;
use super::for_git::objects::index::Index;

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

pub fn ls_files(path: &str) -> Result<(), Box<std::error::Error>> {
    let index = Index::make_object_from_path(path)?;
    println!("{}", index.list_files());
    Ok(())
}

pub fn ls_files_stage(path: &str) -> Result<(), Box<std::error::Error>> {
    let index = Index::make_object_from_path(path)?;
    println!("{}", index.list_files_verbose());
    Ok(())
}