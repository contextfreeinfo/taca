# Taca

A runtime for multimedia wasm apps that runs native and in browsers.

Build optimized:

```
# TODO Minify js.
cargo build --target wasm32-unknown-unknown --profile release-lto
wasm-opt -Os target/wasm32-unknown-unknown/release-lto/taca.wasm \
    -o target/wasm32-unknown-unknown/release-lto/taca.opt.wasm
```

These files are copied and maybe modified from miniquad:

- examples/gl.js
- examples/index.html
