use std::{
    borrow::BorrowMut,
    env::{self, consts::OS},
    fs::{self, copy, create_dir_all, remove_file},
    os::unix::fs::symlink,
    path::{PathBuf, Path},
    str::FromStr, io,
};

use crate::os_type::OSType;
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
        let mut path = env::current_dir().unwrap();
        path.push("testRepo");
        path.push("dotfiles");
        path.push(repo_path);
        return ConfigMapping {
            repo_path: path,
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
        let link_path = self.link_path.as_path();
        let repo_path = self.repo_path.as_path();
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
        symlink(
            self.repo_path.display().to_string(),
            self.link_path.display().to_string(),
        )
        .expect("could not link file/directory");
    }

    /// backs up a file to the backup location. mimics the dir structure relative to `/`
    pub fn backup(&self) {
        if !self.link_path.as_path().exists() {
            return;
        }
        let backup_store_path = PathBuf::from("/Users/kbc/.dotman-backup.d");
        let backup_path = PathBuf::new()
            .join(backup_store_path.as_path())
            .join(self.link_path.as_path().strip_prefix("/Users/kbc").unwrap());

        create_dir_all(backup_path.as_path().parent().unwrap()).err();
        copy(self.link_path.as_path(), backup_path.as_path()).unwrap();
        remove_file(self.link_path.as_path()).unwrap();
    }
    pub fn copy_recursively(
        source: impl AsRef<Path>,
        destination: impl AsRef<Path>,
    ) -> io::Result<()> {
        fs::create_dir_all(&destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let filetype = entry.file_type()?;
            if filetype.is_dir() {
                Self::copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }
}
