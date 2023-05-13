const g = @cImport({
    @cInclude("wgpu.h");
    @cInclude("webgpu-headers/webgpu.h");
});

pub fn main() void {
    // Instance.
    const instance = g.wgpuCreateInstance(&g.WGPUInstanceDescriptor{
        .nextInChain = null,
    }) orelse unreachable;
    defer g.wgpuInstanceDrop(instance);
    // Surface.
    const surface = g.wgpuInstanceCreateSurface(
        instance,
        &g.WGPUSurfaceDescriptor{
            .nextInChain = @ptrCast(
                *const g.WGPUChainedStruct,
                &g.WGPUSurfaceDescriptorFromCanvasHTMLSelector{
                    .chain = g.WGPUChainedStruct{
                        .next = null,
                        .sType = g.WGPUSType_SurfaceDescriptorFromCanvasHTMLSelector,
                    },
                    .selector = "",
                },
            ),
            .label = "Surface",
        },
    ) orelse unreachable;
    defer g.wgpuSurfaceDrop(surface);
}
