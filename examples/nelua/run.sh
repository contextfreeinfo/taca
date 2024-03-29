CC=$WASI_SDK_PATH/bin/clang
node minify.js && \
    nelua --cc="$CC" --release main.nelua --output main.wasm && \
    wasm2wat --generate-names main.wasm -o main.wat && \
    ls -l && \
    time ../../target/release/taca run main.wasm

# nelua --cc="$CC" main.nelua --output main.wasm
