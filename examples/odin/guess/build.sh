export PATH="$WASI_SDK/bin:$PATH"

pub() {
    for dir in dist public; do
        pub_dir=../../../web/$dir/apps/$2
        mkdir -p $pub_dir && \
        cp $1 $pub_dir/
    done
}

(cd ../../nim/textbox && bash build-lib.sh) && \
rm -rf out/bundle && \
mkdir -p out/bundle/taca/lib && \
cp ../../nim/textbox/out/textbox.wasm out/bundle/taca/lib/ && \
odin build src -no-entry-point -o:speed -out:out/bundle/taca/app.wasm \
    -target=freestanding_wasm32 && \
rm -f out/guess.taca && \
(cd out/bundle && zip -r ../guess.taca .) && \
ls -l out/guess.taca && \
pub out/guess.taca odin

# unzip -lv out/guess.taca && \
# lz4 -f9 out/guess.wasm out/guess.taca && \
# wasm2wat --generate-names out/guess.wasm -o out/guess.wat
