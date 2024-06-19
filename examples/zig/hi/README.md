```
naga src/shader.wgsl src/shader.spv && \
    spirv-opt -Os src/shader.spv -o src/shader.opt.spv && \
    zig build && \
    wasm-opt -Os zig-out/bin/hi.wasm -o zig-out/bin/hi.opt.wasm
```
