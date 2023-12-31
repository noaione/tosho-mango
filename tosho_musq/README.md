# tosho-musq

An asynchronous client of MU! API by SQ.

To know what is MUSQ/MU! API, please decode the `base64` data in the constants file.

The following crate is used by the `tosho` app.

## Usages

Download `tosho` crate/app, or you can utilize this crate like any other Rust crate:

```rust
use tosho_musq::MUClient;
use tosho_musq::constants::ANDROID_CONSTANTS;

#[tokio::main]
async fn main() {
    let client = MUClient::new("1234", ANDROID_CONSTANTS);
    let manga = client.get_manga(240).await.unwrap();
    println!("{:?}", manga);
}
```
