build-shader() {
    naga src/$1.wgsl src/$1.spv && \
    spirv-opt -Os src/$1.spv -o src/$1.opt.spv
}

finish-wasm() {
    wasm-opt -Os zig-out/bin/$1.wasm -o zig-out/bin/$1.opt.wasm && \
    lz4 -f9 zig-out/bin/$1.opt.wasm zig-out/bin/$1.taca && \
    pub zig-out/bin/$1.taca zig
}

pub() {
    for dir in dist public; do
        pub_dir=../../../web/$dir/apps/$2
        mkdir -p $pub_dir && \
        cp $1 $pub_dir/
    done
}

build-shader shader && build-shader shader2 && \
    zig build && \
    finish-wasm hi && finish-wasm hi2
