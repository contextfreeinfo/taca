zig build-obj -target wasm32-wasi -O ReleaseSmall \
    -I ../../include/wgpu-native/ffi -I ../../include/tacana \
    explore-webgpu.zig && \
zig clang -target wasm32-wasi -nostdlib -Wl,--export-table \
    explore-webgpu.o -o explore-webgpu.wasm && \
wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
    explore-webgpu.wasm -o explore-webgpu.wat && \
wasm-opt -O4 -all explore-webgpu.wasm -o explore-webgpu.opt.wasm && \
wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
    explore-webgpu.opt.wasm -o explore-webgpu.opt.wat
