//! Library for `tomltojson`. Required for integration testing.

type Result<T> = core::result::Result<T, Error>;
type Error = Box<dyn std::error::Error>;

mod files;
mod to_json;

use clap::{Arg, ArgAction, ArgMatches, Command};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// The type of filepath, if any, for `<SOURCE>` and `[TARGET]` arguments.
#[derive(Debug, Clone)]
pub enum PathType {
    File(PathBuf),
    Folder(PathBuf),
    None,
}

/// Command line interface (CLI) for the program.
pub fn cmd() -> Command {
    Command::new("tomltojson")
        .author("ArchTangent")
        .version("0.4.0")
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
