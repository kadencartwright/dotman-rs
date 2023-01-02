mod cli;

mod file_mapping;
mod os_type;

use file_mapping::FileMapping;
use os_type::OSType;
use std::{path::PathBuf, vec};

fn main() {
    cli::process_command();
}

fn define_configs() -> Vec<FileMapping> {
    let types = Some(vec![OSType::Linux]);
    let home_dir = home::home_dir().expect("could not find home directory");
    vec![FileMapping::new(
        PathBuf::from("vim/.vimrc"),
        home_dir.join(".vimrc.test"),
        types,
    )]
}
