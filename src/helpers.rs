use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{constants, errors::HermanErrors, types};

///
/// Helper function that 'cuts' the vector of path bufs into the new directory
///
pub fn move_files(entries: Vec<PathBuf>) {
    for path in entries {
        if let Err(error_type) = move_file(&path) {
            eprintln!(
                "Something happened while moving {:?}: {:?}",
                path.to_str(),
                error_type
            );
        }
    }
}

///
/// Helper function that 'cuts' the file into the new directory
///
pub fn move_file(path_buf: &Path) -> Result<(), HermanErrors> {
    let (from, to) = get_new_file_path(path_buf);

    if fs::copy(&from, to).is_err() {
        Err(HermanErrors::FileCopy)
    } else {
        match fs::remove_file(from) {
            Ok(_) => Ok(()),
            Err(_) => Err(HermanErrors::FileDelete),
        }
    }
}

///
/// Transforms an absolute `&Path` into a relative path to properly
/// copy data over with `std::fs`.
///
/// Returns the relative path of the file and the directory where the file will be moved  
///
pub fn get_new_file_path(path_buf: &Path) -> (String, String) {
    let name = path_buf.file_name().unwrap().to_str().unwrap();
    let parent = path_buf.parent().unwrap().to_str().unwrap();

    let relative_file_path = format!("{parent}/{name}");
    let mut new_relative_file_path = format!("{parent}/");

    match get_media_type(path_buf) {
        types::FileType::Docs => new_relative_file_path.push_str("docs/"),
        types::FileType::Media => new_relative_file_path.push_str("media/"),
        types::FileType::Programming => new_relative_file_path.push_str("programming/"),
        types::FileType::Others => new_relative_file_path.push_str("etc/"),
    }

    new_relative_file_path.push_str(name);

    (relative_file_path, new_relative_file_path)
}

///
/// Helper function that maps a `&Path` into a `FileType`
///
pub fn get_media_type(path_buf: &Path) -> types::FileType {
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
