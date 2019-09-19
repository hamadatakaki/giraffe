use std::fs::{create_dir_all, read_dir};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use super::objects::blob::Blob;
use super::objects::compressed::GiraffeObject;

pub fn init() -> std::io::Result<i32> {
    let repo_path = Path::new("./experiment/.repo");

    if repo_path.exists() {
        print!("already initialized\n");
        Ok(1)
    } else {
        create_dir_all("./experiment/.repo/objects")?;
        create_dir_all("./experiment/.repo/refs/heads")?;
        Ok(0)
    }
}

pub fn first_add() -> std::io::Result<()> {
    // let repo_path = Path::new("./experiment/.repo");
    // assert!(repo_path.exists());

    // TODO: Migrate this code to commit command.
    // write(".repo/HEAD", "refs/heads/master\n")?;

    // 1. Get all directory and file information recursively
    visit_dirs(Path::new("./experiment"))?;

    Ok(())
}

// TODO: Refactoring how we ignore particular file.
fn path_valid(path: &Path) -> bool {
    let ignore_path_str = vec!["./experiment/.repo"];
    path.is_dir() && !ignore_path_str.contains(&(path.to_str().unwrap()))
}

fn visit_dirs(path: &Path) -> std::io::Result<()> {
    if path_valid(path) {
        rec_visit_dirs(path)?;
    }
    Ok(())
}

fn rec_visit_dirs(path: &Path) -> std::io::Result<()> {
    // TODO: 2. Make blob object one by one.
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            let blob = Blob::create_object(&path).unwrap();
            let obj = blob.encode_to_entry();
            println!("{}", String::from_utf8(obj).unwrap());
        }
        if path_valid(&path) {
            rec_visit_dirs(&path)?;
        }
    }
    Ok(())
}
