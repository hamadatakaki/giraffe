pub struct Entry {
    mode: u32,
    sha1: String,
    pub name: String
}

impl Entry {
    pub fn new(mode: u32, sha1: String, name: String) -> Self {
        Self {mode: mode, sha1: sha1, name: name}
    }
}

impl Entry {
    pub fn generalize_mode(&self) -> String {
        format!("{:o}", self.mode+2u32.pow(15))
    }

    pub fn get_file_verbose(&self) -> String {
        format!("{mode} {sha1}\t\t{name}", mode=self.generalize_mode(), sha1=self.sha1, name=self.name)
    }
}