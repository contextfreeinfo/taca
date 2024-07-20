naga src/shader.wgsl src/shader.spv && \
spirv-opt -Os src/shader.spv -o src/shader.opt.spv && \
zig build && \
wasm-opt -Os zig-out/bin/hi.wasm -o zig-out/bin/hi.opt.wasm && \
lz4 -f9 zig-out/bin/hi.opt.wasm
# Copy to docs only when we want to publish updates.
# TODO Make a convenience script for that.
# cp zig-out/bin/hi.opt.wasm.lz4 ../../../docs/demo/apps/zig/hi.taca
cp zig-out/bin/hi.opt.wasm.lz4 ../../../cana/public/hi.taca
