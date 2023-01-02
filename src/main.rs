mod cli;
mod config_file;
mod file_mapping;
mod os_type;

use config_file::ConfigFile;
use file_mapping::FileMapping;
use os_type::OSType;
use std::{path::PathBuf, vec};

fn main() {
    cli::process_command();
}

fn define_configs() -> Vec<FileMapping> {
    let types = Some(vec![OSType::Linux]);
    let home_dir = home::home_dir().expect("could not find home directory");

    let mut test_path = home_dir.clone();
    test_path.push(".vimrc.test");
    let test_config = ConfigFile::new(test_path);

    vec![FileMapping::new(
        PathBuf::from("vim/.vimrc"),
        test_config,
        types,
    )]
}
