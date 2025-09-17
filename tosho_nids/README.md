# tosho-nids

![crates.io version](https://img.shields.io/crates/v/tosho-nids)

A minimal asynchronous client for NI by DS.

The following crate is used by the [`tosho`](https://crates.io/crates/tosho) app.

## Usage

Download the [`tosho`](https://crates.io/crates/tosho) app, or you can utilize this crate like any other Rust crate:

```rust,no_run
use tosho_nids::{Filter, NIClient};

#[tokio::main]
async fn main() {
    let constants = tosho_nids::constants::get_constants(1); // Web
    let client = NIClient::new(None, constants).unwrap();

    let issues = client.get_issues(Filter::default().with_per_page(18)).await.unwrap();
    println!("Issues: {:?}", issues);
}
```

## Authentication

The following sources only have one method of authentication, and that method uses your email and password.

```bash
$ tosho ni auth <jwtToken>
```

Or, if you use the crates:

```rust,no_run
use tosho_nids::NIClient;

#[tokio::main]
async fn main() {
    let constants = tosho_nids::constants::get_constants(1); // Web
    let client = NIClient::new(Some("your_jwt_token_here"), constants).unwrap();
    
    // Now you can make authenticated requests
    let my_series = client.get_series_run_collections(None).await.unwrap();
    println!("My series: {:?}", my_series);
}
```

To get the JWT token, you need to login to your account on the website, then:
1. Open the developer tools or inspector
2. Go to the "Application" tab
3. Look for "Local Storage" in the sidebar and the site URL
4. Find the key `access_token` and copy the value
5. Authenticate using the CLI or the crate

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)
