use std::thread;

use clap::{Parser, Subcommand};
use herman::{helpers, initialize_directory, watch_directory};

/// A rusty daemon that watches and rearranges the files
#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Watches the specified directory for changes
    Watch { dir: String },

    /// Cleans the specified directory
    Clean { dir: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Clean { dir } => match initialize_directory(dir) {
            Ok(entries) => helpers::move_files(entries),
            Err(_) => {
                eprintln!("Path does not exist or we lack permissions to modify the directory")
            }
        },
        Commands::Watch { dir } => match watch_directory(dir) {
            Ok(_) => {
                println!("Watching {} for changes...", dir);
                thread::park();
            }
            Err(e) => eprintln!("Something happened while watching {}: {e}", &dir),
        },
    }
}
