// time zig build-exe -target wasm32-wasi -O ReleaseSmall -I ../wgpu-native/ffi explore-webgpu.zig && wasm2wat explore-webgpu.wasm -o explore-webgpu.wat

const g = @cImport({
    @cInclude("wgpu.h");
    @cInclude("webgpu-headers/webgpu.h");
});

pub fn main() void {
    const instance = g.wgpuCreateInstance(&g.WGPUInstanceDescriptor{
        .nextInChain = null,
    });
    defer g.wgpuInstanceDrop(instance);
}
