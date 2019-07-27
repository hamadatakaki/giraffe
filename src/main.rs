pub mod details;

fn main() {
    details::cat_file("./repo/blobobj");
    details::cat_file_verbose("./repo/commitobj");
}
