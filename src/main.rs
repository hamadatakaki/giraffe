extern crate giraffe;
use crate::giraffe::command::ls_files_stage;

fn main() -> Result<(), Box<std::error::Error>> {
    ls_files_stage("./repo/index")?;
    Ok(())
}
