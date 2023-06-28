## Instructions

Original plan was to go through C interfaces in Zig for the implementation as
well. Considering going back to this, but starting with a focus of a simplified
user experience as the initial focus, since raw WebGPU is no fun and I like
small binaries.

SDL also has haptic feedback built in, so that could end up fun.

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
