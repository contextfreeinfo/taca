# (cd examples/as && npm install && npm run asbuild) && \
(
    cd examples/zig \
    && zig build-exe -I. hello.zig -lc \
        -target wasm32-wasi -O ReleaseSmall -dynamic \
) && \
zig build-exe \
    -Ivendor/sdl/include/SDL2 -Lvendor/sdl/lib \
    -Ivendor/wasmtime/include -Lvendor/wasmtime/lib \
    src/tacana.zig -lSDL2 -lSDL2main -lwasmtime -lc && \
./tacana examples/zig/hello.wasm
