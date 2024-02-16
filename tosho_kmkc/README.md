# tosho-kmkc

A minimal asynchronous client for the KM API by KC.

The following crate is used by the `tosho` app.

## Usage

Download the `tosho` app, or you can utilize this crate like any other Rust crate:

```rust
use tosho_kmkc::{KMClient, KMConfig, KMConfigMobile, KMConfigMobilePlatform};

#[tokio::main]
async fn main() {
    let config = KMConfigMobile {
        user_id: "123",
        hash_key: "abcxyz",
        platform: KMConfigMobilePlatform::Android,
    };

    let client = KMClient::new(KMConfig::Mobile(config));

    let manga = client.get_titles(vec![10007]).await.umwrap();
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

There is no significant difference between Android and iOS.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

### Versioning

The following crates follow the [`tosho`](https://crates.io/crates/tosho) crates version, please see [CHANGELOG](https://github.com/noaione/tosho-mango/blob/master/CHANGELOG.md) to see if there is any changes to this crates.
