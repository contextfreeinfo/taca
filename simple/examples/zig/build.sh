# naga shader.wgsl shader.spv &&
# naga shader.spv shader-out.wgsl &&
# node minify.js &&
zig build-exe -target wasm32-wasi -O ReleaseSmall --export-table \
    -I ../../../include/wgpu-native -I ../../../include/taca \
    explore-taca.zig && \
wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
    explore-taca.wasm -o explore-taca.wat && \
wasm-opt -O4 -all explore-taca.wasm -o explore-taca.opt.wasm && \
ls -l explore-taca.opt.wasm && \
wasm2wat --generate-names --fold-exprs --inline-exports --inline-imports \
    explore-taca.opt.wasm -o explore-taca.opt.wat
