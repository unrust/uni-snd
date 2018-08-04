# unrust / uni-snd

[![Build Status](https://travis-ci.org/unrust/uni-snd.svg?branch=master)](https://travis-ci.org/unrust/uni-snd)
[![Documentation](https://docs.rs/uni-snd/badge.svg)](https://docs.rs/uni-snd)
[![crates.io](https://meritbadge.herokuapp.com/uni-snd)](https://crates.io/crates/uni-snd)

This library is a part of [Unrust](https://github.com/unrust/unrust), a pure rust native/wasm game engine.
This library provides a low level native/wasm compatibility layer for following components :
* Sound output

**This project is under heavily development, all api are very unstable until version 0.2**

## Usage

See examples

## Build

### As web app (wasm32-unknown-unknown)

When targetting `wasm32-unknown-unknown`, stdweb currently requires Rust nightly.

```
cargo install --force cargo-web # installs web sub command
rustup override set nightly
rustup target install wasm32-unknown-unknown
cargo web start --example basic --release
```

### As desktop app (native-opengl)

Native compilation works with current stable Rust (1.28)

```
rustup override set stable
cargo run --example basic --release
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
