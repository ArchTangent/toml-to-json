# TOML-to-JSON (`tomltojson`) To-Do List

## CLI

Big picture:

```text
tomltojson <SOURCE> [TARGET] [OPTIONS]

Usage:
- SOURCE : source file or folder. File type is detected based on extension
- TARGET : target file or folder. File type is detected based on extension

Options:
-p, --pretty             formats JSON output in human-readable 'pretty' format
-m, --modified <SINCE>   convert only files modified since <SINCE> ago, e.g. `10d`, `5m`, `300s` from program start
-r, --recursion <DEPTH>  for folder conversion, use specfied recursion depth (default 0)
```

Sane defaults:

- recursion (0-5 is fine)
- modified (any time: always convert if `-m` not specified)

Consider:

- custom extensions: (other than `.toml` for source or `.json` for target)

### Recursion (folder to folder)

- for starters, use `from_toml_folder()` to convert files in `source` folder to files in existing `target` folder
- later, , use `from_toml_folders()` with _recursion_ to get _subfolders_, preserving subdirectory structure under `target`

### Check 'last modified' time to convert only files modified within last `<TIME>`

Examples:

`-m 10d`  only modify files that have been modified within 10 days since program start
`-m 300s` only modify files that have been modified within 300s since program start

## Command Handling Table

|source |target |result |
|-------|-------|-------|
|none   |none   |`Error`|
|file   |none   |convert `SOURCE.toml` to `TARGET.json`|
|file   |file   |convert `SOURCE.toml` to `TARGET.json`|
|folder |none   |convert all `.toml` in `SOURCE` folder to `.json` in the same folder according to recursion `DEPTH` |
|folder |folder |convert all `.toml` in `SOURCE` folder to `.json` in `TARGET` folder according to recursion `DEPTH`, preserving directory structure |

## Readme File

- Usage
