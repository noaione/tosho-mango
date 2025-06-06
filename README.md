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

`tosho-mango` (or `tosho`) is a manga downloader and general CLI tools for [official licensor platform](#supported-platform).

All of the implementations started as a personal script that I used before I decided to rewrite it into a proper CLI app with the help of other people to figure out some parts that I had trouble with.

### But, why?
- I hate using the app.
- I want to have my own local copy for my self-hosted instance.
- And, I'm kinda burned out from doing a *certain* thing and hope someone else can handle it.

This is just a fun side project, and as a disclaimer, I'm not condoning anything that will get you into trouble and I'm not responsible if you got banned from the platform you're using.

## Installation

**Requirements**:
- Rust 1.85.0+ (If manually building or using `cargo`)
- 64-bit devices (ARM64/aarch64 support is untested)
- Modern terminal with the following ANSI support:
  - Support [OSC-8](https://github.com/Alhadis/OSC8-Adoption#terminal-emulators)
  - Support [truecolor](https://github.com/termstandard/colors#terminal-emulators) and the standard 8/16 colors
  - Test code: https://gist.github.com/lilydjwg/fdeaf79e921c2f413f44b6f613f6ad53

`tosho` comes with a pre-compiled binary that you can choose:
- The **Stable** release in the **[Releases](https://github.com/noaione/tosho-mango/releases)** tab.
- The **Nightly** release from any latest successful commits: [Master CI](https://github.com/noaione/tosho-mango/actions/workflows/ci.yml?query=branch%3Amaster) / [nightly.link](https://nightly.link/noaione/tosho-mango/workflows/ci/master?preview).

You can also utilize `cargo`:
```bash
cargo install --locked tosho
```

Or, with [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):
```bash
cargo binstall --locked tosho
```

Or, you can clone and build manually:
1. Clone the repository.
2. Run `cargo build --release --all`.
3. Execute `target/release/tosho` (or `target/release/tosho.exe` on Windows).

**Note**:<br />
With the exception of building manually, all the builds above is always compiled with the `--release` flag and with the
latest stable Rust version.

## Usage

Refer to each source's folder for information on authenticating each source with `tosho`.<br />
For a list of available commands, use the `--help` argument.

[![asciicast](https://asciinema.org/a/636303.svg)](https://asciinema.org/a/636303)

## Supported Platform

We support the following platform:
- [MU! by SQ](https://github.com/noaione/tosho-mango/tree/master/tosho_musq) (Android, Apple)
- [KM by KC](https://github.com/noaione/tosho-mango/tree/master/tosho_kmkc) (Android, Apple, Web)
- [AM by AP](https://github.com/noaione/tosho-mango/tree/master/tosho_amap) (Android)
- [SJ/M by V](https://github.com/noaione/tosho-mango/tree/master/tosho_sjv) (Android, Apple, Web)
- [小豆 (Red Bean) by KRKR](https://github.com/noaione/tosho-mango/tree/master/tosho_rbean) (Android)
- [M+ by S](https://github.com/noaione/tosho-mango/tree/master/tosho_mplus) (Android)

## License

[MIT License](LICENSE)

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter on each platform with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## Acknowledgements

- `neckothy`, provided some help and info for KM.
- Tachiyomi team, general idea on how to descramble SJ image.
- myself, created this from scratch

### Legacy Python Code

The codebase has been rewritten in Rust. You can find the original Python version in the [`legacy-snek`](https://github.com/noaione/tosho-mango/tree/legacy-snek) branch.
