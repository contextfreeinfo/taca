PUB_DIR=../../../web/public/apps/c3

c3c compile --reloc=none --target wasm32 -g0 --link-libc=no --no-entry -Os \
    src/main.c3 --output-dir build && \
wasm-opt -Os build/out.wasm -o build/out.opt.wasm && \
lz4 -f9 build/out.opt.wasm build/fly.taca && \
mkdir -p $PUB_DIR && \
cp build/fly.taca $PUB_DIR/
