//! Conversion to JSON.

use crate::files::open;
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

/// Transcodes TOML files in a folder to JSON files in target folder, using selected formatting.
///
/// Where:
/// - `fp_in` is the filepath of the source folder.
/// - `fp_out` is the filepath or the target folder.
///
/// Returns the number of files converted.
pub fn from_toml_folder(fp_in: &Path, fp_out: &Path, formatting: JsonFormat) -> Result<usize> {
    // TODO: for each TOML file in `fp_in` according to recursion, call `from_toml(toml, json)`
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
