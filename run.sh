time (
    (
        cd examples/zig &&
        time sh build.sh
    ) &&
    time cargo build --release &&
    ls -lh target/release/taca &&
    RUST_BACKTRACE=1 /usr/bin/time -v target/release/taca run examples/zig/explore-webgpu.opt.wasm
)

# mkdir -p notes &&
# cp examples/zig/explore-webgpu.opt.wasm target/release/taca notes &&
# gzip -f notes/explore-webgpu.opt.wasm notes/taca &&
# ls -l notes/explore-webgpu.opt.wasm.gz &&
# ls -lh notes/taca.gz
