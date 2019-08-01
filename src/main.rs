extern crate giraffe;
use crate::giraffe::details::{cat_file, cat_file_verbose, ls_files};

fn main() -> Result<(), Box<std::error::Error>> {
    ls_files("./repo/index")?;
    Ok(())
}
