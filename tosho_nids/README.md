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

    let filter = Filter::default()
        .add_filter(tosho_nids::FilterType::Title, "Attack on Titan")
        .with_per_page(18);
    let issues = client.get_issues(&filter).await.unwrap();
    println!("Issues: {:?}", issues);
}
```

**Also see**: [FILTERS.md][filters-example] for more information about the filters system.

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

### Understanding reader frames
Some pages may have multiple frames for guided reading. These frames are defined by their coordinates and dimensions relative to the page size.

Minimized example:
```json
"frames": [
    {
        "index": 0,
        "x": 0.04467951784087,
        "y": 0.032608695652174,
        "width": 0.91787978937503,
        "height": 0.20893719806763,
        "opacity": 0.75
    }
]
```

With the full image size being 2550x3263 pixels, the first frame would be located at:
- X: 0.04467951784087 * 2550 ≈ 114 pixels
- Y: 0.032608695652174 * 3263 ≈ 106 pixels
- Width: 0.91787978937503 * 2550 ≈ 2340 pixels
- Height: 0.20893719806763 * 3263 ≈ 681 pixels

That would means you would need to create a highlight box at (114, 106) with a size of 2340x681 pixels on the full image for the first frame.

Note:
- Opacity is used to define how transparent the outer area should be when highlighting the frame.
- There is also `color` field, although currently unused but may be used in the future to define the outer area color.
   - Note that the color can be anything supported by CSS color values.
- The way the website do it is creating an SVG wrapper that has `image` and then creating `mask` of white + black rectangles to highlight the frame area.
   - With the black frame being the frame area, and white being the outer area.
   - The mask itself also has a color fill that can be defined by the `color` field.

## Disclaimer

This project is designed as an experiment and to create a local copy for personal use. These tools will not circumvent any paywall, and you will need to purchase and own each chapter with your own account to be able to make your own local copy.

We're not responsible if your account got deactivated.

## License

This project is licensed with MIT License ([LICENSE](https://github.com/noaione/tosho-mango/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)

[filters-example]: https://github.com/noaione/tosho-mango/blob/master/tosho_nids/FILTERS.md
