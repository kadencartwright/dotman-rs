use std::{fs::read_to_string, path::PathBuf};

use serde_derive::Deserialize;

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
#[derive(Deserialize)]
pub struct ConfigFileData {
    mappings: Vec<FileMapping>,
}

pub fn read_mappings_config_file(path: &str) -> Vec<FileMapping> {
    let file_contents = read_to_string(path).unwrap();
    let file_data: ConfigFileData = toml::from_str(&file_contents).unwrap();
    return file_data.mappings;
}
