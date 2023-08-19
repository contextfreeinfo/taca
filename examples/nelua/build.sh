CC=$WASI_SDK_PATH/bin/clang
# nelua --cc="$CC" --release main.nelua --output main.wasm
nelua --cc="$CC" main.nelua --output main.wasm
wasm2wat --generate-names main.wasm -o main.wat
