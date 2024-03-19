# tosho

![crates.io version](https://img.shields.io/crates/v/tosho) ![CI](https://github.com/noaione/tosho-mango/actions/workflows/ci.yml/badge.svg)

[`tosho-mango`](tosho) (or `tosho`) is a downloader but can also be considered an almost full-blown replacement for the app/web version, with the exception of currency purchase, as a simple CLI application.

Currently we support the following source:
- [MU! by SQ](https://crates.io/crates/tosho-musq)
- [KM by KC](https://crates.io/crates/tosho-kmkc)
- [AM by AP](https://crates.io/crates/tosho-amap)
- [SJ/M by V](https://crates.io/crates/tosho-sjv)
- [小豆 (Red Bean) by KRKR](https://crates.io/crates/tosho-rbean)

## Installation

You can install by cloning the repository then building manually...

Or...

```bash
cargo install --locked tosho
```

Or, if you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)...

```bash
cargo binstall --locked tosho
```

## Usage

Refer to the [repo](tosho) on how to authenticate with each source.<br />
For a list of available commands, use the `--help` argument.

[![asciicast](https://asciinema.org/a/636303.svg)](https://asciinema.org/a/636303)

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter on each platform with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

[tosho]: https://github.com/noaione/tosho-mango