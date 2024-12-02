export PATH="$WASI_SDK/bin:$PATH"

pub() {
    for dir in dist public; do
        pub_dir=../../../web/$dir/apps/$2
        mkdir -p $pub_dir && \
        cp $1 $pub_dir/
    done
}

odin build src -no-entry-point -o:speed -out:out/guess.wasm \
    -target=freestanding_wasm32 && \
lz4 -f9 out/guess.wasm out/guess.taca && \
pub out/guess.taca odin

# wasm2wat --generate-names out/guess.wasm -o out/guess.wat
