use clap::{Parser, Subcommand};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(name = "dotman")]
#[command(author = "Kaden Cartwright <cartwrightkaden@gmail.com>")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let args = Cli::parse();
}
