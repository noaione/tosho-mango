# tosho-common

A common shared library used by tosho-* sources crates.

Not recommended to be used by other projects.

## Features
- `serde` - Enable error type when parsing with `serde` + `serde_json` (Also enable the parser for `reqwest`)
- `protobuf` - Enable error type when parsing with `protobuf` via `prost` (Also enable the parser for `reqwest`)
- `image` - Enable error type when processing image with `image-rs`
- `complete-errors` - Enable all error types described above
- `id-gen` - Enable random token/ID generator
- `all` - Enable all features

This is made since not all sources crates will need all of the features.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)
