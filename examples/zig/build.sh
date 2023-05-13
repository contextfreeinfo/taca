zig build-exe -target wasm32-wasi -O ReleaseSmall -I ../wgpu-native/ffi \
    explore-webgpu.zig && \
wasm-opt -O4 -all explore-webgpu.wasm -o explore-webgpu.opt.wasm && \
wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
    explore-webgpu.opt.wasm -o explore-webgpu.wat
