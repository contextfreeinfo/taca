pub() {
    for dir in dist public; do
        pub_dir=../../../web/$dir/apps/$2
        mkdir -p $pub_dir && \
        cp $1 $pub_dir/
    done
}

rm -rf out && \
mkdir -p out/bundle && \
dxc -T vs_6_0 -E vertex_main -spirv -Fo out/vertex.spv src/shader.hlsl && \
dxc -T ps_6_0 -E fragment_main -spirv -Fo out/fragment.spv src/shader.hlsl && \
c3c compile --reloc=none --target wasm32 -g0 --link-libc=no --no-entry -Os \
    src/*.c3 --output-dir out && \
wasm-opt -Os out/out.wasm -o out/bundle/app.wasm && \
(cd out/bundle && zip -r ../fly.taca .) && \
ls -l out/*.taca && \
pub out/fly.taca c3

# Currently breaks uniforms handling. TODO Figure out the issue.
# spirv-opt -Os out/vertex.spv -o out/vertex.opt.spv && \
# spirv-opt -Os out/fragment.spv -o out/fragment.opt.spv && \
