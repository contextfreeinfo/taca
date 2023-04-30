@REM pushd examples\as
@REM npm install
@REM npm run build
@REM popd

pushd examples\zig
zig build-exe -I. hello.zig -target wasm32-freestanding -dynamic
popd

zig build-exe ^
    -Ivendor/sdl/include -Lvendor/sdl/lib/x64 ^
    -Ivendor/wasmtime/include -Lvendor/wasmtime/lib ^
    src/tacana.zig -lSDL2 -lSDL2main -lwasmtime -lbcrypt -lc -lole32 -lucrt -luserenv -lvcruntime -lws2_32
copy vendor\sdl\lib\x64\SDL2.dll .

tacana examples\zig\hello.wasm
