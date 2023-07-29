use clap::Parser;
use herman::{helpers, watch_directory};

/// A rusty daemon that watches and rearranges the files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// The name of the directory to watch
    #[arg(short, long)]
    directory: String,
}

fn main() {
    let args = Args::parse();

    match helpers::initialize_directory(args.directory.as_str()) {
        Ok(_) => println!("Initialzed Directories!"),
        Err(_) => {
            eprintln!("Path does not exist or we lack permissions to modify the directory")
        }
    };

    match watch_directory(&args.directory) {
        Ok(_) => println!("Watching {} for changes...", &args.directory),
        Err(e) => eprintln!("Something happened while watching {}: {e}", &args.directory),
    };

    loop {}
}

#[cfg(test)]
mod integration_tests {
    use std::{fs, path::Path};

    use super::*;

    fn initialize_directory_test_directory() {
        let dirname = "./integration_tests";

        if !Path::new(dirname).exists() {
            let res = fs::create_dir(dirname);
            assert!(res.is_ok());
        }
    }

    #[test]
    fn start() {
        initialize_directory_test_directory();
        helpers::initialize_directory("./integration_tests").unwrap();

        assert!(Path::new("./integration_tests/docs").is_dir());
        assert!(Path::new("./integration_tests/programming").is_dir());
        assert!(Path::new("./integration_tests/media").is_dir());
    }
}
