// Based largely on:
// https://github.com/eliemichel/LearnWebGPU-Code/blob/step033/main.cpp

const std = @import("std");
const assert = std.debug.assert;
const a = @import("./zalgebra/main.zig");
const d = @import("./data.zig");
const p = @import("./pipeline.zig");
const c = @cImport({
    @cInclude("taca.h");
    @cInclude("webgpu.h");
    @cInclude("wgpu.h");
});

pub fn main() void {
    c.taca_windowSetTitle("Exploring WebGPU with Taca");

    // Instance
    const instance = c.wgpuCreateInstance(&c.WGPUInstanceDescriptor{
        .nextInChain = null,
    }) orelse unreachable;
    errdefer c.wgpuInstanceDrop(instance);

    // Surface
    const surface = c.wgpuInstanceCreateSurface(
        instance,
        &c.WGPUSurfaceDescriptor{
            .nextInChain = @ptrCast(
                *const c.WGPUChainedStruct,
                &c.WGPUSurfaceDescriptorFromCanvasHTMLSelector{
                    .chain = .{
                        .next = null,
                        .sType = c.WGPUSType_SurfaceDescriptorFromCanvasHTMLSelector,
                    },
                    .selector = "",
                },
            ),
            .label = null,
        },
    ) orelse unreachable;
    errdefer c.wgpuSurfaceDrop(surface);

    // Adapter
    // This only works because the callback is effectively synchronous.
    // Otherwise, we'd need to allocate on the heap or global.
    var request_adapter_callback_data = RequestAdapterCallbackData{
        .instance = instance,
        .surface = surface,
    };
    c.wgpuInstanceRequestAdapter(
        instance,
        &c.WGPURequestAdapterOptions{
            .nextInChain = null,
            .compatibleSurface = surface,
            .powerPreference = c.WGPUPowerPreference_Undefined,
            .forceFallbackAdapter = false,
        },
        requestAdapterCallback,
        &request_adapter_callback_data,
    );
    const adapter = request_adapter_callback_data.adapter orelse unreachable;
    var supported_limits = std.mem.zeroInit(c.WGPUSupportedLimits, .{});
    _ = c.wgpuAdapterGetLimits(adapter, &supported_limits) or unreachable;
    errdefer c.wgpuAdapterDrop(adapter);

    // Device & Queue
    var request_device_callback_data = RequestDeviceCallbackData{
        .adapter = adapter,
        .surface = surface,
    };
    const required_limits = c.WGPURequiredLimits{
        .nextInChain = null,
        .limits = std.mem.zeroInit(c.WGPULimits, .{
            .maxTextureDimension1D = 5000,
            .maxTextureDimension2D = 3000,
            .maxTextureArrayLayers = 1,
            .maxBindGroups = 1,
            .maxSampledTexturesPerShaderStage = 1,
            .maxBufferSize = @max(@sizeOf(@TypeOf(d.point_data)), @sizeOf(Uniforms)),
            .maxUniformBufferBindingSize = @sizeOf(Uniforms),
            .maxUniformBuffersPerShaderStage = 1,
            .maxVertexAttributes = 3,
            .maxVertexBuffers = 1,
            .maxVertexBufferArrayStride = d.vertex_stride,
            .minStorageBufferOffsetAlignment = supported_limits.limits.minStorageBufferOffsetAlignment,
            .minUniformBufferOffsetAlignment = supported_limits.limits.minUniformBufferOffsetAlignment,
            .maxInterStageShaderComponents = 6,
        }),
    };
    c.wgpuAdapterRequestDevice(
        adapter,
        &c.WGPUDeviceDescriptor{
            .nextInChain = null,
            .label = null,
            .requiredFeaturesCount = 0,
            .requiredFeatures = null,
            .requiredLimits = &required_limits,
            .defaultQueue = std.mem.zeroInit(c.WGPUQueueDescriptor, .{}),
        },
        requestDeviceCallback,
        &request_device_callback_data,
    );
    const device = request_device_callback_data.device orelse unreachable;
    errdefer c.wgpuDeviceDrop(device);
    c.wgpuDeviceSetUncapturedErrorCallback(device, deviceUncapturedErrorCallback, null);
    const queue = c.wgpuDeviceGetQueue(device);

    // Buffers
    const vertex_buffer = c.wgpuDeviceCreateBuffer(
        device,
        &c.WGPUBufferDescriptor{
            .nextInChain = null,
            .label = null,
            .usage = c.WGPUBufferUsage_CopyDst | c.WGPUBufferUsage_Vertex,
            .size = d.vertex_data_size,
            .mappedAtCreation = false,
        },
    );
    c.wgpuQueueWriteBuffer(queue, vertex_buffer, 0, &d.vertex_data, d.vertex_data_size);
    const index_buffer = c.wgpuDeviceCreateBuffer(
        device,
        &c.WGPUBufferDescriptor{
            .nextInChain = null,
            .label = null,
            .usage = c.WGPUBufferUsage_CopyDst | c.WGPUBufferUsage_Index,
            .size = @sizeOf(@TypeOf(d.index_data)),
            .mappedAtCreation = false,
        },
    );
    c.wgpuQueueWriteBuffer(queue, index_buffer, 0, &d.index_data, @sizeOf(@TypeOf(d.index_data)));
    const point_buffer = c.wgpuDeviceCreateBuffer(
        device,
        &c.WGPUBufferDescriptor{
            .nextInChain = null,
            .label = null,
            .usage = c.WGPUBufferUsage_CopyDst | c.WGPUBufferUsage_Vertex,
            .size = @sizeOf(@TypeOf(d.point_data)),
            .mappedAtCreation = false,
        },
    );
    c.wgpuQueueWriteBuffer(queue, point_buffer, 0, &d.point_data, @sizeOf(@TypeOf(d.point_data)));
    const uniform_buffer = c.wgpuDeviceCreateBuffer(
        device,
        &c.WGPUBufferDescriptor{
            .nextInChain = null,
            .label = null,
            .usage = c.WGPUBufferUsage_CopyDst | c.WGPUBufferUsage_Uniform,
            .size = @sizeOf(Uniforms),
            .mappedAtCreation = false,
        },
    );

    // Texture
    const image_texture_desc = std.mem.zeroInit(c.WGPUTextureDescriptor, .{
        .usage = c.WGPUTextureUsage_CopyDst | c.WGPUTextureUsage_TextureBinding,
        .dimension = c.WGPUTextureDimension_2D,
        .size = .{
            .width = 256,
            .height = 256,
            .depthOrArrayLayers = 1,
        },
        .format = c.WGPUTextureFormat_RGBA8Unorm,
        .mipLevelCount = 1,
        .sampleCount = 1,
        .viewFormatCount = 0,
        .viewFormats = null,
    });
    const image_texture = c.wgpuDeviceCreateTexture(device, &image_texture_desc);
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = general_purpose_allocator.allocator();
    const image_texture_data = allocator.alloc(
        u8,
        4 * image_texture_desc.size.width * image_texture_desc.size.height,
    ) catch unreachable;
    for (0..image_texture_desc.size.width) |j| {
        for (0..image_texture_desc.size.height) |i| {
            const n = 4 * (i * image_texture_desc.size.width + j);
            image_texture_data[n + 0] = if ((i / 16) % 2 == (j / 16) % 2) 255 else 0;
            image_texture_data[n + 1] = if (((i - j) / 16) % 2 == 0) 255 else 0;
            image_texture_data[n + 2] = if (((i + j) / 16) % 2 == 0) 255 else 0;
            image_texture_data[n + 3] = 255;
        }
    }
    c.wgpuQueueWriteTexture(
        queue,
        &std.mem.zeroInit(c.WGPUImageCopyTexture, .{ .texture = image_texture }),
        image_texture_data.ptr,
        image_texture_data.len,
        &std.mem.zeroInit(c.WGPUTextureDataLayout, .{
            .bytesPerRow = 4 * image_texture_desc.size.width,
            .rowsPerImage = image_texture_desc.size.height,
        }),
        &image_texture_desc.size,
    );
    const image_texture_view = c.wgpuTextureCreateView(
        image_texture,
        &std.mem.zeroInit(c.WGPUTextureViewDescriptor, .{
            .format = image_texture_desc.format,
            .dimension = c.WGPUTextureViewDimension_2D,
            .mipLevelCount = 1,
            .arrayLayerCount = 1,
        }),
    );

    // Uniform & texture binding
    const bind_group_layout = c.wgpuDeviceCreateBindGroupLayout(device, &c.WGPUBindGroupLayoutDescriptor{
        .nextInChain = null,
        .label = null,
        .entryCount = 2,
        .entries = &[_]c.WGPUBindGroupLayoutEntry{
            std.mem.zeroInit(c.WGPUBindGroupLayoutEntry, .{
                .binding = 0,
                .visibility = c.WGPUShaderStage_Vertex | c.WGPUShaderStage_Fragment,
                .buffer = std.mem.zeroInit(c.WGPUBufferBindingLayout, .{
                    .type = c.WGPUBufferBindingType_Uniform,
                    .minBindingSize = @sizeOf(Uniforms),
                }),
            }),
            std.mem.zeroInit(c.WGPUBindGroupLayoutEntry, .{
                .binding = 1,
                .visibility = c.WGPUShaderStage_Fragment,
                .sampler = std.mem.zeroInit(c.WGPUSamplerBindingLayout, .{
                    .type = c.WGPUSamplerBindingType_NonFiltering,
                }),
                .texture = std.mem.zeroInit(c.WGPUTextureBindingLayout, .{
                    .sampleType = c.WGPUTextureSampleType_Float,
                    .viewDimension = c.WGPUTextureViewDimension_2D,
                }),
            }),
        },
    });
    const bind_group = c.wgpuDeviceCreateBindGroup(device, &c.WGPUBindGroupDescriptor{
        .nextInChain = null,
        .label = null,
        .layout = bind_group_layout,
        .entryCount = 2,
        .entries = &[_]c.WGPUBindGroupEntry{
            std.mem.zeroInit(c.WGPUBindGroupEntry, .{
                .binding = 0,
                .buffer = uniform_buffer,
                .size = @sizeOf(Uniforms),
            }),
            std.mem.zeroInit(c.WGPUBindGroupEntry, .{
                .binding = 1,
                .textureView = image_texture_view,
            }),
        },
    });

    // Depth texture & swap chain
    const size = c.taca_windowInnerSize();
    const depth_texture_out = createDepthTexture(.{
        .device = device,
        .size = size,
    });
    const format = c.wgpuSurfaceGetPreferredFormat(surface, adapter);
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
        .position = a.Vec3.zero(),
        .projection = buildPerspective(size),
        .queue = queue,
        .size = size,
        .swap_chain = swap_chain,
        .surface = surface,
        .uniform_buffer = uniform_buffer,
        .time = 0,
        .velocity = a.Vec3.zero(),
        .vertex_buffer = vertex_buffer,
        .view = a.Mat4.identity()
            .translate(a.Vec3.new(0, 0, -2))
            .rotate(135, a.Vec3.new(1, 0, 0))
            .transpose(),
    };
    // std.debug.print("---->\n{}\n", .{global_state.projection});
    // std.debug.print("---->\n{}\n", .{global_state.view});

    // Listen
    // Option for either named export via null or else an indexed export with pointer.
    // And pass in state pointer even if global for now.
    // c.taca_windowListen(null, &global_state);
    c.taca_windowListen(windowListen, &global_state);
}

