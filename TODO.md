# TOML-to-JSON To-Do List

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
```

### Direct File to File conversion (.toml to .json)

Ways to use:

1. Enter the source file, creating target `.json` file in the same directory.

2. Enter the source and target file, creating JSON files as specified target filepath.

### Folder to Folder conversion (folder of .toml to folder of .json)

Ways to use:

1. Enter the source folder, creating target `.json` files in the same directory as each `.toml` file found.

2. Enter the source and target folders, creating `.json` files in target folder, preserving same directory structure as that found in source.

## Readme File

- Usage
