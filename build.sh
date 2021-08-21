(
    cd examples \
    && zig build-exe -Izig zig/hello.zig -target wasm32-freestanding -dynamic \
) \
&& zig build-exe \
    -Ivendor/sdl/include/SDL2 -Lvendor/sdl/lib \
    -Ivendor/wasmtime/include -Lvendor/wasmtime/lib \
    src/tacana.zig -lSDL2 -lSDL2main -lwasmtime -lc \
&& ./tacana 