export fn windowListen(event_type: c.taca_WindowEventType, userdata: ?*anyopaque) void {
    const state = @ptrCast(*State, @alignCast(@alignOf(State), userdata));
    switch (event_type) {
        c.taca_WindowEventType_Close => windowClose(state),
        c.taca_WindowEventType_Key => keyPress(state),
        c.taca_WindowEventType_Redraw => windowRedraw(state),
        c.taca_WindowEventType_Resize => windowResize(state),
        else => unreachable,
    }
}

fn keyPress(state: *State) void {
    const event = c.taca_keyEvent();
    const speed: f32 = if (event.pressed) 0.02 else 0;
    switch (event.code) {
        c.taca_KeyCode_Undefined => {},
        c.taca_KeyCode_Left => {
            updateSpeed(&state.velocity, 0, -1, speed);
        },
        c.taca_KeyCode_Up => {
            updateSpeed(&state.velocity, 1, 1, speed);
        },
        c.taca_KeyCode_Right => {
            updateSpeed(&state.velocity, 0, 1, speed);
        },
        c.taca_KeyCode_Down => {
            updateSpeed(&state.velocity, 1, -1, speed);
        },
        else => unreachable,
    }
}

fn updateSpeed(vec: *a.Vec3, index: usize, direction: f32, speed: f32) void {
    if (std.math.sign(vec.data[index]) == direction or speed != 0) {
        vec.data[index] = direction * speed;
    }
}

