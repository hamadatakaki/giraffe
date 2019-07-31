extern crate giraffe;
use crate::giraffe::details::{cat_file, cat_file_verbose};

fn main() -> Result<(), Box<std::error::Error>> {
    cat_file("./repo/blobobj")?;
    cat_file_verbose("./repo/commitobj")?;
    Ok(())
}
