module fly @private;
import glb;
import std::math;
import taca;

fn MeshBuf buf_mesh(Mesh mesh, usz limit = 1) {
    return {
        .idx = taca::buffer_new(INDEX, mesh.idx),
        .idx_len = mesh.idx.len / ushort.sizeof,
        .limit = limit,
        .norm = taca::buffer_new(VERTEX, mesh.norm),
        .offset = taca::buffer_new(VERTEX, null[:3 * limit * float.sizeof]),
        .pos = taca::buffer_new(VERTEX, mesh.pos),
    };
}

macro slice_as_slice($Type, slice) {
    assert((slice.len * $sizeof(slice[0])) % $Type.sizeof == 0);
    return (($Type*)(void*)slice)[:slice.len * $sizeof(slice[0]) / $Type.sizeof];
}

macro value_as_slice($Type, value) {
    return (($Type*)&value)[:$sizeof(value)];
}

const usz TARGET_LIMIT = 100;

struct Control {
    bool up;
    bool down;
    bool left;
    bool right;
}

struct Game {
    Control control;
    uint count;
    MeshBuf light;
    Vec3f pos;
    Pcg32Random random;
    int score;
    MeshBuf ship;
    usz target_count;
    float[3][TARGET_LIMIT] target_offsets;
    MeshBuf targets;
}

struct MeshBuf {
    Buffer idx;
    usz idx_len;
    usz limit;
    Buffer norm;
    Buffer offset;
    Buffer pos;
}

struct Uniforms {
    Matrix4f proj;
    Matrix4f view;
}
