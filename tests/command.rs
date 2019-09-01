#[cfg(test)]
mod tests {
    use giraffe;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_init_not_initialized() {
        let repo_path = Path::new("./.repo");

        if repo_path.exists() {
            fs::remove_dir_all(repo_path);
        }

        assert_eq!(giraffe::command::init().unwrap(), 0);

        fs::remove_dir_all(repo_path);
    }

    #[test]
    fn test_init_already_initialized() -> std::io::Result<()> {
        fs::create_dir_all("./.repo/objects")?;
        fs::create_dir_all("./.repo/refs/heads")?;
        fs::write(".repo/HEAD", "refs/heads/master\n")?;
        fs::write(".repo/refs/heads/master", "")?;

        assert_eq!(giraffe::command::init().unwrap(), 1);

        fs::remove_dir_all("./.repo");

        Ok(())
    }
}