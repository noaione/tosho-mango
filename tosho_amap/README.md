# tosho-amap

![crates.io version](https://img.shields.io/crates/v/tosho-amap)

A minimal asynchronous client for the AM API by AP.

The following crate is used by the [`tosho`](https://crates.io/crates/tosho) app.

## Usage

Download the [`tosho`](https://crates.io/crates/tosho) app, or you can utilize this crate like any other Rust crate:

```rust,no_run
use tosho_amap::{AMClient, AMConfig};

#[tokio::main]
async fn main() {
    let config = AMConfig::new("123", "abcxyz", "xyz987abc");
    let client = AMClient::new(config).unwrap();

    let manga = client.get_comic(48000051).await.unwrap();
    println!("{:?}", manga);
}
```

## Authentication

The following sources only have one method of authentication, and that method uses your email and password.

```bash
$ tosho am auth email password
```

Or, if you use this crates as library:

```rust,no_run
use tosho_amap::AMClient;

#[tokio::main]
async fn main() {
    let login_res = AMClient::login("test@mail.com", "mypassword").await.unwrap();
}
```

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)
