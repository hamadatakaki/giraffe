extern crate giraffe;
use crate::giraffe::command;

fn main() -> Result<(), std::io::Error> {
    command::init()?;
    Ok(())
}