fn windowClose(state: *State) void {
    c.wgpuDeviceDrop(state.device);
    c.wgpuAdapterDrop(state.adapter);
    c.wgpuSurfaceDrop(state.surface);
    c.wgpuInstanceDrop(state.instance);
    // std.debug.print("Virtual time: {}\n", .{state.time});
}

fn windowRedraw(state: *State) void {
    state.position = state.position.add(state.velocity);
    state.time += 1.0 / 60.0;
    // TODO Be more selective about what uniforms we send when?
    const uniforms = Uniforms{
        .projection = state.projection,
        .view = state.view,
        .position = state.position,
        .time = state.time,
    };
    c.wgpuQueueWriteBuffer(state.queue, state.uniform_buffer, 0, &uniforms, @sizeOf(Uniforms));
    const view = c.wgpuSwapChainGetCurrentTextureView(state.swap_chain) orelse unreachable;
    const encoder = c.wgpuDeviceCreateCommandEncoder(
        state.device,
        &c.WGPUCommandEncoderDescriptor{
            .nextInChain = null,
            .label = null,
        },
    ) orelse unreachable;
    const render_pass = c.wgpuCommandEncoderBeginRenderPass(
        encoder,
        &c.WGPURenderPassDescriptor{
            .nextInChain = null,
            .label = null,
            .colorAttachmentCount = 1,
            .colorAttachments = &c.WGPURenderPassColorAttachment{
                .view = view,
                .resolveTarget = null,
                .loadOp = c.WGPULoadOp_Clear,
                .storeOp = c.WGPUStoreOp_Store,
                .clearValue = .{
                    .r = 0.05,
                    .g = 0.05,
                    .b = 0.05,
                    .a = 1.0,
                },
            },
            .depthStencilAttachment = &c.WGPURenderPassDepthStencilAttachment{
                .view = state.depth_texture_out.depth_texture_view,
                .depthLoadOp = c.WGPULoadOp_Clear,
                .depthStoreOp = c.WGPUStoreOp_Store,
                .depthClearValue = 1,
                .depthReadOnly = false,
                .stencilLoadOp = c.WGPULoadOp_Clear,
                .stencilStoreOp = c.WGPUStoreOp_Store,
                .stencilClearValue = 0,
                .stencilReadOnly = true,
            },
            .occlusionQuerySet = null,
            .timestampWriteCount = 0,
            .timestampWrites = null,
        },
    ) orelse unreachable;
    c.wgpuRenderPassEncoderSetPipeline(render_pass, state.pipeline);
    // c.wgpuRenderPassEncoderSetVertexBuffer(render_pass, 0, state.vertex_buffer, 0, d.vertex_data_size);
    // c.wgpuRenderPassEncoderDraw(render_pass, d.vertex_count, 1, 0, 0);
    c.wgpuRenderPassEncoderSetVertexBuffer(render_pass, 0, state.point_buffer, 0, @sizeOf(@TypeOf(d.point_data)));
    c.wgpuRenderPassEncoderSetIndexBuffer(render_pass, state.index_buffer, c.WGPUIndexFormat_Uint16, 0, @sizeOf(@TypeOf(d.index_data)));
    c.wgpuRenderPassEncoderSetBindGroup(render_pass, 0, state.bind_group, 0, null);
    c.wgpuRenderPassEncoderDrawIndexed(render_pass, d.index_data.len, 1, 0, 0, 0);
    c.wgpuRenderPassEncoderEnd(render_pass);
    c.wgpuTextureViewDrop(view);
    const command_buffer = c.wgpuCommandEncoderFinish(
        encoder,
        &c.WGPUCommandBufferDescriptor{
            .nextInChain = null,
            .label = null,
        },
    ) orelse unreachable;
    c.wgpuQueueSubmit(state.queue, 1, &command_buffer);
    c.wgpuSwapChainPresent(state.swap_chain);
}

