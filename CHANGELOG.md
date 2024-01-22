# Changelog

Starting from Rust port of the project, all changes will be put into this file.

## Unreleased (git master)

Nothing yet.

## [0.2.1] 2024-01-22
### Build
- Add proper version hash if `tosho` built as nightly or by yourself.

### Changes
- Add missing help description for `tosho tools`
- Make terminal looks more consisten
- `MU`: Fix rankings selector not working as intended
- `AM`: Simplify `strptime` to only show `YYYY-MM-DD`

### Docs
- Rewording and grammar fixes

## [0.2.0] 2024-01-22
### New Features
- Added `AM` as a new source
- Support proper login for `KM` source
- Support mobile source (both Android and iOS) for `KM` source.

### Changed
- Port everything into Rust, legacy Python can be seen from [legacy-snek](https://github.com/noaione/tosho-mango/tree/legacy-snek) branch.
- Split every sources into their own Rust crates. (prefixed with `tosho_` in folder or `tosho-` in crates name)

### Incompatibility
- You would need to re-authenticate your `KM` web session since there is a bit of changes and it might not be compatible anymore.
