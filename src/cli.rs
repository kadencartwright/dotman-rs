use clap::{Parser, Subcommand};

use crate::define_configs;

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
    CopyToVersionControl,
    /// backs up original config files, then overwrites them with symlinks from the config repo
    LinkFromVersionControl,
}
pub fn process_command() {
    let cli = Cli::parse();
    let mut mappings = define_configs();

    // map the cli enum to it's corresponding action
    match &cli.command {
        Commands::CopyToVersionControl => mappings
            .iter_mut()
            .for_each(|mapping| mapping.copy_to_version_control()),

        Commands::LinkFromVersionControl => mappings
            .iter_mut()
            .for_each(|mapping| mapping.link_from_version_control()),
    }
}
