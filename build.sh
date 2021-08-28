# (cd examples/as && npm install && npx run build) && \
(
    cd examples/zig \
    && zig build-exe -I. hello.zig -target wasm32-freestanding -dynamic \
) && \
zig build-exe \
    -Ivendor/sdl/include/SDL2 -Lvendor/sdl/lib \
    -Ivendor/wasmtime/include -Lvendor/wasmtime/lib \
    src/tacana.zig -lSDL2 -lSDL2main -lwasmtime -lc && \
./tacana examples/zig/hello.wasm
