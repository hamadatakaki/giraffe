use std::fs::{create_dir_all, write};
use std::path::Path;

pub fn init() -> std::io::Result<i32> {
    let repo_path = Path::new("./.repo");

    if repo_path.exists() {
        print!("already initialized\n");
        Ok(1)
    } else {
        create_dir_all("./.repo/objects")?;
        create_dir_all("./.repo/refs/heads")?;
        write("./.repo/HEAD", "refs/heads/master\n")?;
        write("./.repo/refs/heads/master", "")?;
        Ok(0)
    }
}