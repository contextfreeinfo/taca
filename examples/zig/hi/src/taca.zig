// Nice api.

pub const Pipeline = extern struct {};

pub const PipelineShaderInfo = struct {
    entry_point: []const u8,
    shader: *Shader,
};

pub const PipelineInfo = struct {
    fragment: PipelineShaderInfo,
    vertex: PipelineShaderInfo,
};

pub const RenderingContext = extern struct {
    pub fn newPipeline(self: *RenderingContext, info: PipelineInfo) *Pipeline {
        return taca_RenderingContext_newPipeline(self, &info);
    }
    pub fn newShader(self: *RenderingContext, bytes: []const u8) *Shader {
        return taca_RenderingContext_newShader(self, bytes.ptr, bytes.len);
    }
};

pub const Shader = extern struct {};

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
