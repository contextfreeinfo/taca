// Nice api.

pub const RenderingContext = extern struct {
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

extern fn taca_RenderingContext_newShader(
    context: *RenderingContext,
    bytes: [*c]const u8,
    bytesLength: usize,
) callconv(.C) *Shader;

extern fn taca_Window_get() callconv(.C) *Window;

extern fn taca_Window_newRenderingContext(
    window: *Window,
) callconv(.C) *RenderingContext;
