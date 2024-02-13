# tosho-sjv

A minimal asynchronous client for the SJ API by V.

The following crate is used by the `tosho` app.

## Usage

Download the `tosho` app, or you can utilize this crate like any other Rust crate:

```rust
use tosho_sjv::{SJClient, SJConfig, SJMode};
use tosho_sjv::constants::get_constants;

#[tokio::main]
async fn main() {
    let config = SJConfig {
        user_id: 123,
        token: "xyz987abc",
        instance: "abcxyz",
    };
    let constants = get_constants(1);

    let client = SJClient::new(config, constants, SJMode::VM);
    let manga = client.get_manga(777).await.unwrap();
    println!("{:?}", manga);
}
```

## Authentication

The following sources only have one method of authentication, and that method uses your email and password.

```bash
$ tosho sj auth email password
```
