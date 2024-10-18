pub() {
    for dir in dist public; do
        pub_dir=../../../web/$dir/apps/$2
        mkdir -p $pub_dir && \
        cp $1 $pub_dir/
    done
}

mkdir -p out &&
xxd -i src/musicbox.mp3 > out/musicbox-data.c && \
"$WASI_SDK/bin/clang" --std=c++23 -Os -s -Wall -Wextra -Werror -Isrc -Iout \
     -Wno-missing-field-initializers \
     -o out/music.wasm src/main.cpp && \
lz4 -f9 out/music.wasm out/music.taca && \
pub out/music.taca cpp

# && \
# wasm2wat --generate-names out/music.wasm -o out/music.wat
# wasm-opt -Os out/music.wasm -o out/music.opt.wasm && \
# "$WASI_SDK/bin/clang++" --std=c++23 -fmodules -o out/music.wasm src/main.cpp -x c++-module src/taca.cpp && \
# wit-bindgen c --out-dir out --no-helpers --no-object-file --rename-world taca src/taca.wit && \
