module fly @private;
import std::math;
import taca;

macro slice_as_slice($Type, slice) {
    assert((slice.len * $sizeof(slice[0])) % $Type.sizeof == 0);
    return (($Type*)(void*)slice)[:slice.len * $sizeof(slice[0]) / $Type.sizeof];
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
    Vec3f pos;
    int score;
    MeshBuf ship;
}

struct MeshBuf {
    Buffer idx;
    usz idx_len;
    Buffer norm;
    Buffer offset;
    Buffer pos;
}

struct Uniforms {
    Matrix4f proj;
    Matrix4f view;
}
