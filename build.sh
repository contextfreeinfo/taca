# (cd examples/as && npm install && npx run build) && \
(
    cd examples/zig \
    && zig build-exe -I. hello.zig -lc \
        -target wasm32-wasi -O ReleaseSmall -dynamic \
) && \
zig build-exe \
    -Ivendor/sdl/include/SDL2 -Lvendor/sdl/lib \
    -Ivendor/wasmtime/include -Lvendor/wasmtime/lib \
    src/tactic.zig -lSDL2 -lSDL2main -lwasmtime -lc && \
./tactic examples/zig/hello.wasm
