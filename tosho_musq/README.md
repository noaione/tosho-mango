# tosho-musq

An asynchronous client for the MU! API by SQ.

The following crate is used by the `tosho` app.

## Usage

Download the `tosho` app, or you can utilize this crate like any other Rust crate:

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

## Authentication

The following sources do not have any easy authentication method.

The command to authenticate is `tosho mu auth`.

It's recommended that you set up network intercepting first; please read [INTERCEPTING](https://github.com/noaione/tosho-mango/blob/master/INTERCEPTING.md).

```bash
$ tosho mu auth secret -t android
```

```bash
$ tosho mu auth secret -t ios
```

### Android

1. Open the source app.
2. Click on the home page or my page.
3. Observe the requests on HTTP Toolkit and find the request to the API that has `secret` as the query parameters.
4. Save that secret elsewhere and authenticate with `tosho`.

### Apple

1. Open the Stream app and click `Sniff Now`.
2. Go to the source app and open the `Home` or `My Page`.
3. Return to the Stream app and click `Sniff History`, then select the most recent item.
4. Find the request that goes to the API of the source app and locate the request that has `secret=xxxxx` in them.
5. Copy the link and save the secret value somewhere so you can authenticate with `tosho`.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

### Versioning

The following crates follow the [`tosho`](https://crates.io/crates/tosho) crates version, please see [CHANGELOG](https://github.com/noaione/tosho-mango/blob/master/CHANGELOG.md) to see if there is any changes to this crates.
