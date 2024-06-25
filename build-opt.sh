uglifyjs static/gl.js static/taca.js -o docs/demo/taca.js && \
cargo build --target wasm32-unknown-unknown --profile release-lto && \
wasm-opt -Os target/wasm32-unknown-unknown/release-lto/taca.wasm \
    -o docs/demo/taca.wasm
