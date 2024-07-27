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

pub const RenderingContext = struct {
    const Self = RenderingContext;

    pub fn applyBindings(bindings: Bindings) void {
        taca_RenderingContext_applyBindings(ExternBindings.from(bindings));
    }

    pub fn applyPipeline(pipeline: *Pipeline) void {
        taca_RenderingContext_applyPipeline(pipeline);
    }

    pub fn applyUniforms(uniforms: anytype) void {
        taca_RenderingContext_applyUniforms(as_u8_span(uniforms));
    }

    pub fn beginPass() void {
        taca_RenderingContext_beginPass();
    }

    pub fn commitFrame() void {
        taca_RenderingContext_commitFrame();
    }

    pub fn draw(
        item_begin: u32,
        item_count: u32,
        instance_count: u32,
    ) void {
        taca_RenderingContext_draw(item_begin, item_count, instance_count);
    }

    pub fn drawText(text: []const u8, x: f32, y: f32) void {
        taca_RenderingContext_drawText(Span(u8).from(text), x, y);
    }

    pub fn drawTexture(texture: *Texture, x: f32, y: f32) void {
        taca_RenderingContext_drawTexture(texture, x, y);
    }

    pub fn endPass() void {
        taca_RenderingContext_endPass();
    }

    pub fn newBuffer(
        typ: BufferType,
        usage: BufferUsage,
        slice: BufferSlice,
    ) *Buffer {
        return taca_RenderingContext_newBuffer(typ, usage, &slice);
    }

    pub fn newPipeline(info: PipelineInfo) *Pipeline {
        return taca_RenderingContext_newPipeline(ExternPipelineInfo.from(info));
    }

    pub fn newShader(bytes: []const u8) *Shader {
        return taca_RenderingContext_newShader(Span(u8).from(bytes));
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

pub const Text = struct {
    pub fn draw(text: []const u8) *Texture {
        return taca_Text_draw(Span(u8).from(text));
    }
};

pub const Texture = extern struct {};

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
    bindings: ExternBindings,
) void;

extern fn taca_RenderingContext_applyPipeline(
    pipeline: *Pipeline,
) void;

extern fn taca_RenderingContext_applyUniforms(
    uniforms: Span(u8),
) void;

extern fn taca_RenderingContext_beginPass(
    // Nothing
) void;

extern fn taca_RenderingContext_commitFrame(
    // Nothing
) void;

extern fn taca_RenderingContext_draw(
    item_begin: u32,
    item_count: u32,
    instance_count: u32,
) void;

extern fn taca_RenderingContext_drawText(
    bytes: Span(u8),
    x: f32,
    y: f32,
) void;

extern fn taca_RenderingContext_drawTexture(
    texture: *Texture,
    x: f32,
    y: f32,
) void;

extern fn taca_RenderingContext_endPass(
    // Nothing
) void;

extern fn taca_RenderingContext_newBuffer(
    typ: BufferType,
    usage: BufferUsage,
    info: *const BufferSlice,
) callconv(.C) *Buffer;

extern fn taca_RenderingContext_newPipeline(
    info: ExternPipelineInfo,
) callconv(.C) *Pipeline;

extern fn taca_RenderingContext_newShader(
    bytes: Span(u8),
) callconv(.C) *Shader;

extern fn taca_Text_draw(
    text: Span(u8),
) callconv(.C) *Texture;

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
