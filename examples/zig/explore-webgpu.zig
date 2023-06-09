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
    @cInclude("tactic.h");
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
            .maxTextureDimension1D = 5000,
            .maxTextureDimension2D = 3000,
            .maxTextureArrayLayers = 1,
            .maxBindGroups = 1,
            .maxBufferSize = @sizeOf(@TypeOf(d.point_data)),
            .maxUniformBufferBindingSize = 16 * @sizeOf(f32),
            .maxUniformBuffersPerShaderStage = 1,
            .maxVertexAttributes = 2,
            .maxVertexBuffers = 1,
            .maxVertexBufferArrayStride = d.vertex_stride,
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
    const uniform_buffer = g.wgpuDeviceCreateBuffer(
        device,
        &g.WGPUBufferDescriptor{
            .nextInChain = null,
            .label = null,
            .usage = g.WGPUBufferUsage_CopyDst | g.WGPUBufferUsage_Uniform,
            .size = @sizeOf(Uniforms),
            .mappedAtCreation = false,
        },
    );

    // Uniform binding
    const bind_group_layout_entry = std.mem.zeroInit(g.WGPUBindGroupLayoutEntry, .{
        .binding = 0,
        .visibility = g.WGPUShaderStage_Vertex,
        .buffer = std.mem.zeroInit(g.WGPUBufferBindingLayout, .{
            .type = g.WGPUBufferBindingType_Uniform,
            .minBindingSize = @sizeOf(Uniforms),
        }),
    });
    const bind_group_layout = g.wgpuDeviceCreateBindGroupLayout(device, &g.WGPUBindGroupLayoutDescriptor{
        .nextInChain = null,
        .label = null,
        .entryCount = 1,
        .entries = &bind_group_layout_entry,
    });
    const bind_group = g.wgpuDeviceCreateBindGroup(device, &g.WGPUBindGroupDescriptor{
        .nextInChain = null,
        .label = null,
        .layout = bind_group_layout,
        .entryCount = 1,
        .entries = &[_]g.WGPUBindGroupEntry{
            .{
                .nextInChain = null,
                .binding = 0,
                .buffer = uniform_buffer,
                .offset = 0,
                .size = @sizeOf(Uniforms),
                .sampler = null,
                .textureView = null,
            },
        },
    });

    // Depth texture & swap chain
    const size = t.tac_windowInnerSize();
    const depth_texture_out = createDepthTexture(.{
        .device = device,
        .size = size,
    });
    const format = g.wgpuSurfaceGetPreferredFormat(surface, adapter);
    const pipeline = p.buildPipeline(device, format, bind_group_layout);
    const swap_chain = createSwapChain(.{
        .device = device,
        .format = format,
        .size = size,
        .surface = surface,
    });

    // Full state
    global_state = .{
        .adapter = adapter,
        .bind_group = bind_group,
        .depth_texture_out = depth_texture_out,
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
        .uniform_buffer = uniform_buffer,
        .time = 0,
        .vertex_buffer = vertex_buffer,
    };

    // Listen
    // Probably smaller binaries with this instead of exported functions for
    // each even type?
    // TODO Pass in state pointer even if global?
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
    // std.debug.print("Virtual time: {}\n", .{state.time});
}

fn windowRedraw(state: *State) void {
    state.time += 1.0 / 60.0;
    const uniforms = Uniforms{
        .aspect = @intToFloat(f32, state.size.x) / @intToFloat(f32, state.size.y),
        .time = state.time,
    };
    g.wgpuQueueWriteBuffer(state.queue, state.uniform_buffer, 0, &uniforms, @sizeOf(Uniforms));
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
            .depthStencilAttachment = &g.WGPURenderPassDepthStencilAttachment{
                .view = state.depth_texture_out.depth_texture_view,
                .depthLoadOp = g.WGPULoadOp_Clear,
                .depthStoreOp = g.WGPUStoreOp_Store,
                .depthClearValue = 1,
                .depthReadOnly = false,
                .stencilLoadOp = g.WGPULoadOp_Clear,
                .stencilStoreOp = g.WGPUStoreOp_Store,
                .stencilClearValue = 0,
                .stencilReadOnly = true,
            },
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
    g.wgpuRenderPassEncoderSetBindGroup(render_pass, 0, state.bind_group, 0, null);
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
        // Depth texture
        g.wgpuTextureViewDrop(state.depth_texture_out.depth_texture_view);
        g.wgpuTextureDestroy(state.depth_texture_out.depth_texture);
        state.depth_texture_out = createDepthTexture(.{
            .device = state.device,
            .size = state.size,
        });
        // Swap chain
        g.wgpuSwapChainDrop(state.swap_chain);
        state.swap_chain = createSwapChain(.{
            .device = state.device,
            .format = state.format,
            .size = state.size,
            .surface = state.surface,
        });
    }
}

const State = struct {
    adapter: g.WGPUAdapter,
    bind_group: g.WGPUBindGroup,
    depth_texture_out: CreateDepthTextureOut,
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
    time: f32,
    uniform_buffer: g.WGPUBuffer,
    vertex_buffer: g.WGPUBuffer,
};

var global_state: State = undefined;

const Uniforms = struct {
    aspect: f32,
    time: f32,
};

const CreateDepthTextureIn = struct {
    device: g.WGPUDevice,
    size: t.tac_Vec2,
};

const CreateDepthTextureOut = struct {
    depth_texture: g.WGPUTexture,
    depth_texture_view: g.WGPUTextureView,
};

fn createDepthTexture(in: CreateDepthTextureIn) CreateDepthTextureOut {
    const depth_texture_format: g.WGPUTextureFormat = g.WGPUTextureFormat_Depth24Plus;
    const depth_texture = g.wgpuDeviceCreateTexture(
        in.device,
        &std.mem.zeroInit(g.WGPUTextureDescriptor, .{
            .usage = g.WGPUTextureUsage_RenderAttachment,
            .dimension = g.WGPUTextureDimension_2D,
            .size = .{
                .width = @intCast(u32, in.size.x),
                .height = @intCast(u32, in.size.y),
                .depthOrArrayLayers = 1,
            },
            .format = depth_texture_format,
            .mipLevelCount = 1,
            .sampleCount = 1,
            .viewFormatCount = 1,
            .viewFormats = &depth_texture_format,
        }),
    );
    const depth_texture_view = g.wgpuTextureCreateView(
        depth_texture,
        &std.mem.zeroInit(g.WGPUTextureViewDescriptor, .{
            .format = depth_texture_format,
            .dimension = g.WGPUTextureViewDimension_2D,
            .mipLevelCount = 1,
            .arrayLayerCount = 1,
            .aspect = g.WGPUTextureAspect_DepthOnly,
        }),
    );
    return .{
        .depth_texture = depth_texture,
        .depth_texture_view = depth_texture_view,
    };
}

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
