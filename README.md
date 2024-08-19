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

### Rules

1. If `SOURCE` is a files and `TARGET` is set, `TARGET` must be a file. This is a _file-to-file_ conversion.

2. If `SOURCE` is a folder and `TARGET` is set, `TARGET` must be a folder. This is a _folder-to-folder_ conversion.

3. For _folder-to-folder_ conversion where `--recursion > 0`, any subdirectory in `SOURCE` must have of subdirectory of _the same name_ in `TARGET`.

4. The `--modified` option applies only to _folder-to-folder_ conversions.

Given the following directory structure:
```
foo/
  dir_a/
    file_a.toml
    dir_c/
      file_c.toml
  dir_b/    
    file_b.toml

bar/
  dir_a/
  dir_b/
```

This case is __valid__ as `SOURCE` and `TARGET` have the same subdirectories `dir_a` and `dir_b`:
```
tomltojson foo bar -r 1
```

This case is __invalid__ as `TARGET` is missing `dir_a/dir_c`, which is present under `SOURCE`:
```
tomltojson foo bar -r 2
```
