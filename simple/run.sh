# (cd examples/as && npm install && npx run build) && \
time (
    (
        cd examples/zig &&
        time sh build.sh
    ) && \
    time zig build-exe -O Debug \
        -Ivendor/sdl/include/SDL2 -Lvendor/sdl/lib \
        -Ivendor/wgpu -Lvendor/wgpu \
        -Ivendor/wasmtime/include -Lvendor/wasmtime/lib \
        src/taca.zig -lSDL2 -lSDL2main -lwasmtime -lwgpu_native -lunwind -lc && \
    /usr/bin/time -v ./taca examples/zig/explore-taca.opt.wasm
)
