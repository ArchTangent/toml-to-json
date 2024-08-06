//! Conversion to JSON.

use crate::files::{open, get_filepaths};
use crate::Result;
use serde_json::Serializer;
use std::fs::{write, File};
use std::io::{BufWriter, Read};
use std::path::{Path, PathBuf};

/// Determines the formatting of exported JSON.
#[derive(Clone, Copy)]
pub enum JsonFormat {
    Normal,
    Pretty,
}

/// Transcodes TOML files in a folder to JSON files in target folder, using selected 
/// formatting and recursion depth.
///
/// Where:
/// - `fp_in` is the filepath of the source folder.
/// - `fp_out` is the filepath or the target folder.
///
/// Returns the number of files converted.
pub fn from_toml_folders(fp_in: &Path, fp_out: &Path, recursion: usize, formatting: JsonFormat) -> Result<usize> {
    // TODO: gather all source folders (in fp_in) according to recursion depth
    // TODO: for each folder, call `from_toml_folder()`
    // TODO: keep `root` (`fp_in`), adding subfolder paths under `root` to `fp_out`, ensuring folder exists
    

    // TODO: let root = `fp_in`.clone();
    // TODO: let subdirs = get_subfolders(fp_in)
    // TODO: for src_subdir in subdirs: tgt_subdir = fp_out + src_subdir.strip_prefix(fp_in)
    // TODO: from_toml_folder(src_subdir, tgt_subdir)
    todo!()
}

/// Transcodes TOML files in a folder to JSON files in target folder, using selected formatting.
///
/// Where:
/// - `fp_in` is the filepath of the source folder.
/// - `fp_out` is the filepath or the target folder.
///
/// Returns the number of files converted.
pub fn from_toml_folder(fp_in: &Path, fp_out: &Path, formatting: JsonFormat) -> Result<usize> {
    // TODO: for each TOML file in `fp_in` according to recursion, call `from_toml(toml, json)`
    // TODO: for each `parse_...()` function, take ArgMatches as an input
    // TODO: redo with parse_source(&ArgMatches) and parse_target(&ArgMatches, source)
    todo!()
}

/// Transcodes a TOML file into a JSON file using selected formatting.
///
/// Where:
/// - `fp_in` is the filepath of the source file.
/// - `fp_out` is the filepath or the target file.
///
/// Returns the number of bytes read.
pub fn from_toml(fp_in: &Path, fp_out: &Path, formatting: JsonFormat) -> Result<usize> {
    let mut buf = Vec::new();
    let bytes_read = open(fp_in)?.read_to_end(&mut buf)?;
    let toml_str = std::str::from_utf8(&buf)?;

    let deserializer = toml::de::Deserializer::new(toml_str);
    let writer = BufWriter::new(File::create(&fp_out)?);

    let result = match formatting {
        JsonFormat::Normal => {
            let mut serializer = Serializer::new(writer);
            serde_transcode::transcode(deserializer, &mut serializer)
        }
        JsonFormat::Pretty => {
            let mut serializer = Serializer::pretty(writer);
            serde_transcode::transcode(deserializer, &mut serializer)
        }
    };

    match result {
        Ok(_) => Ok(bytes_read),
        Err(e) => Err(Box::new(e)),
    }
}
