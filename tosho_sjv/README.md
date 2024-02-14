# tosho-sjv

A minimal asynchronous client for the SJ API by V.

The following crate is used by the `tosho` app.

## Usage

Download the `tosho` app, or you can utilize this crate like any other Rust crate:

```rust
use tosho_sjv::{SJClient, SJConfig, SJMode, SJPlatform};

#[tokio::main]
async fn main() {
    let config = SJConfig {
        user_id: 123,
        token: "xyz987abc",
        instance: "abcxyz",
        platform: SJPlatform::Android,
    };

    let client = SJClient::new(config, SJMode::VM);
    let manga = client.get_manga(777).await.unwrap();
    println!("{:?}", manga);
}
```

## Authentication

The following sources only have one method of authentication, and that method uses your email and password.

```bash
$ tosho sj auth email password
```

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
