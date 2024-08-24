build-shader() {
    naga src/$1.wgsl src/$1.spv && \
    spirv-opt -Os src/$1.spv -o src/$1.opt.spv
}

finish-wasm() {
    PUB_DIR=../../../web/public/apps/zig
    wasm-opt -Os zig-out/bin/$1.wasm -o zig-out/bin/$1.opt.wasm && \
    lz4 -f9 zig-out/bin/$1.opt.wasm zig-out/bin/$1.opt.wasm.lz4 && \
    mkdir -p $PUB_DIR && \
    cp zig-out/bin/$1.opt.wasm.lz4 $PUB_DIR/$1.taca
}

build-shader shader && build-shader shader2 && \
    zig build && \
    finish-wasm hi && finish-wasm hi2
