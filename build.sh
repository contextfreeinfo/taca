(cd examples && zig build-exe hello.zig -target wasm32-freestanding -dynamic) \
&& zig build-exe \
    -Ivendor/sdl/include -Lvendor/sdl/lib \
    -Ivendor/wasmtime/include -Lvendor/wasmtime/lib \
    src/tacana.zig -lSDL2 -lSDL2main -lwasmtime -lc \
&& ./tacana
