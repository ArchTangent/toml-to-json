# TOML-to-JSON (`tomltojson`) To-Do List

## CLI

Big picture:

```text
tomltojson <SRC> [TGT] [OPTIONS]

Usage:
- SRC : source file or folder. File type is detected based on extension
- TGT : target file or folder. File type is detected based on extension

Options:
-p, --pretty             format the JSON in human-readable 'pretty' format
-f, --folder <PATH>      converts all `.toml` files in pecify expected source extension (if not default for type)
-r, --recursion <DEPTH>  for folder conversion, use specfied recursion depth (default 5)
-m, --modified <WITHIN>  convert only files modified within specified time frame from program start, e.g. `10d`, `5m`, `300s`
```

### Direct File to File conversion (.toml to .json)

Ways to use:

1. Enter the source file, creating target `.json` file in the same directory.

2. Enter the source and target file, creating JSON files as specified target filepath.

### Folder to Folder conversion (folder of .toml to folder of .json)

Ways to use:

1. Enter the source folder, creating target `.json` files in the same directory as each `.toml` file found.

2. Enter the source and target folders, creating `.json` files in target folder, preserving same directory structure as that found in source.

### Check 'last modified' time to convert only files modified within last `<TIME>`

Examples:

`-m 10d`  only modify files that have been modified within 10 days since program start
`-m 300s` only modify files that have been modified within 300s since program start

## Readme File

- Usage
