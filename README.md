# Taca

A runtime for multimedia wasm apps that runs native and in browsers.

## Live Demos

- [Mouse pointer spotlight on RGB triangle](https://contextfreeinfo.github.io/taca/demo/?app=apps/zig/hi.taca)
  ([Zig source](examples/zig/hi/src/main.zig))

## Demo Screenshot

^^^ SEE ACTUAL DEMO LINK JUST ABOVE. ^^^

Screenshot, which is close to half the size of the entire runtime and demo app
combined, when everything is gzipped:

![Taca demo app screenshot with colorful RGB triangle and white spotlight](docs/screenshot.png)

## Dev Notes

Look at package.json under cana for web versions. For simple dev:

```sh
npm run dev
# Separate tab on Rust code change:
npm run pack-dev
```

Or for actual builds:

```sh
npm run preview
# Separate tab, and pick your poison:
npm run build
npm run build-split
```

For native, either build faster:

```sh
cargo run --bin waca --profile release-quick -- run cana/public/hi.taca
```

Or build more optimized:

```sh
cargo run --bin waca --release -- run cana/public/hi.taca
```

## Old Dev Notes

Build normal:

```sh
cargo build --target wasm32-unknown-unknown --release && \
  cp target/wasm32-unknown-unknown/release/taca.wasm static/
```

Build optimized:

```sh
./build-opt.sh
```

Run native:

```sh
cargo run --release -- run docs/demo/apps/zig/hi.taca
```

Run web:

```sh
# First time: npm install -g http-server
https-server .
```

Open: http://127.0.0.1:8080/static/?app=../docs/demo/apps/zig/hi.taca

## Sources

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
