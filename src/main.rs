extern crate giraffe;
use crate::giraffe::git_command;

fn main() -> Result<(), Box<std::error::Error>> {
    git_command::ls_files_stage("./repo/index")?;
    Ok(())
}
