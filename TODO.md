# TOML-to-JSON (`tomltojson`) To-Do List

## CLI

Sane defaults:

- recursion (0)
- modified (any time: always convert if `-m` not specified)

Consider:

- custom extensions: (other than `.toml` for source or `.json` for target)

### Recursion (folder to folder)

- for starters, use `from_toml_folder()` to convert files in `source` folder to files in existing `target` folder
- later, , use `from_toml_folders()` with _recursion_ to get _subfolders_, preserving subdirectory structure under `target`

When using the `--nested` option on some OSes, the program will crash if a given nested `target` subfolder does not already exist. See `std::fs::write()` for more.

## Readme File

Usage
- file to file
- folder to folder
- `--nested` set and unset for folder-to-folder
