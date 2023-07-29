use notify::{event::CreateKind, Event, EventKind, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};

pub mod constants;
pub mod errors;
pub mod helpers;
pub mod types;

/// Watches a directory where herman will rearrange the files into sub-directories.
/// The sub-directories that will be created can be found at `constants::INITIAL_DIRECTORIES`
pub fn watch_directory(directory: &str) -> notify::Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res: Result<Event, notify::Error>| match res {
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
                        helpers::get_new_file_path(added_file_buf).unwrap();

                    if let Err(err) = helpers::move_file(&relative_file_path, &new_file_path) {
                        eprintln!("{err}");
                    }
                }
                _ => {}
            }
        }
        Err(e) => println!("Notify watcher error: {e}"),
    })?;

    watcher.watch(Path::new(directory), RecursiveMode::NonRecursive)?;
    Ok(())
}
