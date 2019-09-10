// mod settings;

// #[cfg(test)]
// mod tests {
//     use giraffe;
//     use std::fs;
//     use std::path::Path;

//     use crate::settings::tests_command_initialize;

//     #[test]
//     fn test_init_not_initialized() {
//         tests_command_initialize::initialize_for_init_not_initialized();
//         assert_eq!(giraffe::command::init().unwrap(), 0);
//         fs::remove_dir_all("./.repo");
//     }

//     #[test]
//     fn test_init_already_initialized() -> std::io::Result<()> {
//         tests_command_initialize::initialize_for_init_already_initialized()?;
//         assert_eq!(giraffe::command::init().unwrap(), 1);
//         fs::remove_dir_all("./.repo");
//         Ok(())
//     }
// }