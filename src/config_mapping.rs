use std::{
    env::{self, consts::OS},
    fs::{copy, create_dir_all, remove_file},
    os::unix::fs::symlink,
    path::PathBuf,
    str::FromStr,
};

use crate::os_type::OSType;
use serde_derive::Deserialize;
/// a data structure that represents the relationship between a config file on the host and its location inside the config repo
#[derive(Deserialize)]
pub struct ConfigMapping {
    pub repo_path: PathBuf,
    pub host_path: PathBuf,
    applicable_os_types: Option<Vec<OSType>>,
}

impl ConfigMapping {
    pub fn new(
        repo_path: PathBuf,
        host_path: PathBuf,
        applicable_os_types: Option<Vec<OSType>>,
    ) -> Self {
        let mut path = env::current_dir().unwrap();
        path.push("testRepo");
        path.push("dotfiles");
        path.push(repo_path);
        return ConfigMapping {
            repo_path: path,
            host_path,
            applicable_os_types,
        };
    }
}
impl ConfigMapping {
    /// A helper function. returns true if the file mapping should be acted upon given the current operating system
    fn applies_to_current_os(&mut self) -> bool {
        let os_type = OSType::from_str(OS).unwrap();

        // if there are no applicable types, default to applying for all os's
        if self.applicable_os_types.is_none() {
            return true;
        }
        if self
            .applicable_os_types
            .as_mut()
            .unwrap()
            .contains(&os_type)
        {
            return true;
        };

        return false;
    }
    pub fn copy_to_version_control(&mut self) {
        if !self.applies_to_current_os() {
            return;
        }
        // get the paths for the original file and its location in the repo
        let host_path = self.host_path.as_path();
        let repo_path = self.repo_path.as_path();
        if !host_path.is_file() {
            println!("file {} does not exist. skipping...", host_path.display());
            return;
        }
        // copy the file from the host into version control
        copy(host_path, repo_path).unwrap();
    }
    pub fn link_from_version_control(&mut self) {
        if !self.applies_to_current_os() {
            return;
        }
        self.backup();
        symlink(
            self.repo_path.display().to_string(),
            self.host_path.display().to_string(),
        )
        .expect("could not link file/directory");
    }

    /// backs up a file to the backup location. mimics the dir structure relative to `/`
    pub fn backup(&self) {
        if !self.host_path.as_path().exists() {
            return;
        }
        let backup_store_path = PathBuf::from("/Users/kbc/.dotman-backup.d");
        let backup_path = PathBuf::new()
            .join(backup_store_path.as_path())
            .join(self.host_path.as_path().strip_prefix("/Users/kbc").unwrap());

        create_dir_all(backup_path.as_path().parent().unwrap()).err();
        copy(self.host_path.as_path(), backup_path.as_path()).unwrap();
        remove_file(self.host_path.as_path()).unwrap();
    }
}
