use byteorder::{BigEndian, WriteBytesExt};

pub struct Entry {
    mode: Mode,
    sha1: String,
    pub name: String
}

pub struct Mode {
    mode: u32,
    is_directory: bool
}

impl Mode {
    pub fn new(mode: u32, is_directory: bool) -> Self {
        Self {mode: mode, is_directory: is_directory}
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut wtr = vec![];
        wtr.write_u32::<BigEndian>(self.calc_mode_number()).unwrap();
        wtr
    }

    fn calc_mode_number(&self) -> u32 {
        self.mode + if self.is_directory {
            1000
        } else {
            0
        }
    }
}

impl Entry {
    pub fn new(mode: Mode, sha1: String, name: String) -> Self {
        Self {mode: mode, sha1: sha1, name: name}
    }
}

impl Entry {
    pub fn generalize_mode(&self) -> String {
        format!("{:o}", self.mode.mode+8u32.pow(3))
    }

    pub fn get_file_verbose(&self) -> String {
        format!("{mode} {sha1}\t\t{name}", mode=self.generalize_mode(), sha1=self.sha1, name=self.name)
    }
}
