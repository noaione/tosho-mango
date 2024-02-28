# tosho-kmkc

A minimal asynchronous client for the KM API by KC.

The following crate is used by the [`tosho`](tosho) app.

## Usage

Download the [`tosho`](tosho) app, or you can utilize this crate like any other Rust crate:

```rust
use tosho_kmkc::{KMClient, KMConfig, KMConfigMobile, KMConfigMobilePlatform};

#[tokio::main]
async fn main() {
    let config = KMConfigMobile {
        user_id: "123".to_string(),
        hash_key: "abcxyz".to_string(),
        platform: KMConfigMobilePlatform::Android,
    };

    let client = KMClient::new(KMConfig::Mobile(config));

    let manga = client.get_titles(vec![10007]).await.unwrap();
    println!("{:?}", manga[0]);
}
```

## Authentication

The following source has many kinds of authentication methods:
- `auth`: Experimental login system with email + password.
- `auth-mobile`: Login by providing user ID and key.
- `auth-web`: Login by providing a [Netscape Cookies file](http://fileformats.archiveteam.org/wiki/Netscape_cookies.txt).
- `auth-adapt`: Convert a web authentication into mobile authentication.

For the easiest method, use the `auth` command and then `auth-adapt` to obtain the mobile version.

```bash
$ tosho km auth email password -t web
```

Alternatively, if you only want the mobile version:

```bash
$ tosho km auth email password -t android
```

```bash
$ tosho km auth email password -t ios
```

Or, if you use this crates as library:

```rust
use tosho_kmkc::{KMClient, KMConfigMobilePlatform};

#[tokio::main]
async fn main() {
    let login_res = KMClient::login("test@mail.com", "mypassword", None).await.unwrap();
    // Or, with mobile platform
    let login_res = KMClient::login("test@mail.com", "mypassword", Some(KMConfigMobilePlatform::Android)).await.unwrap();
}
```

There is no significant difference between Android and iOS.

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

[tosho]: https://crates.io/crates/tosho