//! TOML to JSON conversion binary using Rust.

type Result<T> = core::result::Result<T, Error>;
type Error = Box<dyn std::error::Error>;

mod files;
mod to_json;

use std::fs::File;
use std::time::{Duration, SystemTime};

/// Parses time modified limit (`--modified, -m`) for comparison against a file's time modified.
///
/// Notes:
/// - values ending in `s` are 'seconds', e.g. `300s`
/// - values ending in `m` are 'minutes', e.g. `10m`
/// - values ending in `h` are 'hours', e.g. `1h`
/// - values ending in `d` are 'days', e.g. `30d`
fn parse_modified(m: &str) -> Result<Duration> {
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
fn is_transcode_eligible(file: &File, threshold: Duration) -> Result<bool> {
    let modified_time = get_time_modified(file)?;
    Ok(modified_time < threshold)
}

/// Gets the date modified for a file at given path, expressed as seconds.
fn get_time_modified(file: &File) -> Result<Duration> {
    let modified = file
        .metadata()?
        .modified()?;

    let elapsed = SystemTime::now().duration_since(modified)?;

    Ok(elapsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_modified_opt() {
        let valid_strs = ["30d", "60s", "60m", "24h"];
        let error_strs = ["24hr", "abc", "a30m", "60j"];

        let actual: Vec<Duration> = valid_strs.iter().map(|s| parse_modified(s).unwrap()).collect();
        let expected = vec![
            Duration::from_secs(2592000),
            Duration::from_secs(60),
            Duration::from_secs(3600),
            Duration::from_secs(86400),
        ];

        assert_eq!(actual, expected);

        for error_str in error_strs.iter() {
            assert!(parse_modified(error_str).is_err())
        }
    }
}


fn main() {
    println!("--- TOML to JSON ---");
}
