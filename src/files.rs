//! File handling functionality.

use std::fmt::{Debug, Display};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use crate::Result;

/// Helper function to open and return a `File` object.
pub fn open<P: AsRef<Path> + Debug>(path: P) -> Result<File> {
    let file = File::open(path)?;
    Ok(file)
}

/// Gets filepaths for files of specified `extension` in the folder path.
///
/// Can search recursively through folders according to `recursion` parameter.
pub fn get_filepaths<P>(
    path: P,
    recursion: usize,
    extension: &str,
) -> Result<Vec<PathBuf>>
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
                    let nested = get_filepaths(&path, recursion - 1, extension)?;
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
pub fn get_subfolders<P>(
    path: P,
    recursion: usize,
) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path> + Debug,
{
    let mut result: Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(&path)?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            println!("[get folders by name] path found: {:?}", path);

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
