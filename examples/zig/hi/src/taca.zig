const std = @import("std");

pub const Bindings = extern struct {
    vertex_buffers: []const *const Buffer,
    index_buffer: *const Buffer,
    // TODO images
};

pub const Buffer = extern struct {};

pub const BufferSlice = extern struct {
    ptr: *const anyopaque,
    size: usize,
    item_size: usize,

    pub fn new(comptime items: anytype) BufferSlice {
        const info = @typeInfo(@TypeOf(items)).Pointer;
        const item_size = @sizeOf(info.child);
        return .{
            .ptr = items.ptr,
            .size = items.len * item_size,
            .item_size = item_size,
        };
    }
};

pub const BufferType = enum(c_int) {
    vertex_buffer,
    index_buffer,
};

pub const BufferUsage = enum(c_int) {
    immutable,
    dynamic,
    stream,
};

pub const EventKind = enum(c_int) {
    draw,
};

pub const Event = extern struct {
    kind: EventKind,
    detail: extern union {
        //
    },
};

pub const Pipeline = extern struct {};

pub const PipelineShaderInfo = struct {
    entry_point: []const u8,
    shader: *Shader,
};

pub const PipelineInfo = struct {
    attributes: []const VertexAttribute,
    fragment: PipelineShaderInfo,
    vertex: PipelineShaderInfo,
};

pub const RenderingContext = extern struct {
    const Self = RenderingContext;

    pub fn newBuffer(
        self: *Self,
        typ: BufferType,
        usage: BufferUsage,
        slice: BufferSlice,
    ) *Buffer {
        return taca_RenderingContext_newBuffer(self, typ, usage, &slice);
    }

    pub fn newPipeline(self: *Self, info: PipelineInfo) *Pipeline {
        return taca_RenderingContext_newPipeline(self, &info);
    }

    pub fn newShader(self: *Self, bytes: []const u8) *Shader {
        return taca_RenderingContext_newShader(self, bytes.ptr, bytes.len);
    }
};

pub const Shader = extern struct {};

pub const VertexAttribute = extern struct {
    format: VertexFormat,
    buffer_index: usize = 0,
};

pub const VertexFormat = enum(c_int) {
    // Retain order!
    float1,
    float2,
    float3,
    float4,
    byte1,
    byte2,
    byte3,
    byte4,
    short1,
    short2,
    short3,
    short4,
    int1,
    int2,
    int3,
    int4,
    mat4,
};

pub const Window = extern struct {
    pub const get = taca_Window_get;
    pub const newRenderingContext = taca_Window_newRenderingContext;
};

// Extern definitions.

// TODO Delete if not needed.
pub const ExternPipelineInfo = extern struct {
    fragment: ExternPipelineShaderInfo,
    vertex: ExternPipelineShaderInfo,
};

// TODO Delete if not needed.
pub const ExternPipelineShaderInfo = extern struct {
    entryPoint: []const u8,
    entryPointLength: usize,
    shader: *Shader,
};

extern fn taca_RenderingContext_newBuffer(
    context: *RenderingContext,
    typ: BufferType,
    usage: BufferUsage,
    info: *const BufferSlice,
) callconv(.C) *Buffer;

extern fn taca_RenderingContext_newPipeline(
    context: *RenderingContext,
    info: *const PipelineInfo,
) callconv(.C) *Pipeline;

extern fn taca_RenderingContext_newShader(
    context: *RenderingContext,
    bytes: [*c]const u8,
    bytesLength: usize,
) callconv(.C) *Shader;

extern fn taca_Window_get() callconv(.C) *Window;

extern fn taca_Window_newRenderingContext(
    window: *Window,
) callconv(.C) *RenderingContext;