fn windowResize(state: *State) void {
    const size = c.taca_windowInnerSize();
    if (size.x > 0 and size.y > 0) {
        state.projection = buildPerspective(size);
        state.size = size;
        // Depth texture
        c.wgpuTextureViewDrop(state.depth_texture_out.depth_texture_view);
        c.wgpuTextureDestroy(state.depth_texture_out.depth_texture);
        state.depth_texture_out = createDepthTexture(.{
            .device = state.device,
            .size = state.size,
        });
        // Swap chain
        c.wgpuSwapChainDrop(state.swap_chain);
        state.swap_chain = createSwapChain(.{
            .device = state.device,
            .format = state.format,
            .size = state.size,
            .surface = state.surface,
        });
    }
}

const State = struct {
    adapter: c.WGPUAdapter,
    bind_group: c.WGPUBindGroup,
    depth_texture_out: CreateDepthTextureOut,
    device: c.WGPUDevice,
    format: c.WGPUTextureFormat,
    index_buffer: c.WGPUBuffer,
    instance: c.WGPUInstance,
    pipeline: c.WGPURenderPipeline,
    point_buffer: c.WGPUBuffer,
    position: a.Vec3,
    projection: a.Mat4,
    queue: c.WGPUQueue,
    size: c.taca_Vec2,
    swap_chain: c.WGPUSwapChain,
    surface: c.WGPUSurface,
    time: f32,
    uniform_buffer: c.WGPUBuffer,
    velocity: a.Vec3,
    vertex_buffer: c.WGPUBuffer,
    view: a.Mat4,
};

