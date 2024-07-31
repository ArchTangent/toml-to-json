//! Parsing tests for `tomltojson`.

use std::time::Duration;
use tomltojson::*;

#[test]
fn parse_modified_opt() {
    let valid_strs = ["30d", "60s", "60m", "24h"];
    let error_strs = ["24hr", "abc", "a30m", "60j"];

    let actual: Vec<Duration> = valid_strs
        .iter()
        .map(|s| parse_modified(s).unwrap())
        .collect();
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
