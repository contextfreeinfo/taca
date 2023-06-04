// Based largely on:
// https://github.com/eliemichel/LearnWebGPU-Code/blob/step033/main.cpp

const std = @import("std");
const assert = std.debug.assert;
const d = @import("./data.zig");
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
    var request_adapter_callback_data = RequestAdapterCallbackData{
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
        &request_adapter_callback_data,
    );
    const adapter = request_adapter_callback_data.adapter orelse unreachable;
    var supported_limits = std.mem.zeroInit(g.WGPUSupportedLimits, .{});
    _ = g.wgpuAdapterGetLimits(adapter, &supported_limits) or unreachable;
    errdefer g.wgpuAdapterDrop(adapter);

    // Device & Queue
    var request_device_callback_data = RequestDeviceCallbackData{
        .adapter = adapter,
        .surface = surface,
    };
    const required_limits = g.WGPURequiredLimits{
        .nextInChain = null,
        .limits = std.mem.zeroInit(g.WGPULimits, .{
            .maxVertexAttributes = 2,
            .maxVertexBuffers = 1,
            .maxBufferSize = 6 * 5 * @sizeOf(f32),
            .maxVertexBufferArrayStride = 5 * @sizeOf(f32),
            .minStorageBufferOffsetAlignment = supported_limits.limits.minStorageBufferOffsetAlignment,
            .minUniformBufferOffsetAlignment = supported_limits.limits.minUniformBufferOffsetAlignment,
            .maxInterStageShaderComponents = 3,
        }),
    };
    g.wgpuAdapterRequestDevice(
        adapter,
        &g.WGPUDeviceDescriptor{
            .nextInChain = null,
            .label = null,
            .requiredFeaturesCount = 0,
            .requiredFeatures = null,
            .requiredLimits = &required_limits,
            .defaultQueue = std.mem.zeroInit(g.WGPUQueueDescriptor, .{}),
        },
        requestDeviceCallback,
        &request_device_callback_data,
    );
    const device = request_device_callback_data.device orelse unreachable;
    errdefer g.wgpuDeviceDrop(device);
    g.wgpuDeviceSetUncapturedErrorCallback(device, deviceUncapturedErrorCallback, null);
    const queue = g.wgpuDeviceGetQueue(device);

    // Buffers
    const vertex_buffer = g.wgpuDeviceCreateBuffer(
        device,
        &g.WGPUBufferDescriptor{
            .nextInChain = null,
            .label = null,
            .usage = g.WGPUBufferUsage_CopyDst | g.WGPUBufferUsage_Vertex,
            .size = d.vertex_data_size,
            .mappedAtCreation = false,
        },
    );
    g.wgpuQueueWriteBuffer(queue, vertex_buffer, 0, &d.vertex_data, d.vertex_data_size);

    const index_buffer = g.wgpuDeviceCreateBuffer(
        device,
        &g.WGPUBufferDescriptor{
            .nextInChain = null,
            .label = null,
            .usage = g.WGPUBufferUsage_CopyDst | g.WGPUBufferUsage_Index,
            .size = @sizeOf(@TypeOf(d.index_data)),
            .mappedAtCreation = false,
        },
    );
    g.wgpuQueueWriteBuffer(queue, index_buffer, 0, &d.index_data, @sizeOf(@TypeOf(d.index_data)));

    const point_buffer = g.wgpuDeviceCreateBuffer(
        device,
        &g.WGPUBufferDescriptor{
            .nextInChain = null,
            .label = null,
            .usage = g.WGPUBufferUsage_CopyDst | g.WGPUBufferUsage_Vertex,
            .size = @sizeOf(@TypeOf(d.point_data)),
            .mappedAtCreation = false,
        },
    );
    g.wgpuQueueWriteBuffer(queue, point_buffer, 0, &d.point_data, @sizeOf(@TypeOf(d.point_data)));

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
        .index_buffer = index_buffer,
        .instance = instance,
        .pipeline = pipeline,
        .point_buffer = point_buffer,
        .queue = queue,
        .size = size,
        .swap_chain = swap_chain,
        .surface = surface,
        .vertex_buffer = vertex_buffer,
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
                    .r = 0.05,
                    .g = 0.05,
                    .b = 0.05,
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
    // g.wgpuRenderPassEncoderSetVertexBuffer(render_pass, 0, state.vertex_buffer, 0, d.vertex_data_size);
    // g.wgpuRenderPassEncoderDraw(render_pass, d.vertex_count, 1, 0, 0);
    g.wgpuRenderPassEncoderSetVertexBuffer(render_pass, 0, state.point_buffer, 0, @sizeOf(@TypeOf(d.point_data)));
    g.wgpuRenderPassEncoderSetIndexBuffer(render_pass, state.index_buffer, g.WGPUIndexFormat_Uint16, 0, @sizeOf(@TypeOf(d.index_data)));
    g.wgpuRenderPassEncoderDrawIndexed(render_pass, d.index_data.len, 1, 0, 0, 0);
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
    index_buffer: g.WGPUBuffer,
    instance: g.WGPUInstance,
    pipeline: g.WGPURenderPipeline,
    point_buffer: g.WGPUBuffer,
    queue: g.WGPUQueue,
    size: t.tac_Vec2,
    swap_chain: g.WGPUSwapChain,
    surface: g.WGPUSurface,
    vertex_buffer: g.WGPUBuffer,
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

fn deviceUncapturedErrorCallback(
    status: g.WGPURequestDeviceStatus,
    message: [*c]const u8,
    userdata: ?*anyopaque,
) callconv(.C) void {
    // TODO Once we can actually call this.
    _ = status;
    _ = message;
    _ = userdata;
}
