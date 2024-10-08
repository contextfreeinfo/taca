PUB_DIR=../../../web/public/apps/nelua
export LUA_PATH=$PWD/src/?.lua

mkdir -p out && \
naga src/shader.wgsl out/shader.spv && \
nelua --cc="$WASI_SDK/bin/clang" --add-path src -o out/walk.wasm --release \
    src/main.nelua && \
lz4 -f9 out/walk.wasm out/walk.taca && \
mkdir -p $PUB_DIR && \
cp out/walk.taca $PUB_DIR/

# Inspection
# wasm2wat --generate-names out/walk.wasm -o out/walk.wat && \

# Only saving a tiny bit here:
# wasm-opt -Os out/walk.wasm -o out/walk.opt.wasm && \
# lz4 -f9 out/walk.opt.wasm out/walk.taca && \
