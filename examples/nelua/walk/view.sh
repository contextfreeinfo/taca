export LUA_PATH=$PWD/src/?.lua
mkdir -p out
nelua --print-code --add-path src -o out/walk.wasm --release \
    src/walk.nelua > out/code.c
