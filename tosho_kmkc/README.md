# tosho-kmkc

An asynchronous client of KM API by KC.

To know what is KMKC/KM API, please decode the `base64` data in the constants file.

The following crate is used by the `tosho` app.

## Usages

Download `tosho` crate/app, or you can utilize this crate like any other Rust crate:

```rust
use tosho_kmkc::{KMClient, KMConfig, KMConfigMobile};

#[tokio::main]
async fn main() {
    let config = KMConfigMobile {
        user_id: "123",
        user_token: "abcxyz",
    };

    let client = KMClient::new(KMConfig::Mobile(config));

    let manga = client.get_titles(vec![10007]).await.umwrap();
    println!("{:?}", manga[0]);
}
```
