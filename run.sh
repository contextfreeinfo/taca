time (
    (
        cd examples/zig &&
        time sh build.sh
    ) &&
    time cargo build --release &&
    /usr/bin/time -v target/release/tacana run examples/zig/explore-webgpu.opt.wasm
)
