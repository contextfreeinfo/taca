## Old instructions

To build:

- Download sdl and build and install to vendor/sdl
- Download wasmtime binary and extract to vendor/wasmtime
- Download wgpu-native binary and extract to vendor/wgpu
- Run `./run.sh` from top project dir

I probably should make a build.zig, but this was working for now.

Maybe in the future:

- Download dawn binary and .c files to vendor/dawn
  - From https://github.com/hexops/mach-gpu-dawn
- Download wasmer binary to vendor/wasmer
