//! TOML to JSON conversion binary using Rust.

type Result<T> = core::result::Result<T, Error>;
type Error = Box<dyn std::error::Error>;

mod files;
mod to_json;

#[cfg(test)]
mod tests;

use clap::{Arg, ArgAction, ArgMatches, Command};
use std::fs::{File, FileType};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// The type of filepath, if any, for `<SOURCE>` and `[TARGET]` arguments.
#[derive(Debug, Clone, Copy)]
pub enum PathType {
    File,
    Folder,
    None,
}

/// Filepaths for `<SOURCE>` and `[TARGET]` arguments, and whether paths are for
/// file-to-file or folder-to-folder conversion.
#[derive(Debug, Clone)]
pub struct FilePaths {
    source: PathBuf,
    target: PathBuf,
    kind: PathType,
}

impl FilePaths {
    pub fn new(source: PathBuf, target: PathBuf, kind: PathType) -> Self {
        Self {
            source,
            target,
            kind,
        }
    }
}

/// Command line interface (CLI) for the program.
pub fn cmd() -> Command {
    Command::new("tomltojson")
        .author("ArchTangent")
        .version("0.5.0")
        .about("Converts TOML file(s) to JSON")
        // Positional Arg 1: <SOURCE>
        .arg(
            Arg::new("source")
                .value_name("SOURCE")
                .required(true)
                .help("input file or folder"),
        )
        // Positional Arg 2: [TARGET]
        .arg(
            Arg::new("target")
                .value_name("TARGET")
                .required(false)
                .help("output file or folder"),
        )
        // Option 1: [--pretty -p]
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .action(ArgAction::SetTrue)
                .help("formats JSON output in human-readable 'pretty' format"),
        )
        // Option 2: [--modified -m <SINCE>]
        .arg(
            Arg::new("modified")
                .short('m')
                .long("modified")
                .action(ArgAction::Set)
                .value_name("SINCE")
                .value_parser(clap::builder::StringValueParser::new())
                .num_args(1)
                .help("converts only files modified since <SINCE> ago, e.g. `10d`"),
        )
        // Option 3: [--recursion -r <DEPTH>]
        .arg(
            Arg::new("recursion")
                .short('r')
                .long("recursion")
                .action(ArgAction::Set)
                .value_name("DEPTH")
                .value_parser(clap::builder::RangedU64ValueParser::<u8>::new().range(1..256))
                .num_args(1)
                .help("recursion depth when converting a folder of files (default 0)"),
        )
}

/// Parses `<SOURCE>` and `[TARGET]` arguments as files or folders.
///
/// If `SOURCE` is a __file__ and `TARGET` is:
/// - None:   convert `SOURCE` to .json file of same name.
/// - file:   convert `SOURCE` to .json file of given path.
/// - folder: convert `SOURCE` to .json file of same name, in `TARGET` folder.
///
/// If `SOURCE` is a __folder__ and `TARGET` is:
/// - None:   convert all .toml` in `SOURCE` to .json in the `SOURCE` folder.
/// - file:   returns an error, as the target path must be a folder.
/// - folder: convert `SOURCE` to .json file of given folder.
///
pub fn parse_source_target(matches: &ArgMatches) -> Result<FilePaths> {
    let src_fp = PathBuf::from(
        matches
            .get_one::<&String>("source")
            .expect("SOURCE is a required argument"),
    );
    let tgt_arg: Option<&String> = matches.get_one("target");

    let src = if src_fp.is_file() {
        PathType::File
    } else {
        PathType::Folder
    };

    let (tgt, tgt_fp) = if let Some(fp) = tgt_arg {
        let tgt_fp = PathBuf::from(fp);

        if tgt_fp.is_dir() {
            (PathType::Folder, tgt_fp)
        } else {
            (PathType::File, tgt_fp)
        }
    } else {
        // No target specified - set according to source
        let mut tgt_fp = src_fp.clone();
        match src {
            PathType::File => {
                tgt_fp.pop();
            }
            PathType::Folder => {
                tgt_fp.set_extension("json");
            }
            PathType::None => (),
        }
        (src, tgt_fp)
    };

    match (src, tgt) {
        (PathType::File, PathType::None) => Ok(FilePaths::new(src_fp, tgt_fp, src)),
        (PathType::File, PathType::File) => Ok(FilePaths::new(src_fp, tgt_fp, src)),
        (PathType::File, PathType::Folder) => Ok(FilePaths::new(src_fp, tgt_fp, src)),
        (PathType::Folder, PathType::None) => Ok(FilePaths::new(src_fp, tgt_fp, src)),
        (PathType::Folder, PathType::Folder) => Ok(FilePaths::new(src_fp, tgt_fp, src)),
        // Errors
        (PathType::Folder, PathType::File) => {
            Err("SOURCE folder must have a folder as a TARGET".into())
        }
        _ => Err("SOURCE is a required argument".into()),
    }
}

/// Parses time modified limit (`--modified, -m`) for comparison against a file's time modified.
///
/// Notes:
/// - values ending in `s` are 'seconds', e.g. `300s`
/// - values ending in `m` are 'minutes', e.g. `10m`
/// - values ending in `h` are 'hours', e.g. `1h`
/// - values ending in `d` are 'days', e.g. `30d`
pub fn parse_modified(m: &str) -> Result<Duration> {
    if let (true, Some(days_str)) = (m.ends_with('d'), m.strip_suffix('d')) {
        if let Ok(days) = days_str.parse::<u64>() {
            return Ok(Duration::from_secs(days * 86400));
        }
    }
    if let (true, Some(hours_str)) = (m.ends_with('h'), m.strip_suffix('h')) {
        if let Ok(hours) = hours_str.parse::<u64>() {
            return Ok(Duration::from_secs(hours * 3600));
        }
    }
    if let (true, Some(minutes_str)) = (m.ends_with('m'), m.strip_suffix('m')) {
        if let Ok(minutes) = minutes_str.parse::<u64>() {
            return Ok(Duration::from_secs(minutes * 60));
        }
    }
    if let (true, Some(seconds_str)) = (m.ends_with('s'), m.strip_suffix('s')) {
        if let Ok(seconds) = seconds_str.parse::<u64>() {
            return Ok(Duration::from_secs(seconds));
        }
    }

    Err("`--modified`: invalid duration specified.".into())
}

/// Returns `true` if file is eligible for transcoding.
///
/// Eligible file was modified more recently than `--modified` time threshold.
pub fn is_transcode_eligible(file: &File, threshold: Duration) -> Result<bool> {
    let modified_time = get_time_modified(file)?;
    Ok(modified_time < threshold)
}

/// Gets the date modified for a file at given path, expressed as seconds.
pub fn get_time_modified(file: &File) -> Result<Duration> {
    let modified = file.metadata()?.modified()?;

    let elapsed = SystemTime::now().duration_since(modified)?;

    Ok(elapsed)
}

fn main() {
    println!("--- TOML to JSON ---");

    let matches = cmd().get_matches();
    for m in matches.ids() {
        println!("{m}");
    }
}
