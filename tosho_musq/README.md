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

## Authentication

The following sources does not have any easy authentication method.

The command to authenticate is `tosho mu auth`.

It's recommended you setup network intercepting first, please read [INTERCEPTING](../INTERCEPTING.md)

```bash
$ tosho mu auth secret -t android
```

```bash
$ tosho mu auth secret -t ios
```

### Android

1. Open the source app
2. Click on home page or my page.
3. See the request on HTTP Toolkit and find request to the API that have `secret` as the query parameters.
4. Save that secret elsewhere and authenticate with `tosho`.

### Apple

1. Open the Stream app and click `Sniff Now`
2. Go to the source app and open the home or my page.
3. Go back to the Stream app and click `Sniff History` and select the most recent item.
4. Find request that goes to the API of the source app and find request that have `secret=xxxxx` on them.
5. Copy the link and save the secret value somewhere so you can authenticate with the `tosho`.
