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
        .queue = queue,
        .size = size,
        .swap_chain = swap_chain,
        .surface = surface,
    };

    // Listen
    // TODO Instead send data only and rely on exported named functions?
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
    _ = state;
//             let view = wgpu_native::device::wgpuSwapChainGetCurrentTextureView(self.swap_chain);
//             let encoder = wgpu_native::device::wgpuDeviceCreateCommandEncoder(
//                 self.device,
//                 Some(&native::WGPUCommandEncoderDescriptor {
//                     nextInChain: std::ptr::null(),
//                     label: CStr::from_bytes_with_nul_unchecked(b"Render Encoder\0").as_ptr(),
//                 }),
//             );
//             let render_pass = wgpu_native::command::wgpuCommandEncoderBeginRenderPass(
//                 encoder,
//                 Some(&native::WGPURenderPassDescriptor {
//                     nextInChain: std::ptr::null(),
//                     label: CStr::from_bytes_with_nul_unchecked(b"Render Pass\0").as_ptr(),
//                     colorAttachmentCount: 1,
//                     colorAttachments: &native::WGPURenderPassColorAttachment {
//                         view,
//                         resolveTarget: std::ptr::null_mut(),
//                         loadOp: native::WGPULoadOp_Clear,
//                         storeOp: native::WGPUStoreOp_Store,
//                         clearValue: native::WGPUColor {
//                             r: 0.1,
//                             g: 0.2,
//                             b: 0.3,
//                             a: 1.0,
//                         },
//                     },
//                     depthStencilAttachment: std::ptr::null(),
//                     occlusionQuerySet: std::ptr::null_mut(),
//                     timestampWriteCount: 0,
//                     timestampWrites: std::ptr::null(),
//                 }),
//             );
//             wgpu_native::command::wgpuRenderPassEncoderEnd(render_pass);
//             wgpu_native::device::wgpuTextureViewDrop(view);
//             let command_buffer = wgpu_native::command::wgpuCommandEncoderFinish(
//                 encoder,
//                 Some(&native::WGPUCommandBufferDescriptor {
//                     nextInChain: std::ptr::null(),
//                     label: CStr::from_bytes_with_nul_unchecked(b"Command Buffer\0").as_ptr(),
//                 }),
//             );
//             wgpu_native::device::wgpuQueueSubmit(self.queue, 1, &command_buffer);
//             wgpu_native::device::wgpuSwapChainPresent(self.swap_chain);
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
