const assert = @import("std").debug.assert;
const p = @import("./pipeline.zig");
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
    errdefer g.wgpuInstanceDrop(instance);

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
    errdefer g.wgpuSurfaceDrop(surface);

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
    errdefer g.wgpuAdapterDrop(adapter);

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
    errdefer g.wgpuDeviceDrop(device);
    const queue = g.wgpuDeviceGetQueue(device);

    // Swap Chain
    const size = t.tac_windowInnerSize();
    const format = g.wgpuSurfaceGetPreferredFormat(surface, adapter);
    const pipeline = p.buildPipeline(device, format);
    const swap_chain = createSwapChain(.{
        .device = device,
        .format = format,
        .size = size,
        .surface = surface,
    });
    global_state = .{
        .adapter = adapter,
        .device = device,
        .format = format,
        .instance = instance,
        .pipeline = pipeline,
        .queue = queue,
        .size = size,
        .swap_chain = swap_chain,
        .surface = surface,
    };

    // Listen
    // Probably smaller binaries with this instead of exported functions for
    // each even type?
    t.tac_windowListen(windowListen, null);
}

fn windowListen(event_type: t.tac_WindowEventType, unused: ?*anyopaque) callconv(.C) void {
    _ = unused;
    switch (event_type) {
        t.tac_WindowEventType_Close => windowClose(&global_state),
        t.tac_WindowEventType_Redraw => windowRedraw(&global_state),
        t.tac_WindowEventType_Resize => windowResize(&global_state),
        else => unreachable,
    }
}

fn windowClose(state: *State) void {
    g.wgpuDeviceDrop(state.device);
    g.wgpuAdapterDrop(state.adapter);
    g.wgpuSurfaceDrop(state.surface);
    g.wgpuInstanceDrop(state.instance);
}

fn windowRedraw(state: *State) void {
    const view = g.wgpuSwapChainGetCurrentTextureView(state.swap_chain) orelse unreachable;
    const encoder = g.wgpuDeviceCreateCommandEncoder(
        state.device,
        &g.WGPUCommandEncoderDescriptor{
            .nextInChain = null,
            .label = null,
        },
    ) orelse unreachable;
    const render_pass = g.wgpuCommandEncoderBeginRenderPass(
        encoder,
        &g.WGPURenderPassDescriptor{
            .nextInChain = null,
            .label = null,
            .colorAttachmentCount = 1,
            .colorAttachments = &g.WGPURenderPassColorAttachment{
                .view = view,
                .resolveTarget = null,
                .loadOp = g.WGPULoadOp_Clear,
                .storeOp = g.WGPUStoreOp_Store,
                .clearValue = .{
                    .r = 0.1,
                    .g = 0.2,
                    .b = 0.3,
                    .a = 1.0,
                },
            },
            .depthStencilAttachment = null,
            .occlusionQuerySet = null,
            .timestampWriteCount = 0,
            .timestampWrites = null,
        },
    ) orelse unreachable;
    g.wgpuRenderPassEncoderSetPipeline(render_pass, state.pipeline);
    g.wgpuRenderPassEncoderDraw(render_pass, 3, 1, 0, 0);
    g.wgpuRenderPassEncoderEnd(render_pass);
    g.wgpuTextureViewDrop(view);
    const command_buffer = g.wgpuCommandEncoderFinish(
        encoder,
        &g.WGPUCommandBufferDescriptor{
            .nextInChain = null,
            .label = null,
        },
    ) orelse unreachable;
    g.wgpuQueueSubmit(state.queue, 1, &command_buffer);
    g.wgpuSwapChainPresent(state.swap_chain);
}

fn windowResize(state: *State) void {
    const size = t.tac_windowInnerSize();
    if (size.x > 0 and size.y > 0) {
        state.size = size;
        g.wgpuSwapChainDrop(state.swap_chain);
    }
    state.swap_chain = createSwapChain(.{
        .device = state.device,
        .format = state.format,
        .size = state.size,
        .surface = state.surface,
    });
}

const State = struct {
    adapter: g.WGPUAdapter,
    device: g.WGPUDevice,
    format: g.WGPUTextureFormat,
    instance: g.WGPUInstance,
    pipeline: g.WGPURenderPipeline,
    queue: g.WGPUQueue,
    size: t.tac_Vec2,
    swap_chain: g.WGPUSwapChain,
    surface: g.WGPUSurface,
};

var global_state: State = undefined;

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
