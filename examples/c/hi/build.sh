mkdir -p out/hi && \
wit-bindgen c ../../../taca.wit --out-dir out && \
"$WASI_SDK/bin/clang" \
    -Os -s -Wall -Wextra -Werror \
    -Wno-missing-field-initializers -Wno-unused-variable \
    -Wno-unused-parameter -fno-exceptions \
    -mexec-model=reactor \
    -Iout -Isrc out/taca.c src/app.c out/taca_component_type.o \
    -o out/hi.wasm && \
wasm-tools component new out/hi.wasm -o out/hi/app.wasm && \
cp app.json out/hi/ && \
../../../native/target/release/taca --build out/hi

# wit-bindgen csharp --runtime mono ../../../taca.wit --out-dir out/csharp-mono && \
# wit-bindgen csharp --runtime native-aot ../../../taca.wit --out-dir out/csharp-native-aot && \
# wit-bindgen markdown ../../../taca.wit --out-dir out/markdown && \
# wit-bindgen moonbit ../../../taca.wit --out-dir out/moonbit && \
# wit-bindgen rust ../../../taca.wit --out-dir out/rust && \

# wasm-tools component new out/hi.wasm -o out/bundle/app.wasm --adapt ../../../ignore/wasi_snapshot_preview1.reactor.wasm
# wit-bindgen c ../../../taca.wit --out-dir out --no-object-file && \
# wasm-tools parse ignore/taca_component_type.o -t
# wasm-tools component wit ignore/taca_component_type.o
# wit-bindgen rust ../../../taca.wit --out-dir ignore
