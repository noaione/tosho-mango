# tosho-mango

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://socialify.git.ci/noaione/tosho-mango/image?description=1&font=Rokkitt&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Dark">
  <img alt="tosho-mango Repository Info as Image" src="https://socialify.git.ci/noaione/tosho-mango/image?description=1&font=Rokkitt&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Light">
</picture>

<div align="center">
  <a href="https://github.com/noaione/tosho-mango/actions/workflows/ci.yml"><img src="https://github.com/noaione/tosho-mango/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="https://github.com/noaione/tosho-mango/blob/master/LICENSE"><img src="https://img.shields.io/github/license/noaione/tosho-mango" alt="License: MIT" /></a>
  <br /><br />
  <p>A simple downloader for some official mango.</p>
</div>

`tosho-mango` (or `tosho`) is a downloader but can also be said as an almost full-blown replacement of the app/web version with the exception of currency purchase as a simple CLI application.

All of the implementations started as a personal script that I use before I decide to rewrite it into a proper CLI app with the help of other people to figure out some part that I have trouble with.

### But, why?
- I hate using the app
- I want to have my own local copy for my own self-hosted instance
- And, I'm kinda burned out doing a *certain* thing and hope someone else can do it.

This is just a fun project and as a disclaimer, I'm not condoning anything that will get you into trouble.<br />
I mainly created this project to get my own local copy of stuff I read so I can have it indefinitely on my own server.

## Installation

**Requirements**
- Rust 1.72+
- 64-bit devices (ARM64/aarch64 support might be experimental)
- Modern enough terminal (VT support)

You can get the binary by either compiling everything yourself by running:
1. Clone the repository
2. Run `cargo build --release --all`
3. Execute `target/release/tosho` (or `target/release/tosho.exe` on Windows)

Or, you can get the precompiled binary:
- **Stable** release from the **[Releases](https://github.com/noaione/tosho-mango/releases)** tab.
- **Beta/canary** release from any latest successful commits: [Master CI](https://github.com/noaione/tosho-mango/actions/workflows/ci.yml?query=branch%3Amaster)

## Usage

See each sources folder on how to use (mainly authentication) each and utilize sources that I've created.

## License

[MIT License](LICENSE)

## Acknowledgements

- `neckothy`, provided some help and info for KMKC.
- myself, created this from scratch

### Legacy Python Code

The codebase has been rewritten to Rust, you can see the original Python version in [`legacy-snek`](https://github.com/noaione/tosho-mango/tree/legacy-snek) branch.
