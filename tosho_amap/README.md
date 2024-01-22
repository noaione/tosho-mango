# tosho-amap

An asynchronous client of AM API by AP.

The following crate is used by the `tosho` app.

## Usages

Download `tosho` crate/app, or you can utilize this crate like any other Rust crate:

```rust
```

## Authentication

The following source have many kind of authentication method:
- `auth`, experimental login system with email + password
- `auth-session`, login by manually providing latest session, device ID, and token.

For the most easiest one, use `auth` command.

```bash
$ tosho am auth email password
```
