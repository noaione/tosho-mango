# Changelog

Starting from Rust port of the project, all changes will be put into this file.

## Unreleased (git master)
### Changes
- `SJ`: Properly login with requested platform.
- `SJ`: Make `DATA_VERSION_CODE` optional when sending requests

### Docs
- Add proper documentation for `tosho-macros`

## [0.3.1] 2024-02-16
### New features
- Added self updater, you can now do `tosho update` to update tosho for supported platform/architecture.

### Changes
- `SJ`: Fix broken serde on renew field at account subcriptions
- `SJ`: Allow downloading "expired" chapters if you have subscription for it.
- `SJ`: Fix failed deser on chapters
- `SJ`: Show excerpt of data when deser fails
- `SJ`: Don't show/download future chapter

### Build
- Pin `windows-sys` dependencies (If you use crates.io, this is already pinned in `0.3.0`)
- Remove `mime` as direct dependency
- `tosho-macros` now pinned to 0.3.0 unless there's any changes

## [0.3.0] 2024-02-14
### New Features
- Added **`SJ/M`** as a new source

### Changes
- Fix some part of command locked behind account select
- `KM`: Optimize descrambled PNG size (Web only)
- `KM`: Descrambled image now follow the original color type instead of always saving as RGB8

### Build
- Remove `cookie-store` as direct dependency
- Remove `prost-types` (unused)
- Set `documented` to v0.3.0

## [0.2.3] 2024-02-07
### New Features
- Proxy support (`--proxy` can be used globally)
  - Example: `tosho --proxy https://proxy.example km download 10007`
- **BREAKING CHANGE**: Moved `-a` or `--account-id` to each source subcommand instead of each commands:
  - Before: `tosho km download 10007 -a 123`
  - After: `tosho km -a 123 download 10007`

### Changes
- `KM`: Fix invalid deserialization on `/account`
- `KM`: Fix invalid deserialization on `/user`

### Build
- Bump dependencies
- Use native M1 CI for building macOS arm64 version

## [0.2.2] 2024-01-29
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
- Smaller size by stripping debuginfo properly from `std`

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
