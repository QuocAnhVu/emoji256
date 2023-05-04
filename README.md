# emoji256

[![ga-svg]][ga-url]
[![crates-svg]][crates-url]
[![docs-svg]][docs-url]
[![codecov-svg]][codecov-url]
[![deps-svg]][deps-url]

[ga-svg]: https://github.com/QuocAnhVu/emoji256/actions/workflows/main.yml/badge.svg
[ga-url]: https://github.com/QuocAnhVu/emoji256/actions/workflows/main.yml
[crates-svg]: https://img.shields.io/crates/v/emoji256.svg
[crates-url]: https://crates.io/crates/emoji256
[docs-svg]: https://docs.rs/emoji256/badge.svg
[docs-url]: https://docs.rs/emoji256
[codecov-svg]: https://img.shields.io/codecov/c/github/QuocAnhVu/emoji256
[codecov-url]: https://codecov.io/gh/QuocAnhVu/emoji256
[deps-svg]: https://deps.rs/repo/github/QuocAnhVu/emoji256/status.svg
[deps-url]: https://deps.rs/repo/github/QuocAnhVu/emoji256

Emoji256 is a binary-to-text encoding scheme for reading hashes and cryptographic keys. By translating text to pictographs, we can use our automatic biological storytelling machinery to grok or even memorize arbritrary byte sequences.

To translate 8-bit values to a table of 256 emojis, emoji256 utilizes a pre-defined lookup table found in [`src/table.rs`](src/table.rs). This table contains a unique emoji for each of the 256 possible combinations of an 8-bit value, ranging from 0 to 255. **The lookup table is still actively being tweaked to reduce biases and increase grok-ability.**

This Rust crate encodes and decodes data into/from emoji256. It uses the same API as the popular Rust crate, [hex](https://crates.io/crates/hex).

## Examples

Encoding a `String`

```rust
let encoded_string = emoji256::encode("Hello world!");

println!("{}", encoded_string); // Prints "🐙👽💉💉💌🍭💦💌💕💉👻🍰"
```

Decoding a `String`

```rust
let decoded_string = String::from_utf8(emoji256::decode("🐙👽💉💉💌🍭💦💌💕💉👻🍰").unwrap()).unwrap();

println!("{}", decoded_string); // Prints "Hello world!"
```

You can find the [documentation](https://docs.rs/emoji256) here.

## Installation

In order to use this crate, you have to add it under `[dependencies]` to your `Cargo.toml`

```toml
[dependencies]
emoji256 = "0.2"
```

By default this will import `std`, if you are working in a
[`no_std`](https://rust-embedded.github.io/book/intro/no-std.html)
environment you can turn this off by adding the following

```toml
[dependencies]
emoji256 = { version = "0.2", default-features = false }
```

## Features

- `std`:
  Enabled by default. Add support for Rust's libstd types.
- `alloc`:
  Enabled by default. Add support for alloc types (e.g. `String`) in `no_std` environment.
- `serde`:
  Disabled by default. Add support for `serde` de/serializing library.
  See the `serde` module documentation for usage.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
