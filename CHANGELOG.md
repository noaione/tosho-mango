# Changelog

Starting from Rust port of the project, all changes will be put into this file.

## Unreleased (git master)
### New Features
- `RB`: Try to support hidden higher resolution images

We've added a simple helper function in `RBClient` to help modify the URL to get higher resolution images.

```rust
use tosho_rbean::{RBClient, config::{RBConfig, RBPlatform}};

#[tokio::main]
async fn main() {
    let mut client = RBClient::new(RBConfig {
      token: "",
      refresh_token: "",
      platform: RBPlatform::Android,
    });

    // Get the chapters list
    let chapters = client.get_chapter_list("73cd427b-0740-4911-91bf-ed17cd13df8e").await.unwrap();
    let first_chapter = chapters.chapters[0].clone();

    // Get the first chapter viewer
    let viewer = client.get_chapter_viewer(&first_chapter.uuid).await.unwrap();
    let first_page = viewer.data.pages[0].clone();

    // Get the first image URL
    let image_url = first_page.image.jpg[0].clone();
    // Get the high resolution image URL
    let high_res_image_url = RBClient::modify_url_for_highres(&image_url);
    println!("High res image URL: {}", high_res_image_url);
}
```

If you use CLI, this is done automatically when you use `download` command.

### Changes
- `KM`: Fix failed deserialization on `TitleNode` on some titles
- `RB`: Fix auth failing when reauthenticating
- `RB`: Properly hide future chapters from being downloaded
- `RB`: Do streaming decryption on image download
- Multiple: Adjustment to constants
- Multiple: Mark some `prost::Message` with `Copy`
- All: Rewrite `lazy_static` to `LazyLock`
- Use static `OnceLock` in win-term VT check
- Simplify win-term VT check
- Expose some `_HOST` constants to public API

### Build
- Update multiple dependencies
- Remove `lazy_static` as direct dependency
- Update MSRV to 1.80.0

## [0.5.1] 2024-06-08
### New Features
- `M+`: Allow overriding app version code.

This should help mitigates issue with needing to update to latest version.

You can use in CLI like this:
```bash
$ tosho mp -p XXX [...]
```

Or, in Rust code:
```rust
use tosho_mplus::MPClient;
use tosho_mplus::proto::Language;
use tosho_mplus::constants::get_constants;

#[tokio::main]
async fn main() {
    let client = MPClient::new("1234", Language::English, get_constants(1))
        .with_app_ver(256);
    let home_view = client.get_home_page().await.unwrap();
}
```

### Changes
- `KM`: Cleanup chapters information a bit
- `M+`: Update app version
- INTERNAL: Use more macros expansion on config implementation.

### Build
- Update `tokio`, `serde`, `prost`, and `documented`

## [0.5.0] 2024-05-20
### New Features
- Add **`M+`** as a new source
- `MU`: Support downloading with subscriptions
- Add command to clear cache for all sources: `tosho tools clear-cache`

### Changes
- All source: Force use `rustls` and use `http2` adaptive window for reqwest client.
- `MU`: Update proto information.
- `MU`: Only use `HTTP/1.1` since `HTTP/2` broke the API request
- `MU`: Rework downloader, image blocks are now stored to make less request to the API
- `MU`: Fix account revoke not working
- `RB`: Fix wrong base host used in homepage view
- `AM`: Add title information when purchasing/downloading
- `AM`, `RB`, and `SJ/M`: Cleanup chapters information
- `SJ/M`: Early fetch chapters information
- `SJ/M`: Improve search/pattern matching
- `SJ/M`: Add `show_to` to notices and add simple helper if the notice should be displayed or not
- `SJ/M`: Return the whole series response for `get_chapters`
- Refactor some duplicate code
- Changes all `ToString` occurences to `std::fmt::Display`

### Build
- Bump `reqwest` to 0.12 (use hyper v1)
- Use `rustls` instead of `native-tls` for `reqwest`
- Set `stream` feature as default on `reqwest`
- Compile with latest Rust stable in CI
- Bump other dependencies

### Docs
- Update `SerializeEnum` rustdoc

## [0.4.3] 2024-03-20
### New Features
- Check for update on startup (this is done for the next 24 hours after each check)

### Changes
- `MU`: Fix panic when search results is less than 25
- `AM`: Better error messages

### Build
- Change `diacritics` to `secular` because of OSS license issue.
- Bump dependencies on all crates (except `macros`)
- Use only `png`, `jpeg`, and `rayon` features for `image-rs`

### Docs
- Suggest using `--locked` when installing
- Add crates.io version shield badge on each crates

### Tests
- Move LFS data to another repo and rewrite tests to support the new path/format

## [0.4.2] 2024-02-28
### Changes
- `SJ/M`: Download the last page properly (previously it was missing)
- `SJ/M`: Fix failed deser on notices
- `SJ/M`: Show latest available chapter properly

### Build
- Pin source crate dependency on `tosho`

### Docs
- Add disclaimer

## [0.4.1] 2024-02-23
### Changes
- `RB`: Remove saving JSON response on error.

## [0.4.0] 2024-02-22
### New Features
- Added **`小豆 (Red Bean)`** as a new source
- Introduce threaded/parallel image download (not chapter) for the following source:
  - [KM by KC](https://crates.io/crates/tosho-kmkc)
  - [SJ/M by V](https://crates.io/crates/tosho-sjv)
  - [小豆 (Red Bean) by KRKR](https://crates.io/crates/tosho-rbean)

By default, the download is sequential. If you want to enable parallel download pass `--parallel` into the
argument list: `tosho km download 10007 --parallel`

### Changes
- `MU`: Refactor API response parsing
- `SJ`: Wrap descrambling with `tokio::task::spawn_blocking`
- `KM`: Wrap descrambling with `tokio::task::spawn_blocking`
- Internally change chapter dump to support both number (`u64`) and string (`UUID`-esque)
- Better progress tick on progress bar

## [0.3.3] 2024-02-17
### Changes
- Self update now should support windows zip properly.

## [0.3.2] 2024-02-17 (YANKED)
### Changes
- `SJ`: Properly login with requested platform.
- `SJ`: Make `DATA_VERSION_CODE` optional when sending requests
- `MU`: Use `&static Constants` for `MUClient::new()` params (**BREAKING CHANGES**)
- `Macros`: Add docs for `enum_error!()` macro
- Make `linkify` from `tosho` crates to be crate-only (a.k.a hide it)

### Docs
- Add proper documentation for `tosho-macros`
- Add proper documentation for `tosho-musq`
- Add proper documentation for `tosho-kmkc`
- Add proper documentation for `tosho-amap`
- Add proper documentation for `tosho-sjv`

### Build
- Make all source crate to not follow workspace version.

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
- Make `tosho-macros` to not follow workspace versioning.

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
