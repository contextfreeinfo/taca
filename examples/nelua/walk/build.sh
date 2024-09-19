PUB_DIR=../../../web/public/apps/nelua
export LUA_PATH=$PWD/src/?.lua
export WASI_SDK=$HOME/apps/wasi-sdk
nelua --cc="$WASI_SDK/bin/clang" --add-path src -o out/walk.wasm --release \
    src/walk.nelua && \
wasm2wat --generate-names out/walk.wasm -o out/walk.wat && \
lz4 -f9 out/walk.wasm out/walk.taca && \
mkdir -p $PUB_DIR && \
cp out/walk.taca $PUB_DIR/
