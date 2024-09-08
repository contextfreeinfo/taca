module taca;

// Functions

extern fn void binding_apply(
    Binding binding,
) @extern("taca_bindings_apply");

extern fn Buffer buffer_new(
    BufferKind kind,
    char[] bytes,
) @extern("taca_buffer_new");

extern fn void buffer_update(
    Buffer buffer,
    char[] bytes,
    usz buffer_offset = 0,
) @extern("taca_buffer_update");

extern fn void draw(
    uint item_begin,
    uint item_count,
    uint instance_count,
) @extern("taca_draw");

extern fn KeyEvent key_event(
    // Nothing
) @extern("taca_key_event");

extern fn void pipeline_apply(
    Pipeline pipeline,
) @extern("taca_pipeline_apply");

extern fn Pipeline pipeline_new(
    PipelineInfo info,
) @extern("taca_pipeline_new");

extern fn void print(
    char[] text,
) @extern("taca_print");

extern fn Shader shader_new(
    char[] bytes,
) @extern("taca_shader_new");

extern fn void text_align(
    TextAlignX x,
    TextAlignY y,
) @extern("taca_text_align");

extern fn void text_draw(
    char[] bytes,
    float x,
    float y,
) @extern("taca_text_draw");

extern fn void title_set(
    char[] title,
) @extern("taca_title_update");

extern fn void uniforms_update(
    char[] bytes,
) @extern("taca_uniforms_apply");

extern fn WindowState window_state(
    // Nothing
) @extern("taca_window_state");

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
    SPACE,
    ESCAPE,
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
    uint press;
    float[2] size;
}
