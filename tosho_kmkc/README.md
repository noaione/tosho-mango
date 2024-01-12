# tosho-kmkc

An asynchronous client of KM API by KC.

To know what is KMKC/KM API, please decode the `base64` data in the constants file.

The following crate is used by the `tosho` app.

## Usages

Download `tosho` crate/app, or you can utilize this crate like any other Rust crate:

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

The following source have many kind of authentication method:
- `auth`, experimental login system with email + password
- `auth-mobile`, login by providing user ID and key
- `auth-web`, login by providing [Netscape Cookies file](http://fileformats.archiveteam.org/wiki/Netscape_cookies.txt)
- `auth-adapt`, convert a web auth into mobile auth.

For the most easiest one, use `auth` command and then `auth-adapt` to get mobile version of it.

```bash
$ tosho km auth email password -t web
```

Or, if you just want mobile version

```bash
$ tosho km auth email password -t android
```

```bash
$ tosho km auth email password -t ios
```

There is no significant difference between Android and iOS.