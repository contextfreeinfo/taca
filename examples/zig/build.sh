# naga shader.wgsl shader.spv &&
# naga shader.spv shader-out.wgsl &&
node minify.js &&
zig build-exe -target wasm32-wasi -O ReleaseSmall --export-table \
    -dynamic -rdynamic \
    -I ../../include/wgpu-native \
    -I ../../include/wgpu-native/webgpu-headers \
    -I ../../include/taca \
    explore-webgpu.zig && \
wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
    explore-webgpu.wasm -o explore-webgpu.wat && \
wasm-opt -O4 -all explore-webgpu.wasm -o explore-webgpu.opt.wasm && \
ls -l explore-webgpu.opt.wasm && \
wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
    explore-webgpu.opt.wasm -o explore-webgpu.opt.wat
