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
