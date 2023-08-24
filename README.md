# LIBFPRINT-RS

This crate provides a wrapper around the native `libfprint` library.

## Dependencies

This package requires `libfprint-2` to be installed in your system alongside `libclang` and `pkg-config`

## Usage

Add `libfprint-rs` as a dependency in `Cargo.toml`

```toml
[dependencies]
libfprint-rs = "0.2.0"
```

Or using `cargo`

```bash
cargo add libfprint-rs
```

Import the `libfprint_rs` crate. The starting point for nearly all `libfprint-rs` functionality is to create a context object. With a context object, you can list devices, open them and execute their functionalities.

```rust
use libfprint_rs::FpContext;

fn main() {
    let ctx = FpContext::new();
    let devices = ctx.devices();
    let dev = devices.get(0).unwrap();

    dev.open_sync(None).unwrap();
}

```

## License

Distributed under the [MIT License](LICENSE).

### Status

Currently libfprint-rs is WIP.
