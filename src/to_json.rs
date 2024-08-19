//! Conversion to JSON.

use crate::files::{get_files, get_subfolders, get_time_modified, open};
use crate::Result;
use serde_json::Serializer;
use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::{Path, PathBuf};
use std::time::Duration;

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
/// - `source` is the filepath of the source folder.
/// - `target` is the filepath or the target folder.
///
/// Returns the number of files converted.
///
/// For some operating systems, file writing will ___panic___ if the parent directory
/// does not already exist.
pub fn from_toml_folders(
    source: &Path,
    target: &Path,
    modified: Option<Duration>,
    recursion: usize,
    formatting: JsonFormat,
) -> Result<usize> {
    let mut num_files = 0;

    num_files += from_toml_folder(&source, &target, modified, formatting)?;

    for src_subdir in get_subfolders(source, recursion)?.iter() {
        let stripped_subdir = src_subdir.strip_prefix(source)?;
        let mut tgt_subdir = PathBuf::from(target);
        tgt_subdir.push(stripped_subdir);

        num_files += from_toml_folder(&src_subdir, &tgt_subdir, modified, formatting)?;
    }

    Ok(num_files)
}

/// Transcodes TOML files in a folder to JSON files in target folder, using selected formatting.
///
/// Where:
/// - `source` is the filepath of the source folder.
/// - `target` is the filepath or the target folder.
///
/// Returns the number of files converted.
pub fn from_toml_folder(
    source: &Path,
    target: &Path,
    modified: Option<Duration>,
    formatting: JsonFormat,
) -> Result<usize> {
    let mut num_files = 0;

    for fp_in in get_files(source, 0, "toml")?.iter() {
        let mut fp_out = PathBuf::from(target);
        let file_out = fp_in.file_name().expect("expected a file");
        fp_out.push(file_out);
        fp_out.set_extension("json");
        num_files += from_toml(fp_in, &fp_out, modified, formatting)?;
    }

    Ok(num_files)
}

/// Transcodes a TOML file into a JSON file using selected formatting.
///
/// Where:
/// - `fp_in` is the filepath of the source file.
/// - `fp_out` is the filepath or the target file.
///
/// Returns the number of files converted. This is `1` if the input file was
/// modified sooner than the `modified` threshold, and `0` otherwise.
pub fn from_toml(
    fp_in: &Path,
    fp_out: &Path,
    modified: Option<Duration>,
    formatting: JsonFormat,
) -> Result<usize> {
    let mut file = open(fp_in)?;

    if let Some(threshold) = modified {
        if threshold < get_time_modified(&file)? {
            println!("Ignoring file \"{}\" ...", fp_in.display());
            return Ok(0);
        }
    }

    println!("Converting file \"{}\" ...", fp_in.display());

    let mut buf = Vec::new();
    let _bytes_read = file.read_to_end(&mut buf)?;

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
        Ok(_) => Ok(1),
        Err(e) => Err(e.into()),
    }
}
