# tosho-mplus

![crates.io version](https://img.shields.io/crates/v/tosho-mplus)

An asynchronous client for the M+ API by S.

The following crate is used by the [`tosho`](https://crates.io/crates/tosho) app.

## Usage

Download the [`tosho`](https://crates.io/crates/tosho) app, or you can utilize this crate like any other Rust crate:

```rust,no_run
use tosho_mplus::MPClient;
use tosho_mplus::proto::Language;
use tosho_mplus::constants::get_constants;

#[tokio::main]
async fn main() {
    let client = MPClient::new("1234", Language::English, get_constants(1)).unwrap();
    let home_view = client.get_home_page().await.unwrap();
}
```

Available `get_constants` value are:
- `1`: Android

## Authentication

The following sources do not have any easy authentication method.

The command to authenticate is `tosho mp auth`.

It's recommended that you set up network intercepting first; please read [INTERCEPTING](https://github.com/noaione/tosho-mango/blob/master/INTERCEPTING.md).

Using the CLI, you can do this:

```bash
$ tosho mp auth secret
```

With crates, you can follow the above usages.

### Android

1. Open the source app.
2. Click on the home page or my page.
3. Observe the requests on HTTP Toolkit and find the request to the API that has `secret` as the query parameters.
4. Save that secret elsewhere and authenticate with `tosho`.

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)
