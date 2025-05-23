mkdir -p out && \
wit-bindgen c ../../../taca.wit --out-dir out && \
"$WASI_SDK/bin/clang" \
    -Os -s -Wall -Wextra -Werror \
    -Wno-missing-field-initializers -Wno-unused-variable \
    -Wno-unused-parameter -fno-exceptions \
    -mexec-model=reactor \
    -Iout -Isrc out/taca.c src/app.c out/taca_component_type.o \
    -o out/hi.wasm && \
wasm-tools component new out/hi.wasm -o out/hi-component.wasm

# wit-bindgen c ../../../taca.wit --out-dir out --no-object-file && \
# wasm-tools parse ignore/taca_component_type.o -t
# wasm-tools component wit ignore/taca_component_type.o
# wit-bindgen rust ../../../taca.wit --out-dir ignore
