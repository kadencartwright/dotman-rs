use std::{
    env::{self, consts::OS},
    os::unix::fs::symlink,
    path::PathBuf,
    str::FromStr,
};

use crate::{config_file::ConfigFile, os_type::OSType};

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
    pub fn applies_to_current_os(&mut self, os_type: OSType) -> bool {
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
}
impl FileMapping {
    pub fn link_to_version_control(&mut self) {
        let os_type = OSType::from_str(OS).unwrap();
        if !self.applies_to_current_os(os_type) {
            return;
        }
        let backup_path = PathBuf::from("/Users/kbc/.dotman-backup.d");
        self.file.backup(backup_path);
        symlink(
            self.repo_path.display().to_string(),
            self.file.path.display().to_string(),
        )
        .expect("could not link file");
    }
}
