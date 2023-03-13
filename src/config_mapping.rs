use std::{
    env::{self, consts::OS},
    fs::{self, copy, create_dir_all, remove_dir, remove_file},
    io,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::os_type::OSType;
use home::home_dir;
use serde_derive::Deserialize;
/// a data structure that represents the relationship between a config file on the host and its location inside the config repo
#[derive(Deserialize)]
pub struct ConfigMapping {
    pub repo_path: PathBuf,
    pub link_path: PathBuf,
    applicable_os_types: Option<Vec<OSType>>,
}

impl ConfigMapping {
    pub fn new(
        repo_path: PathBuf,
        link_path: PathBuf,
        applicable_os_types: Option<Vec<OSType>>,
    ) -> Self {
        return ConfigMapping {
            repo_path,
            link_path,
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
        let link_path = self.get_prefixed_link_path();
        let repo_path = self.get_prefixed_repo_path();
        if !(self.link_path.try_exists().is_ok()) {
            println!(
                "file/dir {} does not exist. skipping...",
                link_path.display()
            );
            return;
        }
        // copy the file from the host into version control
        Self::copy_recursively(link_path, repo_path).unwrap();
    }
    pub fn link_from_version_control(&mut self) {
        if !self.applies_to_current_os() {
            return;
        }
        self.backup();

        let link_path = self.get_prefixed_link_path();
        println!(
            "linking {} -> {}",
            self.get_prefixed_repo_path().display(),
            link_path.display()
        );
        if link_path.exists() {
            fs::remove_dir_all(&link_path).unwrap();
        }
        let mut chars: Vec<char> = link_path.to_str().unwrap().chars().collect();
        if chars.ends_with(&['/']) {
            chars.pop();
        }
        let link_destination: String = chars.iter().collect();
        symlink(self.get_prefixed_repo_path(), link_destination)
            .expect("could not link file/directory");
    }

    /// backs up a file to the backup location. mimics the dir structure relative to `/`
    pub fn backup(&self) {
        let link_path = self.get_prefixed_link_path();
        if !link_path.exists() {
            return;
        }
        let mut backup_store_path = PathBuf::from(home_dir().unwrap());
        backup_store_path.push("dotman_backup.d");
        let backup_path = PathBuf::new()
            .join(backup_store_path.as_path())
            .join(self.link_path.to_owned());

        create_dir_all(&backup_path).err();
        Self::copy_recursively(self.get_prefixed_link_path(), backup_path).unwrap();
        Self::remove_fs_item(self.get_prefixed_link_path()).unwrap();
    }
    pub fn remove_fs_item(path: PathBuf) -> Result<(), std::io::Error> {
        if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            remove_file(path)
        }
    }
    pub fn copy_recursively(
        source: impl AsRef<Path>,
        destination: impl AsRef<Path>,
    ) -> io::Result<()> {
        fs::create_dir_all(&destination).unwrap();
        for entry in fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let filetype = entry.file_type().unwrap();
            if filetype.is_dir() {
                Self::copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))
                    .unwrap();
            } else {
                fs::copy(entry.path(), destination.as_ref().join(entry.file_name())).unwrap();
            }
        }
        Ok(())
    }
    pub fn get_prefixed_repo_path(&mut self) -> PathBuf {
        let mut prefixed_repo_path = env::current_dir().unwrap();
        prefixed_repo_path.push("testRepo");
        prefixed_repo_path.push("dotfiles");
        prefixed_repo_path.push(self.repo_path.to_owned());
        prefixed_repo_path
    }
    pub fn get_prefixed_link_path(&self) -> PathBuf {
        let mut prefixed_link_path = home_dir().unwrap();
        prefixed_link_path.push(self.link_path.to_owned());
        prefixed_link_path
    }
}
