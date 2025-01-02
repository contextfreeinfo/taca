export LUA_PATH=$PWD/src/?.lua

pub() {
    for dir in dist public; do
        pub_dir=../../../web/$dir/apps/$2
        mkdir -p $pub_dir && \
        cp $1 $pub_dir/
    done
}

rm -rf out && \
mkdir -p out/bundle && \
naga src/shader.wgsl out/shader.spv && \
nelua --cc="$WASI_SDK/bin/clang" --add-path src --release \
    -o out/bundle/app.wasm src/main.nelua && \
(cd out/bundle && zip -r ../walk.taca .) && \
ls -l out/*.taca && \
pub out/walk.taca nelua

# Inspection
# wasm2wat --generate-names out/walk.wasm -o out/walk.wat && \

# Only saving a tiny bit here:
# wasm-opt -Os out/walk.wasm -o out/walk.opt.wasm && \
# lz4 -f9 out/walk.opt.wasm out/walk.taca && \
