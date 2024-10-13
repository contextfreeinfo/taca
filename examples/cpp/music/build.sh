PUB_DIR=../../../web/public/apps/cpp

mkdir -p out &&
wit-bindgen c --out-dir out --no-helpers --no-object-file --rename-world taca \
     src/taca.wit && \
"$WASI_SDK/bin/clang" -Os -s --std=c++23 -Wall -Wextra -Werror \
     -o out/music.wasm src/main.cpp && \
lz4 -f9 out/music.wasm out/music.taca && \
mkdir -p $PUB_DIR && \
cp out/music.taca $PUB_DIR/

# && \
# wasm2wat --generate-names out/music.wasm -o out/music.wat
# wasm-opt -Os out/music.wasm -o out/music.opt.wasm && \
# "$WASI_SDK/bin/clang++" --std=c++23 -fmodules -o out/music.wasm src/main.cpp -x c++-module src/taca.cpp && \
