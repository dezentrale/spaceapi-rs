# Rust SpaceAPI Implementation

[![GitHub Actions Build Status](https://github.com/dezentrale/spaceapi-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/dezentrale/spaceapi-rs/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/spaceapi.svg)](https://crates.io/crates/spaceapi-dezentrale)
[![Crates.io Downloads](https://img.shields.io/crates/d/spaceapi.svg)](https://crates.io/crates/spaceapi-dezentrale)

This is an implementation of the [SpaceAPI](https://spaceapi.io/) v0.13 and v14
in Rust. It contains both the type definitions as well as tools for
serialization and deserialization to/from JSON using Serde.

- Crate Documentation: https://docs.rs/spaceapi-dezentrale/
- SpaceAPI Documentation: https://spaceapi.io/pages/docs.html

This library requires Rust 1.36.0 or newer.


## Usage

Add `spaceapi-dezentrale` to your `Cargo.toml`:

    [dependencies]
    spaceapi-dezentrale = "^0.8.2-rc1.1"


## Docs

You can build docs with `make docs`. Find them in the `target/doc/` directory.


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