var global_state: State = undefined;

// Can't put arrays in packed structs, so go extern. I hope this guarantees order:
// https://github.com/ziglang/zig/issues/12547
const Uniforms = extern struct {
    projection: a.Mat4,
    view: a.Mat4,
    time: f32,
    position: a.Vec3,
};

fn buildPerspective(size: c.taca_Vec2) a.Mat4 {
    // The tutorial uses different calculations than zalgebra, and I'm not
    // getting what I want from zalgebra, so go with tutorial.
    const aspect = @intToFloat(f32, size.x) / @intToFloat(f32, size.y);
    const focal_length: f32 = 2;
    const near: f32 = 0.01;
    const far: f32 = 100;
    const divider: f32 = 1.0 / (focal_length * (far - near));
    const perspective = (a.Mat4{
        .data = .{
            .{ 1, 0, 0, 0 },
            .{ 0, aspect, 0, 0 },
            .{ 0, 0, far * divider, -far * near * divider },
            .{ 0, 0, 1 / focal_length, 0 },
        },
    }).transpose();
    // return a.perspective(45.0, aspect, 0.1, 100.0);
    return perspective;
}

const CreateDepthTextureIn = struct {
    device: c.WGPUDevice,
    size: c.taca_Vec2,
};

const CreateDepthTextureOut = struct {
    depth_texture: c.WGPUTexture,
    depth_texture_view: c.WGPUTextureView,
};

