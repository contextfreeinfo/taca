zig build-exe -target wasm32-wasi -O ReleaseSmall --export-table \
    -I ../../include/wgpu-native/ffi -I ../../include/tactic \
    explore-webgpu.zig && \
wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
    explore-webgpu.wasm -o explore-webgpu.wat && \
wasm-opt -O4 -all explore-webgpu.wasm -o explore-webgpu.opt.wasm && \
ls -l explore-webgpu.opt.wasm && \
wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
    explore-webgpu.opt.wasm -o explore-webgpu.opt.wat
