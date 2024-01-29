# Changelog

Starting from Rust port of the project, all changes will be put into this file.

## [0.2.2] (unreleased; git master)
### New Features
- `AM`: Add `favorites` command
- `KM`: Add `favorites` command

### Changes
- `KM`: Fix favorite status deserialization
- `AM`: Replace session v2 cookie in each request

### Docs
- Add missing documentation for `tosho-amap`

### Build
- Pin and bump dependencies on GitHub CI

## [0.2.1] 2024-01-22
### Build
- Add proper version hash if `tosho` built as nightly or by yourself.

### Changes
- Add missing help description for `tosho tools`
- Make terminal looks more consistent
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