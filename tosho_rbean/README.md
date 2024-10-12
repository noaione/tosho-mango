# tosho-rbean

![crates.io version](https://img.shields.io/crates/v/tosho-rbean)

A minimal asynchronous client for 小豆 (Red Bean) API.

The following crate is used by the [`tosho`](https://crates.io/crates/tosho) app.

## Usage

Download the [`tosho`](https://crates.io/crates/tosho) app, or you can utilize this crate like any other Rust crate:

```rust,no_run
use tosho_rbean::{RBClient, RBConfig, RBPlatform};

#[tokio::main]
async fn main() {
    let config = RBConfig::new("123", "abcxyz", RBPlatform::Android);
    let mut client = RBClient::new(config).unwrap();
    // Refresh token
    client.refresh_token().await.unwrap();
    let user = client.get_user().await.unwrap();
    println!("{:?}", user);
}
```

## Authentication

The following sources only have one method of authentication, and that method uses your email and password.

```bash
$ tosho rb auth email password --help
```

Or, if you use the crates:

```rust,no_run
use tosho_rbean::{RBClient, RBPlatform};

#[tokio::main]
async fn main() {
    let login_results = RBClient::login("email@test.com", "mypassword", RBPlatform::Android).await.unwrap();
    println!("{:?}", login_results);
}
```

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)
