use std::{
    env::{self, consts::OS},
    fs::copy,
    os::unix::fs::symlink,
    path::PathBuf,
    str::FromStr,
};

use crate::{config_file::ConfigFile, os_type::OSType};
/// a data structure that represents the relationship between a config file on the host and its location inside the config repo
pub struct FileMapping {
    pub repo_path: PathBuf,
    pub file: ConfigFile,
    applicable_os_types: Option<Vec<OSType>>,
}

impl FileMapping {
    pub fn new(
        repo_path: PathBuf,
        file: ConfigFile,
        applicable_os_types: Option<Vec<OSType>>,
    ) -> Self {
        let mut path = env::current_dir().unwrap();
        path.push("testRepo");
        path.push("dotfiles");
        path.push(repo_path);
        return FileMapping {
            repo_path: path,
            file,
            applicable_os_types,
        };
    }
}
impl FileMapping {
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
        let host_path = self.file.host_path.as_path();
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
        let backup_path = PathBuf::from("/Users/kbc/.dotman-backup.d");
        self.file.backup(backup_path);
        symlink(
            self.repo_path.display().to_string(),
            self.file.host_path.display().to_string(),
        )
        .expect("could not link file");
    }
}
