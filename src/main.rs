use std::thread;

use clap::Parser;
use herman::{helpers, initialize_directory, watch_directory};

/// A rusty daemon that watches and rearranges the files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// The name of the directory to watch
    #[arg(short, long)]
    directory: String,

    /// Specifies if we should clean the folder during startup
    #[arg(short, long, default_value = "false")]
    clean_on_startup: bool,
}

fn main() {
    let args = Args::parse();

    if args.clean_on_startup {
        match initialize_directory(args.directory.as_str()) {
            Ok(entries) => helpers::move_files(entries),
            Err(_) => {
                eprintln!("Path does not exist or we lack permissions to modify the directory")
            }
        };
    }

    match watch_directory(&args.directory) {
        Ok(_) => {
            println!("Watching {} for changes...", &args.directory);
            thread::park();
        }
        Err(e) => eprintln!("Something happened while watching {}: {e}", &args.directory),
    };
}

#[cfg(test)]
mod test {
    use std::{fs, path::Path, thread, time::Duration};

    use super::*;

    #[test]
    fn start_script_arranges_files() {
        initialize_test_folder(TEST_DIR_PATH);
        initialize_test_files(TEST_DIR_PATH);

        let res = initialize_directory(TEST_DIR_PATH);
        assert!(res.is_ok());

        let directories = res.unwrap();
        helpers::move_files(directories);

        assert!(Path::new("./test/docs/test_file.docx").exists());
        assert!(Path::new("./test/programming/test_file.rs").exists());
        assert!(Path::new("./test/media/test_file.jpg").exists());
        assert!(Path::new("./test/etc/test_file").exists());

        clean_dir(TEST_DIR_PATH);
    }

    #[test]
    fn watcher_arranges_files() {
        initialize_test_folder(TEST_WATCHER_DIR_PATH);

        let res = initialize_directory(TEST_WATCHER_DIR_PATH);
        assert!(res.is_ok());

        if let Ok(_) = watch_directory(TEST_WATCHER_DIR_PATH) {
            println!("Watching {TEST_WATCHER_DIR_PATH} for changes...");
            thread::sleep(Duration::from_secs(1));

            initialize_test_files(TEST_WATCHER_DIR_PATH);
            thread::sleep(Duration::from_secs(2));

            assert!(Path::new("./test_watcher/docs/test_file.docx").exists());
            assert!(Path::new("./test_watcher/programming/test_file.rs").exists());
            assert!(Path::new("./test_watcher/media/test_file.jpg").exists());
            assert!(Path::new("./test_watcher/etc/test_file").exists());

            clean_dir(TEST_WATCHER_DIR_PATH);
        }
    }

    ///
    /// Test helper functions
    ///

    const TEST_DIR_PATH: &str = "./test";
    const TEST_WATCHER_DIR_PATH: &str = "./test_watcher";

    fn initialize_test_folder(dir: &str) {
        if let Err(_) = fs::remove_dir_all(dir) {}

        if !Path::new(dir).exists() {
            let res = fs::create_dir(dir);
            assert!(res.is_ok());
        }
    }

    fn initialize_test_files(dir: &str) {
        let test_files = [
            "test_file.rs",
            "test_file.jpg",
            "test_file.docx",
            "test_file",
        ];

        for file_name in test_files {
            let file_path = format!("{dir}/{file_name}");
            fs::write(&file_path, "").expect("Could not create {file_path}");
            assert!(Path::new(&file_path).exists());
        }
    }

    fn clean_dir(dir: &str) {
        if let Err(_) = fs::remove_dir_all(dir) {}
    }
}
