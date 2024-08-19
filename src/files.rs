//! File handling functionality.

use crate::Result;
use std::fmt::Debug;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Helper function to open and return a `File` object.
pub fn open<P: AsRef<Path> + Debug>(path: P) -> Result<File> {
    let file = File::open(path)?;
    Ok(file)
}

/// Gets filepaths for files of specified `extension` in the folder path.
///
/// Can search recursively through folders according to `recursion` parameter.
pub fn get_files<P>(path: P, recursion: usize, extension: &str) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path> + Debug,
{
    let mut result: Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(&path)?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if recursion > 0 {
                    let nested = get_files(&path, recursion - 1, extension)?;
                    result.extend(nested.iter().cloned());
                }
            } else {
                // Find all files with matching extensions
                if path.extension().is_none() {
                    continue;
                }
                let ext = path.extension().unwrap().to_str().expect("UTF-8");
                if extension == ext {
                    result.push(path);
                }
            }
        }
    }

    Ok(result)
}

/// Gets filepaths for _subfolders_ in the folder path according to recursion depth.
pub fn get_subfolders<P>(path: P, recursion: usize) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path> + Debug,
{
    let mut result: Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(&path)?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_dir() {
                result.push(path.clone());

                if recursion > 0 {
                    let nested = get_subfolders(&path, recursion - 1)?;
                    result.extend(nested.iter().cloned());
                }
            }
        }
    }

    Ok(result)
}

/// Gets the date modified for a file at given path, expressed as a `Duration`.
pub fn get_time_modified(file: &File) -> Result<Duration> {
    let modified = file.metadata()?.modified()?;

    let elapsed = SystemTime::now().duration_since(modified)?;

    Ok(elapsed)
}
