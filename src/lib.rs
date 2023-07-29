use errors::HermanErrors;
use notify::{event::CreateKind, Event, EventKind, FsEventWatcher, RecursiveMode, Watcher};
use std::{
    fs,
    path::{Path, PathBuf},
};

mod constants;
mod errors;
pub mod helpers;
mod types;

/// Watches a directory where herman will rearrange the files into sub-directories.
/// The sub-directories that will be created can be found at `constants::INITIAL_DIRECTORIES`
pub fn watch_directory(directory: &str) -> notify::Result<FsEventWatcher> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res: Result<Event, notify::Error>| match res {
        Ok(event) => {
            // println!("event: {:?}", event);

            match event.kind {
                EventKind::Create(CreateKind::File) => {
                    // There is a chance where there are 2 paths, the reason for this is yet to be investigated
                    // but we'll not handle that use case for now
                    if event.paths.len() != 1 {
                        return;
                    }

                    let added_file_buf: &PathBuf = &event.paths[0];

                    if let Err(_) = helpers::move_file(added_file_buf) {
                        // FIXME:- Something with the watcher still marks events after the file has been moved
                        // Think about implementing polling or filtering events more properly to prevent this from happening.

                        // eprintln!(
                        //     "Something happened while moving {:?}: {:?}",
                        //     added_file_buf.to_str(),
                        //     error_type
                        // );
                    }
                }
                _ => {}
            }
        }
        Err(e) => println!("Notify watcher error: {e}"),
    })?;

    watcher.watch(Path::new(directory), RecursiveMode::NonRecursive)?;

    Ok(watcher)
}

/// Creates the initial directories
///
/// # Errors
/// If the supplied directory does not exist
///
pub fn initialize_directory(directory: &str) -> Result<Vec<PathBuf>, HermanErrors> {
    let mut entries: Vec<PathBuf> = fs::read_dir(directory)
        .map_err(|_| errors::HermanErrors::DirectoryReadError)?
        .map(|res| res.map(|e| e.path()).unwrap_or(PathBuf::new()))
        .filter(|path| !path.is_dir() || !path.exists())
        .collect();

    entries.sort();

    for nested_directory in constants::INITIAL_DIRECTORIES {
        let path = format!("{directory}/{nested_directory}");
        print!("Initializing {path}......");

        if let Err(_) = fs::create_dir(&path) {
            print!("DIRECTORY EXISTS! Skipping...\n");
        } else {
            print!("INITIALIZED!\n");
        }
    }

    Ok(entries)
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
