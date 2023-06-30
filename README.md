# LIBFPRINT-RS
This crate provides a wrapper around the native `libfprint` library. 

## Dependencies
This package requires `libfprint-2` to be installed in your system alongside `libclang` and `pkg-config`

## Usage
Add `libfprint-rs` as a dependency in `Cargo.toml`
```
[dependencies]
libfprint-rs = "0.1.10"
```
Or using `cargo`
```
cargo add libfprint-rs
```
Import the `libfprint_rs` crate. The starting point for nearly all `libfprint-rs` functionality is to create a context object. With a context object, you can list devices, open them and execute their functionalities.
```rust
use libfprint_rs::device::FpEnrollProgress;
use libfprint_rs::{self, context::FpContext, device::FpDevice, error::GError, print::FpPrint};

fn main() {
    let ctx = FpContext::new();
    let dev = ctx.get_devices().iter().next().unwrap();

    dev.open().unwrap();
}
``` 
## License 
Distributed under the [MIT License](LICENSE).

### Status
Currently libfprint-rs is WIP.

Todo:
* Add GCancellable or an abstraction to that type to allow users to cancel operations (such as enroll, verify, identify).
* Handle fingerprint initialized passed to `FpDevice` implementations.