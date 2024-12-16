pub() {
    for dir in dist public; do
        pub_dir=../../../web/$dir/apps/$2
        mkdir -p $pub_dir && \
        cp $1 $pub_dir/
    done
}

mkdir -p out && \
naga src/shader.wgsl out/shader.spv && \
nimble build -d:release && \
lz4 -f9 out/textbox.wasm out/textbox.taca && \
pub out/textbox.taca nim
