use std::{
    fs::{copy, create_dir_all, remove_file},
    path::PathBuf,
};

pub struct ConfigFile {
    pub host_path: PathBuf,
}

impl ConfigFile {
    pub fn new(path: PathBuf) -> Self {
        return ConfigFile { host_path: path };
    }
}
impl ConfigFile {
    /// backs up a file to the backup location. mimics the dir structure relative to `/`
    pub fn backup(&self, backup_store_path: PathBuf) {
        if !self.host_path.as_path().exists() {
            return;
        }
        let backup_path = PathBuf::new()
            .join(backup_store_path.as_path())
            .join(self.host_path.as_path().strip_prefix("/Users/kbc").unwrap());
        create_dir_all(backup_path.as_path().parent().unwrap()).err();
        copy(self.host_path.as_path(), backup_path.as_path()).unwrap();
        remove_file(self.host_path.as_path()).unwrap();
    }
}
