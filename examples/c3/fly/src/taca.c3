module taca;

// Functions

extern fn void binding_apply(
    Binding binding,
) @extern("taca_RenderingContext_applyBindings");

extern fn Buffer buffer_new(
    BufferKind kind,
    char[] bytes,
) @extern("taca_RenderingContext_newBuffer");

extern fn void buffer_update(
    Buffer buffer,
    char[] bytes,
    usz buffer_offset = 0,
) @extern("taca_buffer_update");

extern fn void draw(
    uint item_begin,
    uint item_count,
    uint instance_count,
) @extern("taca_RenderingContext_draw");

extern fn KeyEvent key_event(
    // Nothing
) @extern("taca_key_event");

extern fn void pipeline_apply(
    Pipeline pipeline,
) @extern("taca_RenderingContext_applyPipeline");

extern fn Pipeline pipeline_new(
    PipelineInfo info,
) @extern("taca_RenderingContext_newPipeline");

extern fn void print(
    char[] text,
) @extern("taca_Window_print");

extern fn Shader shader_new(
    char[] bytes,
) @extern("taca_RenderingContext_newShader");

extern fn void text_align(
    TextAlignX x,
    TextAlignY y,
) @extern("taca_textAlign");

extern fn void text_draw(
    char[] bytes,
    float x,
    float y,
) @extern("taca_RenderingContext_drawText");

extern fn void title_set(
    char[] title,
) @extern("taca_Window_setTitle");

extern fn void uniforms_update(
    char[] bytes,
) @extern("taca_RenderingContext_applyUniforms");

extern fn WindowState window_state(
    // Nothing
) @extern("taca_Window_state");

// Types

struct AttributeInfo {
    usz shader_location;
    usz value_offset;
}

struct Binding {
    Buffer[] vertex_buffers;
    Buffer index_buffer;
}

distinct Buffer = void*;

struct BufferInfo {
    usz first_attribute;
    Step step;
    usz stride;
}

enum BufferKind : uint {
    VERTEX,
    INDEX,
}

struct BufferUpdate {
    Buffer buffer;
    char[] bytes;
    usz begin;
    usz item_size;
    usz stride;
}

enum EventKind : int {
    FRAME,
    KEY,
}

enum Key : int {
    NONE,
    ARROW_UP,
    ARROW_DOWN,
    ARROW_LEFT,
    ARROW_RIGHT,
}

struct KeyEvent {
    Key key;
    bool pressed;
}

distinct Pipeline = void*;

struct PipelineInfo {
    bool depth_test;
    PipelineShaderInfo fragment;
    PipelineShaderInfo vertex;
    AttributeInfo[] vertex_attributes;
    BufferInfo[] vertex_buffers;
}

struct PipelineShaderInfo {
    char[] entry;
    Shader shader;
}

distinct Shader = void*;

enum Step : uint {
    VERTEX,
    INSTANCE,
}

enum TextAlignX : uint {
    LEFT,
    CENTER,
    RIGHT,
}

enum TextAlignY : uint {
    BASELINE,
    TOP,
    MIDDLE,
    BOTTOM,
}

struct WindowState {
    float[2] pointer;
    float[2] size;
}
