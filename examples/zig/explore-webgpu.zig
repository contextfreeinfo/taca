const assert = @import("std").debug.assert;
const g = @cImport({
    @cInclude("wgpu.h");
    @cInclude("webgpu-headers/webgpu.h");
});
const t = @cImport({
    @cInclude("tacana.h");
});

pub fn main() void {
    // Instance
    const instance = g.wgpuCreateInstance(&g.WGPUInstanceDescriptor{
        .nextInChain = null,
    }) orelse unreachable;
    defer g.wgpuInstanceDrop(instance);

    // Surface
    const surface = g.wgpuInstanceCreateSurface(
        instance,
        &g.WGPUSurfaceDescriptor{
            .nextInChain = @ptrCast(
                *const g.WGPUChainedStruct,
                &g.WGPUSurfaceDescriptorFromCanvasHTMLSelector{
                    .chain = .{
                        .next = null,
                        .sType = g.WGPUSType_SurfaceDescriptorFromCanvasHTMLSelector,
                    },
                    .selector = "",
                },
            ),
            .label = null,
        },
    ) orelse unreachable;
    defer g.wgpuSurfaceDrop(surface);

    // Adapter
    // This only works because the callback is effectively synchronous.
    // Otherwise, we'd need to allocate on the heap or global.
    var requestAdapterCallbackData = RequestAdapterCallbackData{
        .instance = instance,
        .surface = surface,
    };
    g.wgpuInstanceRequestAdapter(
        instance,
        &g.WGPURequestAdapterOptions{
            .nextInChain = null,
            .compatibleSurface = surface,
            .powerPreference = g.WGPUPowerPreference_Undefined,
            .forceFallbackAdapter = false,
        },
        requestAdapterCallback,
        &requestAdapterCallbackData,
    );
    const adapter = requestAdapterCallbackData.adapter orelse unreachable;
    defer g.wgpuAdapterDrop(adapter);

    // Device & Queue
    var requestDeviceCallbackData = RequestDeviceCallbackData{
        .adapter = adapter,
        .surface = surface,
    };
    g.wgpuAdapterRequestDevice(
        adapter,
        null,
        requestDeviceCallback,
        &requestDeviceCallbackData,
    );
    const device = requestDeviceCallbackData.device orelse unreachable;
    defer g.wgpuDeviceDrop(device);
    const queue = g.wgpuDeviceGetQueue(device);
    _ = queue;

    // Swap Chain
    const size = t.tac_windowGetSize();
    const format = g.wgpuSurfaceGetPreferredFormat(surface, adapter);
    _ = size;
    _ = format;
//                 let size = window.inner_size();
//                 let format = wgpu_native::wgpuSurfaceGetPreferredFormat(surface, adapter);
//                 let mut state = State {
//                     surface,
//                     device,
//                     queue,
//                     format,
//                     size,
//                     swap_chain: std::ptr::null_mut(),
//                     window,
//                 };
//                 state.create_swap_chain();
}

// Adapter

const RequestAdapterCallbackData = struct {
    adapter: ?g.WGPUAdapter = null,
    instance: g.WGPUInstance,
    surface: g.WGPUSurface,
};

fn requestAdapterCallback(
    status: g.WGPURequestAdapterStatus,
    adapter: g.WGPUAdapter,
    message: [*c]const u8,
    userdata: ?*anyopaque,
) callconv(.C) void {
    assert(status == g.WGPURequestDeviceStatus_Success);
    _ = message;
    var data = @ptrCast(
        *RequestAdapterCallbackData,
        @alignCast(@alignOf(*RequestAdapterCallbackData), userdata),
    );
    data.adapter = adapter;
}

// Device

const RequestDeviceCallbackData = struct {
    adapter: g.WGPUAdapter,
    device: ?g.WGPUDevice = null,
    surface: g.WGPUSurface,
};

fn requestDeviceCallback(
    status: g.WGPURequestDeviceStatus,
    device: g.WGPUDevice,
    message: [*c]const u8,
    userdata: ?*anyopaque,
) callconv(.C) void {
    assert(status == g.WGPURequestDeviceStatus_Success);
    _ = message;
    var data = @ptrCast(
        *RequestDeviceCallbackData,
        @alignCast(@alignOf(*RequestDeviceCallbackData), userdata),
    );
    data.device = device;
}
