time (
    (
        cd examples/zig &&
        time sh build.sh
    ) &&
    time cargo build --release &&
    ls -lh target/release/tacana &&
    RUST_BACKTRACE=1 /usr/bin/time -v target/release/tacana run examples/zig/explore-webgpu.opt.wasm
)

# mkdir -p notes &&
# cp examples/zig/explore-webgpu.opt.wasm target/release/tacana notes &&
# gzip -f notes/explore-webgpu.opt.wasm notes/tacana &&
# ls -l notes/explore-webgpu.opt.wasm.gz &&
# ls -lh notes/tacana.gz
