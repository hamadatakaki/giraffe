pub mod cat_file;

fn main() {
    cat_file::cat_file("./repo/blobobj");
    cat_file::cat_file_verbose("./repo/commitobj");
}
