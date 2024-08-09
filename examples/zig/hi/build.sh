build() {
    naga src/shader.wgsl src/shader.spv && \
    spirv-opt -Os src/shader.spv -o src/shader.opt.spv && \
    zig build && \
    wasm-opt -Os zig-out/bin/hi.wasm -o zig-out/bin/hi.opt.wasm && \
    lz4 -f9 zig-out/bin/hi.opt.wasm zig-out/bin/hi.opt.wasm.lz4
}

place() {
    PUB_DIR=../../../web/public/apps/zig
    mkdir -p $PUB_DIR
    cp zig-out/bin/hi.opt.wasm.lz4 $PUB_DIR/hi.taca
}

build && place