fn createDepthTexture(in: CreateDepthTextureIn) CreateDepthTextureOut {
    const depth_texture_format: c.WGPUTextureFormat = c.WGPUTextureFormat_Depth24Plus;
    const depth_texture = c.wgpuDeviceCreateTexture(
        in.device,
        &std.mem.zeroInit(c.WGPUTextureDescriptor, .{
            .usage = c.WGPUTextureUsage_RenderAttachment,
            .dimension = c.WGPUTextureDimension_2D,
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
    const depth_texture_view = c.wgpuTextureCreateView(
        depth_texture,
        &std.mem.zeroInit(c.WGPUTextureViewDescriptor, .{
            .format = depth_texture_format,
            .dimension = c.WGPUTextureViewDimension_2D,
            .mipLevelCount = 1,
            .arrayLayerCount = 1,
            .aspect = c.WGPUTextureAspect_DepthOnly,
        }),
    );
    return .{
        .depth_texture = depth_texture,
        .depth_texture_view = depth_texture_view,
    };
}

const CreateSwapChainData = struct {
    device: c.WGPUDevice,
    format: c.WGPUTextureFormat,
    size: c.taca_Vec2,
    surface: c.WGPUSurface,
};

fn createSwapChain(data: CreateSwapChainData) c.WGPUSwapChain {
    const swap_chain = c.wgpuDeviceCreateSwapChain(
        data.device,
        data.surface,
        &c.WGPUSwapChainDescriptor{
            .nextInChain = null,
            .label = null,
            .usage = c.WGPUTextureUsage_RenderAttachment,
            .format = data.format,
            .width = @intCast(u32, data.size.x),
            .height = @intCast(u32, data.size.y),
            .presentMode = c.WGPUPresentMode_Fifo,
        },
    ) orelse unreachable;
    return swap_chain;
}

// Adapter

const RequestAdapterCallbackData = struct {
    adapter: ?c.WGPUAdapter = null,
    instance: c.WGPUInstance,
    surface: c.WGPUSurface,
};

fn requestAdapterCallback(
    status: c.WGPURequestAdapterStatus,
    adapter: c.WGPUAdapter,
    message: [*c]const u8,
    userdata: ?*anyopaque,
) callconv(.C) void {
    assert(status == c.WGPURequestDeviceStatus_Success);
    _ = message;
    var data = @ptrCast(
        *RequestAdapterCallbackData,
        @alignCast(@alignOf(*RequestAdapterCallbackData), userdata),
    );
    data.adapter = adapter;
}

// Device

const RequestDeviceCallbackData = struct {
    adapter: c.WGPUAdapter,
    device: ?c.WGPUDevice = null,
    surface: c.WGPUSurface,
};

fn requestDeviceCallback(
    status: c.WGPURequestDeviceStatus,
    device: c.WGPUDevice,
    message: [*c]const u8,
    userdata: ?*anyopaque,
) callconv(.C) void {
    assert(status == c.WGPURequestDeviceStatus_Success);
    _ = message;
    var data = @ptrCast(
        *RequestDeviceCallbackData,
        @alignCast(@alignOf(*RequestDeviceCallbackData), userdata),
    );
    data.device = device;
}

fn deviceUncapturedErrorCallback(
    status: c.WGPURequestDeviceStatus,
    message: [*c]const u8,
    userdata: ?*anyopaque,
) callconv(.C) void {
    // TODO Once we can actually call this.
    _ = status;
    _ = message;
    _ = userdata;
}
