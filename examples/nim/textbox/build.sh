pub() {
    for dir in dist public; do
        pub_dir=../../../web/$dir/apps/$2
        mkdir -p $pub_dir && \
        cp $1 $pub_dir/
    done
}

rm -rf out && \
mkdir -p out/bundle && \
naga src/shader.wgsl out/shader.spv && \
nimble build -d:release && \
cp out/textbox.wasm out/bundle/app.wasm && \
(cd out/bundle && zip -r ../textbox.taca .) && \
ls -l out/*.taca && \
pub out/textbox.taca nim
