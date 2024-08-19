//! Parsing tests for `tomltojson`.

use crate::*;

#[test]
fn parse_modified_value() {   
    let valid_args = ["tomltojson", "/foo"];
    let matches = cmd().get_matches_from(&valid_args);
    let actual = parse_modified(&matches).unwrap();
    let expected = None;
    assert_eq!(actual, expected);
    
    let valid_args = ["tomltojson", "/foo", "-m", "30d"];
    let matches = cmd().get_matches_from(&valid_args);
    let actual = parse_modified(&matches).unwrap();
    let expected = Some(Duration::from_secs(2592000));
    assert_eq!(actual, expected);
    
    let valid_args = ["tomltojson", "/foo", "-m", "60s"];
    let matches = cmd().get_matches_from(&valid_args);
    let actual = parse_modified(&matches).unwrap();
    let expected = Some(Duration::from_secs(60));
    assert_eq!(actual, expected);
    
    let valid_args = ["tomltojson", "/foo", "-m", "60m"];
    let matches = cmd().get_matches_from(&valid_args);
    let actual = parse_modified(&matches).unwrap();
    let expected = Some(Duration::from_secs(3600));
    assert_eq!(actual, expected);
    
    let valid_args = ["tomltojson", "/foo", "-m", "24h"];
    let matches = cmd().get_matches_from(&valid_args);
    let actual = parse_modified(&matches).unwrap();
    let expected = Some(Duration::from_secs(86400));    
    assert_eq!(actual, expected);

    let error_strs = ["24hr", "abc", "a30m", "60j"];

    for error_str in error_strs.iter() {
        let error_args = ["tomltojson", "/foo", "-m", error_str];
        let error_matches = cmd().get_matches_from(&error_args);
        assert!(parse_modified(&error_matches).is_err())
    }
}

#[test]
fn parse_general_options() {
    // automatic conversion
    let valid_args = ["tomltojson", "foo.toml"];
    let matches = cmd().get_matches_from(&valid_args);
    assert!(matches.contains_id("source"));
    
    // automatic conversion, pretty
    let valid_args = ["tomltojson", "foo.toml", "-p"];
    let matches = cmd().get_matches_from(&valid_args);
    assert!(matches.contains_id("source"));
    assert!(matches.contains_id("pretty"));

    // automatic conversion, pretty, modified
    let valid_args = ["tomltojson", "foo.toml", "-p", "-m", "20d"];
    let matches = cmd().get_matches_from(&valid_args);
    assert!(matches.contains_id("source"));
    assert!(matches.contains_id("pretty"));
    assert!(matches.contains_id("modified"));
    let since: Option<&String> = matches.get_one("modified");
    assert_eq!(since, Some(&"20d".into()));

    // SOURCE to TARGET conversion
    let valid_args = ["tomltojson", "foo.toml", "foo.json"];
    let matches = cmd().get_matches_from(&valid_args);
    assert!(matches.contains_id("source"));
    assert!(matches.contains_id("target"));
    
    // SOURCE to TARGET conversion, pretty
    let valid_args = ["tomltojson", "foo.toml", "foo.json", "-p"];
    let matches = cmd().get_matches_from(&valid_args);
    assert!(matches.contains_id("source"));
    assert!(matches.contains_id("target"));
    assert!(matches.contains_id("pretty"));

    // SOURCE to TARGET conversion, pretty, modified, recursion
    let valid_args = ["tomltojson", "folder1", "folder2", "-p", "-m", "60m", "-r", "5"];
    let matches = cmd().get_matches_from(&valid_args);
    assert!(matches.contains_id("source"));
    assert!(matches.contains_id("target"));
    assert!(matches.contains_id("pretty"));
    let since: Option<&String> = matches.get_one("modified");
    assert_eq!(since, Some(&"60m".into()));
    let depth: Option<&u8> = matches.get_one("recursion");
    assert_eq!(depth, Some(&5));

}
