// mode/sha1/name以外は,後ほどプロジェクトからindexを生成するときに必要になるはず.
#[allow(dead_code)]
pub struct Entry {
    // ctime/mtime の型は chrono に合わせる
    csec: i64,
    cnano: u32,
    msec: i64,
    mnano: u32,
    dev: u32,
    inode: u32,
    mode: String,
    uid: u32,
    guid: u32,
    size: u32,
    sha1: String,
    pub name: String
}

impl Entry {
    pub fn new(csec: i64, cnano: u32, msec: i64, mnano: u32, dev: u32, inode: u32, mode: String,
            uid: u32, guid: u32, size: u32, sha1: String, name: String) -> Self {
        Self {
            csec: csec, cnano: cnano, msec: msec, mnano: mnano, dev: dev, inode: inode,
            mode: mode, uid: uid, guid: guid, size: size, sha1: sha1, name: name
        }
    }
}

impl Entry {
    pub fn get_file_verbose(&self) -> String {
        format!("{mode} {sha1}\t\t{name}", mode=self.mode, sha1=self.sha1, name=self.name)
    }
}