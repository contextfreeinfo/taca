PUB_DIR=../../../web/public/apps/cpp

mkdir -p out &&
"$WASI_SDK/bin/clang" --std=c++23 -Os -s -o out/music.wasm src/main.cpp && \
lz4 -f9 out/music.wasm out/music.taca && \
mkdir -p $PUB_DIR && \
cp out/music.taca $PUB_DIR/

# && \
# wasm2wat --generate-names out/music.wasm -o out/music.wat
# wasm-opt -Os out/music.wasm -o out/music.opt.wasm && \
