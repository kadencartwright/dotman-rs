use clap::{arg, Parser, Subcommand};

use crate::{
    file_mapping::FileMapping,
    mapping_definitions::{define_mappings, read_mappings_config_file},
    dependency_definition::DependencyDefinition
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

//all the possibilites for commands to use from the CLI
#[derive(Subcommand)]
enum Commands {
    /// adds the defined files to the config repo
    CopyToVersionControl {
        #[arg(long, short)]
        file: Option<String>,
    },
    /// backs up original config files, then overwrites them with symlinks from the config repo
    LinkFromVersionControl {
        #[arg(long, short)]
        file: Option<String>,
    },
}
pub fn process_command() {
    let cli = Cli::parse();

    // map the cli enum to it's corresponding action
    match &cli.command {
        Commands::CopyToVersionControl { file } => get_mappings(file)
            .iter_mut()
            .for_each(|mapping| mapping.copy_to_version_control()),

        Commands::LinkFromVersionControl { file } => get_mappings(file)
            .iter_mut()
            .for_each(|mapping| mapping.link_from_version_control()),
    }
}
fn get_mappings(maybe_file: &Option<String>) -> Vec<FileMapping> {
    return match maybe_file {
        Some(path) => read_mappings_config_file(path),
        None => define_mappings(),
    };
}
