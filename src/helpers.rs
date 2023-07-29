use std::{fs, io, path::PathBuf};

use crate::{constants, errors, types};

///
/// Creates the starting directories and moves existing files into
/// their respective folder
///
pub fn initialize_directory(directory: &str) -> Result<(), errors::HermanErrors> {
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

    for absolute_path_buf in entries {
        let (relative_file_path, new_file_path) = get_new_file_path(&absolute_path_buf).unwrap();

        if let Err(_) = move_file(&relative_file_path, &new_file_path) {}
    }

    Ok(())
}

///
/// Transforms an absolute `&PathBuf` into a relative path to properly
/// copy data over with `std::fs`.
///
/// Returns the relative path of the file and the directory where the file will be moved  
///
pub fn get_new_file_path(path_buf: &PathBuf) -> Result<(String, String), io::Error> {
    let name = path_buf.file_name().unwrap().to_str().unwrap();

    let relative_file_path = format!("./herman_watcher_test/{name}");
    let mut new_relative_file_path = format!("./herman_watcher_test/");

    match get_media_type(path_buf) {
        types::FileType::Docs => new_relative_file_path.push_str("docs/"),
        types::FileType::Media => new_relative_file_path.push_str("media/"),
        types::FileType::Programming => new_relative_file_path.push_str("programming/"),
        types::FileType::Others => new_relative_file_path.push_str("etc/"),
    }

    new_relative_file_path.push_str(name);

    Ok((relative_file_path, new_relative_file_path))
}

///
/// Helper function that 'cuts' the file into the new directory
///
pub fn move_file(from: &str, to: &str) -> Result<(), String> {
    if let Err(e) = fs::copy(&from, &to) {
        return Err(e.to_string());
    } else {
        match fs::remove_file(from) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

///
/// Helper function that maps a `&PathBuf` into a `FileType`
///
fn get_media_type(path_buf: &PathBuf) -> types::FileType {
    if let Some(extension) = path_buf.extension() {
        let extension = extension.to_str().unwrap();

        if constants::MEDIA_FILE_EXTENSIONS.contains(&extension) {
            return types::FileType::Media;
        }

        if constants::PROGRAMMING_FILE_EXTENSIONS.contains(&extension) {
            return types::FileType::Programming;
        }

        if constants::OFFICE_DOCUMENT_EXTENSIONS.contains(&extension) {
            return types::FileType::Docs;
        }
    }

    types::FileType::Others
}
