use notify::{event::CreateKind, Error, Event, EventKind, RecursiveMode, Watcher};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// TODO LIST
/// 1. Make commits
/// 2. Create utility enum and function
/// 3. Tie in together the watcher and initialization function
/// 4. Install clap to watch specific directory

// Turn this into a configuration file that allows you to take specific extensions to throw inside a specific folder
const MEDIA_FILE_EXTENSIONS: [&str; 21] = [
    "mp3", "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "wav", "ogg", "aac", "jpg", "jpeg",
    "png", "gif", "bmp", "tif", "tiff", "webp", "svg", "ico",
];

const PROGRAMMING_FILE_EXTENSIONS: [&str; 27] = [
    "c", "cpp", "cc", "java", "py", "js", "php", "rb", "swift", "go", "cs", "html", "css", "xml",
    "json", "yaml", "yml", "sh", "bat", "sql", "ini", "cfg", "conf", "txt", "csv", "tsv", "rs",
];

const OFFICE_DOCUMENT_EXTENSIONS: [&str; 7] = ["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx"];

const INITIAL_DIRECTORIES: [&str; 3] = ["media", "programming", "docs"];

enum FileType {
    Docs,
    Media,
    Programming,
    Others,
}

fn main() -> notify::Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res: Result<Event, Error>| match res {
        Ok(event) => {
            // println!("event: {:?}", event);

            // Save to a text file the error logs
            match event.kind {
                EventKind::Create(CreateKind::File) => {
                    // There is a chance where there are 2 paths, the reason for this is yet to be investigated
                    // but we'll not handle that use case for now
                    if event.paths.len() != 1 {
                        return;
                    }

                    let added_file_buf: &PathBuf = &event.paths[0];

                    let (relative_file_path, new_file_path) =
                        get_new_file_path(added_file_buf).unwrap();

                    organize_file(&relative_file_path, &new_file_path).unwrap();
                }
                _ => {}
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    watcher.watch(
        Path::new("herman_watcher_test/"),
        RecursiveMode::NonRecursive,
    )?;

    initialize_directory()?;

    loop {}
}

///
/// Creates the starting directories and moves the currently exisiting files into
/// their respective folder
///
fn initialize_directory() -> Result<(), io::Error> {
    println!("Initializing Directories");

    let mut entries: Vec<PathBuf> = fs::read_dir("herman_watcher_test")?
        .map(|res| res.map(|e| e.path()).unwrap_or(PathBuf::new()))
        .filter(|path| !path.is_dir() || !path.exists())
        .collect();

    entries.sort();

    // println!("{:?}", entries);

    for directory in INITIAL_DIRECTORIES {
        let path = format!("herman_watcher_test/{directory}");
        match fs::create_dir(&path) {
            Ok(_) => println!("{path} is created!"),
            Err(_) => continue,
        }
    }

    for absolute_path_buf in entries {
        let (relative_file_path, new_file_path) = get_new_file_path(&absolute_path_buf).unwrap();

        organize_file(&relative_file_path, &new_file_path).unwrap();
    }

    Ok(())
}

fn organize_file(from: &str, to: &str) -> Result<(), String> {
    if let Ok(_) = fs::copy(&from, &to) {
        match fs::remove_file(from) {
            Ok(_) => Ok(()),
            Err(_) => Err("Map this and change this to a custom error".to_string()),
        }
    } else {
        eprintln!("An error occurred while moving {from} to {to}");
        return Err("Change this to a custom enum".to_string());
    }
}

///
/// Helper function that converts a `&PathBuf` into a `FileType`
///
fn get_media_type(path_buf: &PathBuf) -> FileType {
    if let Some(extension) = path_buf.extension() {
        let extension = extension.to_str().unwrap();

        if MEDIA_FILE_EXTENSIONS.contains(&extension) {
            return FileType::Media;
        }

        if PROGRAMMING_FILE_EXTENSIONS.contains(&extension) {
            return FileType::Programming;
        }

        if OFFICE_DOCUMENT_EXTENSIONS.contains(&extension) {
            return FileType::Docs;
        }
    }

    FileType::Others
}

///
/// Transforms an absolute `&PathBuf` into a relative path to properly
/// copy data over with `std::fs`.
///
/// Returns the relative path of the original file and the location to be moved
///
fn get_new_file_path(path_buf: &PathBuf) -> Result<(String, String), io::Error> {
    let name = path_buf.file_name().unwrap().to_str().unwrap();

    let relative_file_path = format!("./herman_watcher_test/{name}");
    let mut new_relative_file_path = format!("./herman_watcher_test/");

    match get_media_type(path_buf) {
        FileType::Docs => new_relative_file_path.push_str("docs/"),
        FileType::Media => new_relative_file_path.push_str("media/"),
        FileType::Programming => new_relative_file_path.push_str("programming/"),
        FileType::Others => new_relative_file_path.push_str("etc/"),
    }

    new_relative_file_path.push_str(name);

    Ok((relative_file_path, new_relative_file_path))
}
