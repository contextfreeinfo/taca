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

### Web

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

Demo links:

- Dev: http://localhost:5173/?app=apps/zig/hi.taca
- Dist: http://localhost:4173/?app=apps/zig/hi.taca

### Native

For native, either build faster:

```sh
cargo run --bin waca --profile release-quick -- run cana/public/apps/zig/hi.taca
```

Or build more optimized:

```sh
cargo run --bin waca --release -- run cana/public/apps/zig/hi.taca
```

And when interested in updating the demo docs:

```sh
cp dist/taca.js dist/taca.wasm ../docs/demo/
cp dist/hi.taca ../docs/demo/apps/zig/
```

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
