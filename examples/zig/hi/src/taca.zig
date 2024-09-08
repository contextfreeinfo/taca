pub const Bindings = struct {
    vertex_buffers: []const *Buffer,
    index_buffer: *Buffer,
    // TODO images
};

pub const Buffer = extern struct {};

pub const BufferInfo = extern struct {
    kind: BufferKind = .vertex,
    slice: BufferSlice,
};

// TODO Just do a late translate of a slice to opaque bytes?
pub const BufferSlice = extern struct {
    ptr: ?*const anyopaque = null,
    size: usize,

    pub fn new(items: anytype) BufferSlice {
        const info = @typeInfo(@TypeOf(items)).Pointer;
        return .{ .ptr = items.ptr, .size = @sizeOf(info.child) };
    }
};

pub const BufferKind = enum(c_int) {
    vertex,
    index,
};

pub const EventKind = enum(c_int) {
    draw,
};

pub const Pipeline = extern struct {};

pub const PipelineShaderInfo = struct {
    entry: []const u8 = "",
    shader: ?*const Shader = null,
};

pub const PipelineInfo = struct {
    depth_test: bool = false,
    // Attributes are specified separately from buffers to make it easier to
    // avoid memory allocation when prepping for ffi.
    fragment: PipelineShaderInfo = .{},
    vertex: PipelineShaderInfo = .{},
    vertex_attributes: []const VertexAttribute = &[_]VertexAttribute{},
    vertex_buffers: []const VertexBufferLayout = &[_]VertexBufferLayout{},
};

pub const RenderingContext = struct {
    const Self = RenderingContext;

    pub fn applyBindings(bindings: Bindings) void {
        taca_bindings_apply(ExternBindings.from(bindings));
    }

    pub fn applyPipeline(pipeline: *Pipeline) void {
        taca_pipeline_apply(pipeline);
    }

    pub fn applyUniforms(uniforms: anytype) void {
        taca_uniforms_apply(as_u8_span(uniforms));
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
        taca_draw(item_begin, item_count, instance_count);
    }

    pub fn drawText(text: []const u8, x: f32, y: f32) void {
        taca_text_draw(Span(u8).from(text), x, y);
    }

    pub fn drawTexture(texture: *Texture, x: f32, y: f32) void {
        taca_text_drawure(texture, x, y);
    }

    pub fn endPass() void {
        taca_RenderingContext_endPass();
    }

    pub fn newBuffer(info: BufferInfo) *Buffer {
        return taca_buffer_new(info.kind, &info.slice);
    }

    pub fn newPipeline(info: PipelineInfo) *Pipeline {
        return taca_pipeline_new(ExternPipelineInfo.from(info));
    }

    pub fn newShader(bytes: []const u8) *Shader {
        return taca_shader_new(Span(u8).from(bytes));
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

pub const Step = enum(c_int) {
    vertex,
    instance,
};

pub const Texture = extern struct {};

// TODO Text metrics and rendering

pub const VertexAttribute = extern struct {
    shader_location: usize = 0,
    value_offset: usize = 0,
};

pub const VertexBufferLayout = extern struct {
    first_attribute: usize = 0,
    step: Step = .vertex,
    stride: usize = 0,
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
        taca_print(Span(u8).from(text));
    }

    pub fn setTitle(title: []const u8) void {
        taca_title_update(Span(u8).from(title));
    }

    pub fn state() WindowState {
        return taca_window_state();
    }
};

pub const WindowState = extern struct {
    pointer: [2]f32,
    press: u32,
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
    depth_test: bool,
    fragment: ExternPipelineShaderInfo,
    vertex: ExternPipelineShaderInfo,
    vertex_attributes: Span(VertexAttribute),
    vertex_buffers: Span(VertexBufferLayout),

    pub fn from(info: PipelineInfo) ExternPipelineInfo {
        return .{
            .depth_test = info.depth_test,
            .fragment = ExternPipelineShaderInfo.from(info.fragment),
            .vertex = ExternPipelineShaderInfo.from(info.vertex),
            .vertex_attributes = Span(VertexAttribute).from(info.vertex_attributes),
            .vertex_buffers = Span(VertexBufferLayout).from(info.vertex_buffers),
        };
    }
};

const ExternPipelineShaderInfo = extern struct {
    entry: Span(u8),
    shader: ?*const Shader,

    pub fn from(info: PipelineShaderInfo) ExternPipelineShaderInfo {
        return .{
            .entry = Span(u8).from(info.entry),
            .shader = info.shader,
        };
    }
};

extern fn taca_bindings_apply(
    bindings: ExternBindings,
) void;

extern fn taca_pipeline_apply(
    pipeline: *Pipeline,
) void;

extern fn taca_uniforms_apply(
    uniforms: Span(u8),
) void;

extern fn taca_RenderingContext_beginPass(
    // Nothing
) void;

extern fn taca_RenderingContext_commitFrame(
    // Nothing
) void;

extern fn taca_draw(
    item_begin: u32,
    item_count: u32,
    instance_count: u32,
) void;

extern fn taca_text_draw(
    bytes: Span(u8),
    x: f32,
    y: f32,
) void;

extern fn taca_text_drawure(
    texture: *Texture,
    x: f32,
    y: f32,
) void;

extern fn taca_RenderingContext_endPass(
    // Nothing
) void;

extern fn taca_buffer_new(
    kind: BufferKind,
    info: *const BufferSlice,
) callconv(.C) *Buffer;

extern fn taca_pipeline_new(
    info: ExternPipelineInfo,
) callconv(.C) *Pipeline;

extern fn taca_shader_new(
    bytes: Span(u8),
) callconv(.C) *Shader;

extern fn taca_Window_newRenderingContext(
    // Nothing
) callconv(.C) *RenderingContext;

extern fn taca_print(
    text: Span(u8),
) callconv(.C) void;

extern fn taca_title_update(
    title: Span(u8),
) callconv(.C) void;

extern fn taca_window_state(
    // Nothing
) callconv(.C) WindowState;
