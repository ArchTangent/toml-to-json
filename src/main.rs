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
use to_json::{from_toml_folders, JsonFormat};

/// The type of filepath, if any, for `<SOURCE>` and `[TARGET]` arguments.
#[derive(Debug, Clone, Copy)]
pub enum PathType {
    File,
    Folder,
    None,
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
                .help("Input file or folder"),
        )
        // Positional Arg 2: [TARGET]
        .arg(
            Arg::new("target")
                .value_name("TARGET")
                .required(false)
                .help("Output file or folder"),
        )
        // Option 1: [--pretty -p]
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .action(ArgAction::SetTrue)
                .help("Formats JSON output in human-readable 'pretty' format"),
        )
        // Option 2: [--modified -m <SINCE>]
        .arg(
            Arg::new("modified")
                .short('m')
                .long("modified")
                .action(ArgAction::Set)
                .value_name("TIME")
                .value_parser(clap::builder::StringValueParser::new())
                .num_args(1)
                // .help("converts only files modified within the past <TIME>, e.g. `60s`, `30m`, `24h`, `10d`"),
                .help("Converts only files modified within the past <TIME>.\
                \nExamples:\n- seconds: `10s`, `20s`\n- minutes: `30m`, `60m`\n- hours: `12h`, `24h`\
                \n- days: `31d`, `365d`"),
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
                .help("Recursion depth when converting a folder of files (default 0)"),
        )
}

/// Parses `<SOURCE>` argument as a file or folder path.
///
/// Returns the path and filetype (file, folder, or None).
pub fn parse_source(matches: &ArgMatches) -> Result<(PathBuf, PathType)> {
    match matches.get_one::<&String>("source") {
        Some(fp) => {
            let src_fp = PathBuf::from(fp);

            if src_fp.is_file() {
                Ok((src_fp, PathType::File))
            } else {
                Ok((src_fp, PathType::Folder))
            }
        }
        None => Err("SOURCE is a required argument".into()),
    }
}

/// Parses `[TARGET]` argument as a file or folder path.
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
pub fn parse_target(matches: &ArgMatches, src_fp: &PathBuf, src_pt: PathType) -> Result<PathBuf> {
    let tgt_arg: Option<&String> = matches.get_one("target");

    let tgt_fp = if let Some(fp) = tgt_arg {
        // Target argument specified - ensure it's file-to-file or folder-to-folder
        let tgt_fp = PathBuf::from(fp);

        match (src_pt, tgt_fp.is_file(), tgt_fp.is_dir()) {
            (PathType::File, false, true) => {
                return Err("SOURCE / TARGET must be file-file or dir-dir!".into())
            }
            (PathType::Folder, true, false) => {
                return Err("SOURCE / TARGET must be file-file or dir-dir!".into())
            }
            _ => (),
        }
        tgt_fp
    } else {
        // No target specified - set target according to source
        let mut tgt_fp = src_fp.clone();

        if let PathType::File = src_pt {
            tgt_fp.set_extension("json");
        }
        tgt_fp
    };

    Ok(tgt_fp)
}

/// Parses time modified limit (`--modified, -m`) for comparison against a file's time modified.
///
/// Notes:
/// - values ending in `s` are 'seconds', e.g. `300s`
/// - values ending in `m` are 'minutes', e.g. `10m`
/// - values ending in `h` are 'hours', e.g. `1h`
/// - values ending in `d` are 'days', e.g. `30d`
pub fn parse_modified(matches: &ArgMatches) -> Result<Duration> {
    let m: &String = match matches.get_one("modified") {
        Some(m) => m,
        None => return Ok(Duration::MAX),
    };

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

/// Parses recursion depth. Defaults to `0` if not given.
pub fn parse_recursion(matches: &ArgMatches) -> Result<usize> {
    let depth: usize = match matches.get_one("recursion") {
        Some(d) => *d,
        None => 0,
    };

    Ok(depth)
}

/// Command line interface.
///
/// Gathers matches and returns the number of files converted..
fn cli(matches: &ArgMatches) -> Result<usize> {
    let (source, pathtype) = parse_source(matches)?;
    let target = parse_target(matches, &source, pathtype)?;
    let modified = parse_modified(matches)?;
    let recursion = parse_recursion(matches)?;
    let formatting = match matches.contains_id("pretty") {
        true => JsonFormat::Pretty,
        false => JsonFormat::Normal,
    };

    let result = match pathtype {
        PathType::File => to_json::from_toml(&source, &target, None, formatting),
        PathType::Folder => to_json::from_toml_folders(&source, &target, modified, recursion, formatting),
        PathType::None => unreachable!("SOURCE is a required argument!"),
    };

    result
}

fn main() {
    println!("--- TOML to JSON ---");

    // let c = cmd().print_help();    

    // TODO: remove when done testing
    let src = PathBuf::from(".\\data_toml");
    let tgt = PathBuf::from(".\\data_json");

    let pretty = JsonFormat::Normal;
    let modified = Duration::MAX;
    let folders = from_toml_folders(&src, &tgt, modified, 3,  pretty).unwrap();
    println!("number of files converted: {folders}");
}
