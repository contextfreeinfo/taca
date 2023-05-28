time (
    (
        cd examples/zig &&
        time sh build.sh
    ) &&
    time cargo build --release &&
    RUST_BACKTRACE=1 /usr/bin/time -v target/release/tacana run examples/zig/explore-webgpu.opt.wasm
)
