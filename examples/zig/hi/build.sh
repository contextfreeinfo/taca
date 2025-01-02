build-shader() {
    naga src/$1.wgsl src/$1.spv && \
    spirv-opt -Os src/$1.spv -o src/$1.opt.spv
}

finish-wasm() {
    rm -rf out$1 && \
    mkdir -p out/$1 && \
    wasm-opt -Os zig-out/bin/$1.wasm -o out/$1/app.wasm && \
    (cd out/$1 && zip -r ../$1.taca .) && \
    pub out/$1.taca zig
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
    finish-wasm hi && finish-wasm hi2 && \
    ls -l out/*.taca
