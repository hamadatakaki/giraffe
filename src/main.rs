pub mod details;

fn main() -> Result<(), Box<std::error::Error>> {
    details::cat_file("./repo/blobobj")?;
    details::cat_file_verbose("./repo/commitobj")?;
    Ok(())
}
