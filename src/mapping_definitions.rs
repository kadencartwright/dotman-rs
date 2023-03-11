use std::{fs::read_to_string, path::PathBuf};

use serde_derive::Deserialize;

use crate::{ os_type::OSType, config_mapping::ConfigMapping};

pub fn define_mappings() -> Vec<ConfigMapping> {
    let types = Some(vec![OSType::Linux]);
    let home_dir = home::home_dir().expect("could not find home directory");
    vec![ConfigMapping::new(
        PathBuf::from("vim/.vimrc"),
        home_dir.join(".vimrc.test"),
        types,
    )]
}
#[derive(Deserialize)]
pub struct ConfigFileData {
    file_configs: Vec<ConfigMapping>,
}

pub fn read_mappings_config_file(path: &str) -> Vec<ConfigMapping> {
    let file_contents = read_to_string(path).unwrap();
    let file_data: ConfigFileData = toml::from_str(&file_contents).unwrap();
    return file_data.file_configs;
}
