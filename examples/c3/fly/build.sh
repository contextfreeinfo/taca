PUB_DIR=../../../web/public/apps/c3

dxc -T vs_6_0 -E vertex_main -spirv -Fo build/vertex.spv src/shader.hlsl && \
dxc -T ps_6_0 -E fragment_main -spirv -Fo build/fragment.spv src/shader.hlsl && \
c3c compile --reloc=none --target wasm32 -g0 --link-libc=no --no-entry -Os \
    src/*.c3 --output-dir build && \
wasm-opt -Os build/out.wasm -o build/out.opt.wasm && \
lz4 -f9 build/out.opt.wasm build/fly.taca && \
mkdir -p $PUB_DIR && \
cp build/fly.taca $PUB_DIR/
