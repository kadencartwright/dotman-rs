use std::path::PathBuf;

use crate::{file_mapping::FileMapping, os_type::OSType};

pub fn define_mappings() -> Vec<FileMapping> {
    let types = Some(vec![OSType::Linux]);
    let home_dir = home::home_dir().expect("could not find home directory");
    vec![FileMapping::new(
        PathBuf::from("vim/.vimrc"),
        home_dir.join(".vimrc.test"),
        types,
    )]
}
