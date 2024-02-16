# tosho-mango

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://socialify.git.ci/noaione/tosho-mango/image?description=1&font=Rokkitt&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Dark">
  <img alt="tosho-mango Repository Info as Image" src="https://socialify.git.ci/noaione/tosho-mango/image?description=1&font=Rokkitt&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Light">
</picture>

<div align="center">
  <a href="https://github.com/noaione/tosho-mango/actions/workflows/ci.yml"><img src="https://github.com/noaione/tosho-mango/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="https://github.com/noaione/tosho-mango/blob/master/LICENSE"><img src="https://img.shields.io/github/license/noaione/tosho-mango" alt="License: MIT" /></a><br />
  <img alt="Crates.io Version" src="https://img.shields.io/crates/v/tosho">
  <a href="https://crates.io/crates/tosho"><img src="https://img.shields.io/crates/d/tosho?logo=rust" alt="Crates.io Total Downloads" /></a>
  <a href="https://github.com/noaione/tosho-mango/releases"><img src="https://img.shields.io/github/downloads/noaione/tosho-mango/total?logo=github" alt="GitHub Total Downloads" /></a>
  <br /><br />
  <p>A simple downloader for some official mango.</p>
</div>

`tosho-mango` (or `tosho`) is a downloader but can also be considered an almost full-blown replacement for the app/web version, with the exception of currency purchase, as a simple CLI application.

All of the implementations started as a personal script that I used before I decided to rewrite it into a proper CLI app with the help of other people to figure out some parts that I had trouble with.

### But, why?
- I hate using the app.
- I want to have my own local copy for my self-hosted instance.
- And, I'm kinda burned out from doing a *certain* thing and hope someone else can handle it.

This is just a fun side project, and as a disclaimer, I'm not condoning anything that will get you into trouble.

## Installation

```
cargo install tosho
```

Or, with [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):
```
cargo binstall tosho
```

Or, if you want to build manually:

**Requirements:**
- Rust 1.72+
- 64-bit devices (ARM64/aarch64 support might be experimental)
- Modern enough terminal (VT support)

You can get the binary by either compiling everything yourself by following these steps:
1. Clone the repository.
2. Run `cargo build --release --all`.
3. Execute `target/release/tosho` (or `target/release/tosho.exe` on Windows).

Alternatively, you can obtain the precompiled binary from:
- The **Stable** release in the **[Releases](https://github.com/noaione/tosho-mango/releases)** tab.
- The **Nightly** release from any latest successful commits: [Master CI](https://github.com/noaione/tosho-mango/actions/workflows/ci.yml?query=branch%3Amaster).

## Usage

Refer to each source's folder for information on authenticating each source with `tosho`.<br />
For a list of available commands, use the `--help` argument.

[![asciicast](https://asciinema.org/a/636303.svg)](https://asciinema.org/a/636303)

## License

[MIT License](LICENSE)

## Acknowledgements

- `neckothy`, provided some help and info for KMKC.
- myself, created this from scratch

### Legacy Python Code

The codebase has been rewritten in Rust. You can find the original Python version in the [`legacy-snek`](https://github.com/noaione/tosho-mango/tree/legacy-snek) branch.
