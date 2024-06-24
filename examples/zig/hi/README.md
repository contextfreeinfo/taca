```
naga src/shader.wgsl src/shader.spv && \
    spirv-opt -Os src/shader.spv -o src/shader.opt.spv && \
    zig build && \
    wasm-opt -Os zig-out/bin/hi.wasm -o ../../../docs/demo/apps/zig/hi.taca
```

Also

```
dxc -T vs_6_0 -E vs_main -spirv -Fo src/vs.spv src/shader.hlsl
dxc -T ps_6_0 -E fs_main -spirv -Fo src/fs.spv src/shader.hlsl
```
