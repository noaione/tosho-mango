## The Rust™ Rewrite™

There is an [ongoing process](https://github.com/noaione/tosho-mango/pull/5) of me rewritting everything into Rust because it's fun \:)

So, there will be no more update to the Python code.<br />
The Python code will be hosted on separate branch here: [`legacy-snek`](https://github.com/noaione/tosho-mango/tree/legacy-snek).

The new Rust version will also introduce some more capabilities like actual login system for some sources. (Legacy "login" will also still be supported because honestly it's way easier to implement and use)

# tosho-mango

[![CI](https://github.com/noaione/tosho-mango/actions/workflows/ci.yml/badge.svg)](https://github.com/noaione/tosho-mango/actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/github/license/noaione/tosho-mango)](https://github.com/noaione/tosho-mango/blob/master/LICENSE)

A simple downloader for some official mango.

While I did say this is a "downloader", it can also be said it is an almost full-blown replacement of the app/web version
as a CLI application.

All of the implementations here are originally a separate script that I made myself, I decide to open source it because
I'm kinda burned out doing it myself so someone else can do it.

Please see each folder implementations for how you can authenticate your account with `tosho`

## Installation

**Requirements**
- Rust 1.72+
- 64-bit devices (ARM64/aarch64 support might be experimental)
- Modern enough terminal (VT support)

You can get the binary by either compiling everything yourself by running:
1. Clone the repository
2. Run `cargo build --release --all`
3. Execute `target/release/tosho` (or `target/release/tosho.exe` on Windows)

Or, you can get the precompiled binary from the **[Releases](https://github.com/noaione/tosho-mango/releases)** tab.

## License

[MIT License](LICENSE)

## Acknowledgements

- `neckothy`, provided some help and info for KMKC.
- myself, created this from scratch
