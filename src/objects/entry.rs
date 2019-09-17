pub struct Entry {
    sha1: String,
    pub name: String
}

impl Entry {
    pub fn new(sha1: String, name: String) -> Self {
        Self {sha1: sha1, name: name}
    }
}

impl Entry {
    pub fn get_file_verbose(&self) -> String {
        format!("-rw-r--r-- {sha1}\t\t{name}", sha1=self.sha1, name=self.name)
    }
}
