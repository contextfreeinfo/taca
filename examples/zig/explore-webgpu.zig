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

    // Swap Chain
    const size = t.tac_windowInnerSize();
    const format = g.wgpuSurfaceGetPreferredFormat(surface, adapter);
    const swap_chain = createSwapChain(.{
        .device = device,
        .format = format,
        .size = size,
        .surface = surface,
    });
    state = .{
        .device = device,
        .format = format,
        .queue = queue,
        .size = size,
        .swap_chain = swap_chain,
        .surface = surface,
    };

    // Listen
    t.tac_windowListen(windowListen, null);
}

fn windowListen(event_type: t.tac_WindowEventType, unused: ?*anyopaque) callconv(.C) void {
    _ = event_type;
    _ = unused;
}

const State = struct {
    device: g.WGPUDevice,
    format: g.WGPUTextureFormat,
    queue: g.WGPUQueue,
    size: t.tac_Vec2,
    swap_chain: g.WGPUSwapChain,
    surface: g.WGPUSurface,
};

var state: State = undefined;

const CreateSwapChainData = struct {
    device: g.WGPUDevice,
    format: g.WGPUTextureFormat,
    size: t.tac_Vec2,
    surface: g.WGPUSurface,
};

fn createSwapChain(data: CreateSwapChainData) g.WGPUSwapChain {
    const swap_chain = g.wgpuDeviceCreateSwapChain(
        data.device,
        data.surface,
        &g.WGPUSwapChainDescriptor{
            .nextInChain = null,
            .label = null,
            .usage = g.WGPUTextureUsage_RenderAttachment,
            .format = data.format,
            .width = @intCast(u32, data.size.x),
            .height = @intCast(u32, data.size.y),
            .presentMode = g.WGPUPresentMode_Fifo,
        },
    ) orelse unreachable;
    return swap_chain;
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
