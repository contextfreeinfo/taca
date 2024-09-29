module taca;

// Functions

extern fn Bindings bindings_apply(
    Bindings bindings,
    // TODO ??? char[] uniform_bytes, // leave empty to avoid update
) @extern("taca_bindings_apply");

extern fn Bindings bindings_new(
    BindingsInfo info,
) @extern("taca_bindings_new");

extern fn Buffer buffer_new(
    BufferKind kind,
    char[] bytes,
) @extern("taca_buffer_new");

extern fn void buffer_update(
    Buffer buffer,
    char[] bytes,
    usz buffer_offset = 0,
) @extern("taca_buffer_update");

extern fn void buffers_apply(
    // TODO Mesh mesh,
    // TODO Does mesh make sense vs instance buffers?
    MeshBuffers mesh,
) @extern("taca_buffers_apply");

extern fn void draw(
    uint item_begin,
    uint item_count,
    uint instance_count,
) @extern("taca_draw");

extern fn KeyEvent key_event(
    // Nothing
) @extern("taca_key_event");

// extern fn Mesh mesh_new(
//     MeshBuffers mesh,
// ) @extern("taca_mesh_new");

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

// TODO Combine into slice of text areas, each with alignment? Optional bounds?
extern fn void text_draw(
    char[] bytes,
    float x,
    float y,
) @extern("taca_text_draw");

extern fn void title_set(
    char[] title,
) @extern("taca_title_update");

// TODO Allowed at most once per bindings per frame? No op after?
extern fn void uniforms_update(
    // TODO ??? usz binding, // Or always just one?
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

distinct Bindings = void*;

struct BindingsInfo {
    Pipeline pipeline;
    usz group_index;
    // TODO ??? Sampler[] samplers;
    Texture[] textures;
}

distinct Buffer = void*;

struct BufferInfo {
    usz first_attribute;
    Step step; // TODO Instead have a single first instance index?
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

// distinct Mesh = void*;

struct MeshBuffers {
    // TODO Separate instance buffers from vertex buffers?
    Buffer[] vertex_buffers;
    Buffer index_buffer;
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

distinct Texture = void*;

struct WindowState {
    float[2] pointer;
    uint press;
    float[2] size;
}
