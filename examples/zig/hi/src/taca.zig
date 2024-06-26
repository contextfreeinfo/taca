pub const Bindings = struct {
    vertex_buffers: []const *Buffer,
    index_buffer: *Buffer,
    // TODO images
};

pub const Buffer = extern struct {};

pub const BufferSlice = extern struct {
    ptr: *const anyopaque,
    size: usize,
    item_size: usize,

    // TODO Video on this? Rust, Zig, Jai, D, Odin, C++?
    pub fn new(items: anytype) BufferSlice {
        const info = @typeInfo(@TypeOf(items)).Pointer;
        return .{
            .ptr = items.ptr,
            .size = @sizeOf(info.child),
            .item_size = @sizeOf(@typeInfo(info.child).Array.child),
        };
    }
};

pub const BufferType = enum(c_int) {
    vertex,
    index,
};

pub const BufferUsage = enum(c_int) {
    immutable,
    dynamic,
    stream,
};

pub const EventKind = enum(c_int) {
    draw,
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

    pub fn applyBindings(self: *Self, bindings: Bindings) void {
        taca_RenderingContext_applyBindings(self, ExternBindings.from(bindings));
    }

    pub fn applyPipeline(self: *Self, pipeline: *Pipeline) void {
        taca_RenderingContext_applyPipeline(self, pipeline);
    }

    pub fn applyUniforms(self: *Self, uniforms: anytype) void {
        taca_RenderingContext_applyUniforms(self, as_u8_span(uniforms));
    }

    pub fn beginPass(self: *Self) void {
        taca_RenderingContext_beginPass(self);
    }

    pub fn commitFrame(self: *Self) void {
        taca_RenderingContext_commitFrame(self);
    }

    pub fn draw(
        self: *Self,
        item_begin: i32,
        item_count: i32,
        instance_count: i32,
    ) void {
        taca_RenderingContext_draw(self, item_begin, item_count, instance_count);
    }

    pub fn endPass(self: *Self) void {
        taca_RenderingContext_endPass(self);
    }

    pub fn newBuffer(
        self: *Self,
        typ: BufferType,
        usage: BufferUsage,
        slice: BufferSlice,
    ) *Buffer {
        return taca_RenderingContext_newBuffer(self, typ, usage, &slice);
    }

    pub fn newPipeline(self: *Self, info: PipelineInfo) *Pipeline {
        return taca_RenderingContext_newPipeline(self, ExternPipelineInfo.from(info));
    }

    pub fn newShader(self: *Self, bytes: []const u8) *Shader {
        return taca_RenderingContext_newShader(self, Span(u8).from(bytes));
    }
};

pub const Shader = extern struct {};

pub fn Span(comptime T: type) type {
    return extern struct {
        ptr: [*c]const T,
        len: usize,

        pub fn from(slice: []const T) Span(T) {
            return .{ .ptr = slice.ptr, .len = slice.len };
        }
    };
}

// TODO Text metrics and rendering

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
    pub const newRenderingContext = taca_Window_newRenderingContext;

    pub fn print(text: []const u8) void {
        taca_Window_print(Span(u8).from(text));
    }

    pub fn setTitle(title: []const u8) void {
        taca_Window_setTitle(Span(u8).from(title));
    }

    pub fn state() WindowState {
        return taca_Window_state();
    }
};

pub const WindowState = extern struct {
    // TODO Should size be integer?
    pointer: [2]f32,
    size: [2]f32,
};

// Helpers

fn as_u8_span(any: anytype) Span(u8) {
    const info = @typeInfo(@TypeOf(any)).Pointer;
    return .{
        .ptr = @ptrCast(any),
        .len = @sizeOf(info.child),
    };
}

// Extern definitions

const ExternBindings = extern struct {
    vertex_buffers: Span(*Buffer),
    index_buffer: *Buffer,
    // TODO images

    pub fn from(bindings: Bindings) ExternBindings {
        return .{
            .vertex_buffers = Span(*Buffer).from(bindings.vertex_buffers),
            .index_buffer = bindings.index_buffer,
        };
    }
};

const ExternPipelineInfo = extern struct {
    attributes: Span(VertexAttribute),
    fragment: ExternPipelineShaderInfo,
    vertex: ExternPipelineShaderInfo,

    pub fn from(info: PipelineInfo) ExternPipelineInfo {
        return .{
            .attributes = Span(VertexAttribute).from(info.attributes),
            .fragment = ExternPipelineShaderInfo.from(info.fragment),
            .vertex = ExternPipelineShaderInfo.from(info.vertex),
        };
    }
};

const ExternPipelineShaderInfo = extern struct {
    entry_point: Span(u8),
    shader: *Shader,

    pub fn from(info: PipelineShaderInfo) ExternPipelineShaderInfo {
        return .{
            .entry_point = Span(u8).from(info.entry_point),
            .shader = info.shader,
        };
    }
};

extern fn taca_RenderingContext_applyBindings(
    context: *RenderingContext,
    bindings: ExternBindings,
) void;

extern fn taca_RenderingContext_applyPipeline(
    context: *RenderingContext,
    pipeline: *Pipeline,
) void;

extern fn taca_RenderingContext_applyUniforms(
    context: *RenderingContext,
    uniforms: Span(u8),
) void;

extern fn taca_RenderingContext_beginPass(
    context: *RenderingContext,
) void;

extern fn taca_RenderingContext_commitFrame(
    context: *RenderingContext,
) void;

extern fn taca_RenderingContext_draw(
    context: *RenderingContext,
    item_begin: i32,
    item_count: i32,
    instance_count: i32,
) void;

extern fn taca_RenderingContext_endPass(
    context: *RenderingContext,
) void;

extern fn taca_RenderingContext_newBuffer(
    context: *RenderingContext,
    typ: BufferType,
    usage: BufferUsage,
    info: *const BufferSlice,
) callconv(.C) *Buffer;

extern fn taca_RenderingContext_newPipeline(
    context: *RenderingContext,
    info: ExternPipelineInfo,
) callconv(.C) *Pipeline;

extern fn taca_RenderingContext_newShader(
    context: *RenderingContext,
    bytes: Span(u8),
) callconv(.C) *Shader;

extern fn taca_Window_newRenderingContext(
    // Nothing
) callconv(.C) *RenderingContext;

extern fn taca_Window_print(
    text: Span(u8),
) callconv(.C) void;

extern fn taca_Window_setTitle(
    title: Span(u8),
) callconv(.C) void;

extern fn taca_Window_state(
    // Nothing
) callconv(.C) WindowState;
