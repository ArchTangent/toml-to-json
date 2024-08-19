# TOML-to-JSON (`tomltojson`) To-Do List

## CLI

Sane defaults:

- recursion (0)
- modified (any time: always convert if `-m` not specified)

Consider:

- custom extensions: (other than `.toml` for source or `.json` for target)

Command `help`:

- file-to-file conversion _always_ converts - `--modified` does not apply

## Features

- `--list (-l)` option: take each TOML _dictionary_ of k:v pairs and convert into a _list_ of k:v pairs with an `id` field whose value is the top-level key. Example:

```toml

[foo]
key1 = 1

[bar]
key2 = 2
```

Becomes:

```json
[
    {
        "id": "foo",
        "key1": 1,
    },
    {
        "id": "bar",
        "key2": 2,
    }
]
```

## Readme File

- `--list` usage, if added
