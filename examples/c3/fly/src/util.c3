module fly @private;
import glb;
import std::collections::range;
import std::math;
import taca;

const float CAMERA_Z = -12;
const usz DANGER_LIMIT = 80;
const usz MAX_LIMIT = DANGER_LIMIT;
const float MOVE_LIMIT = 4;
const usz TARGET_LIMIT = 20;

fn MeshBuf buf_mesh(glb::Mesh mesh, usz limit = 1) {
    return {
        .bindings = uniform_bindings_new(),
        .idx = taca::buffer_new(INDEX, mesh.idx),
        .idx_len = mesh.idx.len / ushort.sizeof,
        .limit = limit,
        .norm = taca::buffer_new(VERTEX, mesh.norm),
        .offset = taca::buffer_new(VERTEX, null[:3 * limit * float.sizeof]),
        .pos = taca::buffer_new(VERTEX, mesh.pos),
    };
}

fn void buf_mesh_bindings_apply(MeshBuf* buf, Uniforms* uniforms) {
    taca::buffer_update(buf.bindings.buffer, value_as_slice(char, *uniforms));
    taca::bindings_apply(buf.bindings.bindings);
}

fn char[] sized_slice(usz size) {
    return ((char*)null)[:size];
}

macro slice_as_slice($Type, slice) {
    assert((slice.len * $sizeof(slice[0])) % $Type.sizeof == 0);
    return (($Type*)(void*)slice)[:slice.len * $sizeof(slice[0]) / $Type.sizeof];
}

fn BufferBindings uniform_bindings_new() {
    Buffer buffer = taca::buffer_new(UNIFORM, sized_slice(Uniforms.sizeof));
    return {
        .bindings = taca::bindings_new({.buffers = {buffer}}),
        .buffer = buffer,
    };
}

macro value_as_slice($Type, value) {
    return (($Type*)&value)[:$sizeof(value)];
}

struct Control {
    bool up;
    bool down;
    bool left;
    bool right;
}

struct Game {
    Control control;
    uint count;
    Multi dangers;
    MeshBuf light;
    float light_angle;
    uint pause;
    Vec3f pos;
    uint press;
    Pcg32Random random;
    int score;
    MeshBuf ship;
    Vec2f window_size;
    Multi targets;
}

struct MeshBuf {
    BufferBindings bindings;
    Buffer idx;
    usz idx_len;
    usz limit;
    Buffer norm;
    Buffer offset;
    Buffer pos;
}

struct Multi {
    MeshBuf buf;
    usz count;
    ObjExtra[MAX_LIMIT] extras;
    float[3][MAX_LIMIT] offsets;
}

struct ObjExtra {
    bool hit;
    Vec2f kick;
}

def Range = ExclusiveRange(<int>);

struct BufferBindings {
    Bindings bindings;
    Buffer buffer;
}

struct Uniforms {
    Matrix4f proj;
    Matrix4f view;
    float[3] color;
    float pad_color;
    float[3] light;
    float lit;
}
