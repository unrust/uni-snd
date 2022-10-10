# unrust / uni-snd

[![Build Status](https://travis-ci.org/unrust/uni-snd.svg?branch=master)](https://travis-ci.org/unrust/uni-snd)
[![Documentation](https://docs.rs/uni-snd/badge.svg)](https://docs.rs/uni-snd)
[![crates.io](https://meritbadge.herokuapp.com/uni-snd)](https://crates.io/crates/uni-snd)

This library is a part of [Unrust](https://github.com/unrust/unrust), a pure rust native/wasm game engine.
This library provides a low level native/wasm compatibility layer for following components :
* Sound output

## Usage

See oscillator example.

## Build

### As web app (wasm32-unknown-unknown)

Install wasm32 target :
```
rustup target install wasm32-unknown-unknown
```
Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
and [npm](https://www.npmjs.com/get-npm)

Compile the demo with
```
wasm-pack build examples
```
This creates a wasm package in examples/pkg

Run the demo with
```
cd www
npm install
npm run start
```

Open your browser at http://localhost:8080/

### As desktop app (native-opengl)

```
cargo run --example oscillator --release
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
