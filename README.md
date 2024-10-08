# Taca

A runtime for multimedia wasm apps that runs native and in browsers.

## Live Demos

- [Asteroid field 3D game](https://contextfreeinfo.github.io/taca/demo/?app=apps/c3/fly.taca)
  ([C3 source](examples/c3/fly/src/main.c3))
- [Pixel art platformer demo](https://contextfreeinfo.github.io/taca/demo/?app=apps/nelua/walk.taca) ([Nelua source](examples/nelua/walk/src/main.nelua))
- [Mouse pointer spotlight on RGB triangle](https://contextfreeinfo.github.io/taca/demo/?app=apps/zig/hi2.taca)
  ([Zig source](examples/zig/hi/src/hi2.zig))

## Demo Screenshot

^^^ SEE ACTUAL DEMO LINK JUST ABOVE. ^^^

Screenshot, which is close to half the size of the entire runtime and demo app
combined, when everything is gzipped:

![Taca demo app screenshot with colorful RGB triangle and white spotlight](docs/screenshot.png)

## Dev Notes

### Containers

Containers might make life easier:

```bash
# Get the image
podman pull ghcr.io/contextfreeinfo/taca-dev:latest
# Use the image in this dir
podman run --rm -it -v $PWD:/workspace taca-dev:latest bash
# Run the dev server from the web dir
podman run --rm -it -p 5173:5173 -p 24678:24678 -v $PWD:/workspace taca-dev:latest bash
npm run dev -- --host 0.0.0.0
# Run the preview server from the web dir
podman run --rm -it -p 4173:4173 -v $PWD:/workspace taca-dev:latest bash
npm run preview -- --host 0.0.0.0
```

Or use Docker if you need to.

Even if you don't use a container, the Containerfile gives info on dependencies
for building things.

### Demo app

Just one demo so far, made with Zig and support tools:

```sh
cd examples/zig/hi
./build.sh
```

That puts the built taca app under the top-level web dir for easy access there.

### Native runtime

For native, go back to the top dir and either build faster:

```sh
cargo run --bin taca --profile release-quick -- run web/public/apps/zig/hi.taca
```

Or build more optimized:

```sh
cargo run --bin taca --release -- run web/public/apps/zig/hi.taca
```

The native runtime is pure Rust, but the only demo app so far is in Zig and
more. Maybe it would be nice to make a demo app in Rust sometime to make a
simpler example with few dependencies.

### Web runtime

Look at package.json under web for web versions:

```sh
cd web
```

For simple dev:

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
