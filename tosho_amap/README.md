# tosho-amap

A minimal asynchronous client for the AM API by AP.

The following crate is used by the `tosho` app.

## Usage

Download the `tosho` app, or you can utilize this crate like any other Rust crate:

```rust
use tosho_amap::{AMClient, AMConfig};

#[tokio::main]
async fn main() {
    let config = AMConfig {
        token: "123",
        identifier: "abcxyz",
        session_v2: "xyz987abc",
    };

    let client = AMClient::new(config);

    let manga = client.get_comic(48000051).await.umwrap();
    println!("{:?}", manga);
}
```

## Authentication

The following sources only have one method of authentication, and that method uses your email and password.

```bash
$ tosho am auth email password
```
