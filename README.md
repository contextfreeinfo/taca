# Taca

A runtime for multimedia wasm apps that runs native and in browsers.

Build optimized:

```
# TODO Minify js.
wat2wasm src/hi.wat -o src/hi.wasm
naga src/shader.wgsl src/shader.spv
spirv-opt -Os src/shader.spv -o src/shader.opt.spv
cargo build --target wasm32-unknown-unknown --profile release-lto
wasm-opt -Os target/wasm32-unknown-unknown/release-lto/taca.wasm \
    -o target/wasm32-unknown-unknown/release-lto/taca.opt.wasm
```

These files are copied and maybe modified from miniquad:

- static/gl.js
- static/index.html

## Exploration

Size on WGSL:

```
-rw-r--r--  1 tom tom  957899 Jun  9 06:09 taca.opt.wasm
-rwxr-xr-x  2 tom tom 1062610 Jun  9 10:13 taca.wasm
...
-rw-r--r-- 1 tom tom  424 Jun  7 14:12 shader.wgsl
-rw-r--r-- 1 tom tom  226 Jun  7 14:12 shader.wgsl.gz
```

Size on SPIR-V:

```
-rw-r--r--  1 tom tom 766449 Jun  9 10:58 taca.opt.wasm
-rwxr-xr-x  2 tom tom 852427 Jun  9 10:58 taca.wasm
...
-rw-r--r-- 1 tom tom 1056 Jun  9 10:55 shader.spv
-rw-r--r-- 1 tom tom  438 Jun  9 10:55 shader.spv.gz
```
