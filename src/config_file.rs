use std::{
    fs::{copy, create_dir_all, remove_file},
    path::PathBuf,
};

pub struct ConfigFile {
    pub path: PathBuf,
}

impl ConfigFile {
    pub fn new(path: PathBuf) -> Self {
        return ConfigFile { path };
    }
    pub fn backup(&self, backup_store_path: PathBuf) {
        println!("exists: {}", self.path.as_path().exists());
        println!("{}", self.path.display());
        if !self.path.as_path().exists() {
            return;
        }
        let backup_path = PathBuf::new()
            .join(backup_store_path.as_path())
            .join(self.path.as_path().strip_prefix("/Users/kbc").unwrap());
        println!("self path: {}", self.path.as_path().display());
        println!("backup path: {}", backup_path.as_path().display());
        create_dir_all(backup_path.as_path().parent().unwrap()).err();
        copy(self.path.as_path(), backup_path.as_path()).unwrap();
        remove_file(self.path.as_path()).unwrap();
    }
}
