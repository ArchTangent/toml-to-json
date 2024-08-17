# TOML-to-JSON (`tomltojson`)

Converts TOML files to JSON, with optional pretty printing and batch conversion.

## Usage

```text
tomltojson <SOURCE> [TARGET] [OPTIONS]

Arguments:
- SOURCE : source file or folder
- TARGET : target file or folder

Options:
-p, --pretty             formats JSON output in human-readable 'pretty' format
-m, --modified <SINCE>   convert only files modified since <SINCE> ago, e.g. `10d`, `5m`, `300s` from program start
-r, --recursion <DEPTH>  for folder-to-folder conversion, use specfied recursion depth (default 0)
```

_Note_: when `SOURCE` is a folder and `TARGET` is set `TARGET` must be a folder. If `--recursion > 0`, any subdirectory in `SOURCE` must have of subdirectory of _the same name_ in `TARGET`.
