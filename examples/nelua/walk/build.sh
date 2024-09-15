export WASI_SDK=$HOME/apps/wasi-sdk
nelua --cc="$WASI_SDK/bin/clang" --add-path src -o out/hi.wasm --release \
    src/hi.nelua && \
wasm2wat --generate-names out/hi.wasm -o out/hi.wat && \
lz4 -f9 out/hi.wasm out/hi.taca
