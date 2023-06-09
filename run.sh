time (
    (
        cd examples/zig &&
        time sh build.sh
    ) &&
    time cargo build --release &&
    ls -lh target/release/tactic &&
    RUST_BACKTRACE=1 /usr/bin/time -v target/release/tactic run examples/zig/explore-webgpu.opt.wasm
)

# mkdir -p notes &&
# cp examples/zig/explore-webgpu.opt.wasm target/release/tactic notes &&
# gzip -f notes/explore-webgpu.opt.wasm notes/tactic &&
# ls -l notes/explore-webgpu.opt.wasm.gz &&
# ls -lh notes/tactic.gz
