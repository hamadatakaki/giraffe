// pub mod tests_command_initialize {
//     use std::fs;
//     use std::path::Path;

//     pub fn initialize_for_init_not_initialized() {
//         let path = Path::new("./.repo");
//         if path.exists() {
//             fs::remove_dir_all(path);
//         }
//     }

//     pub fn initialize_for_init_already_initialized() -> std::io::Result<()> {
//         fs::create_dir_all("./.repo/objects")?;
//         fs::create_dir_all("./.repo/refs/heads")?;
//         fs::write(".repo/HEAD", "refs/heads/master\n")?;
//         fs::write(".repo/refs/heads/master", "")?;
//         Ok(())
//     }
// }