module fly @private;
import glb;
import std::math;
import taca;

fn void start() @export("start") {
    taca::print("Hi from C3!");
    taca::title_set("Flyin'");
    Mesh! ship = glb::load($embed("ship.glb"));
    if (catch ship) {
        taca::print("Fail!");
        // TODO Failure indicator?
        // TODO Apparently, !! doesn't crash wasm?
        return;
    }
    Shader fragment = taca::shader_new($embed("../out/fragment.spv"));
    Shader vertex = taca::shader_new($embed("../out/vertex.spv"));
    MeshBuf ship_buf = {
        .idx = taca::buffer_new(INDEX, ship.idx),
        .idx_len = ship.idx.len / ushort.sizeof,
        .pos = taca::buffer_new(VERTEX, ship.pos),
        .norm = taca::buffer_new(VERTEX, ship.norm),
    };
    // taca::print(string::tformat("pos: %s", slice_as_slice(float[3], ship.pos)));
    taca::pipeline_new({
        .depth_test = true,
        .fragment = { .shader = fragment },
        .vertex = { .shader = vertex },
        .vertex_buffers = {
            {},
            { .first_attribute = 1 },
        },
    });
    game = {
        .pos = { -8, 4, -4 },
        .score = 0,
        .ship = ship_buf,
    };
}

fn void update(int event) @export("update") {
    WindowState window = taca::window_state();
    game.score = (int)window.size[0];
    float fov = (float)math::deg_to_rad(45);
    float aspect = window.size[0] / window.size[1];
    // TODO Do we want up fixed or based on some rotation angle?
    Vec3f up = { 0, 1, 0 };
    Uniforms uniforms = {
        .proj = matrix4f_perspective(fov, aspect, 0.1, 100).transpose(),
        .view = vector::matrix4f_look_at(game.pos, { 0, 0, 0 }, up).transpose(),
    };
    taca::uniforms_set(value_as_slice(char, uniforms));
    taca::binding_apply({
        .index_buffer = game.ship.idx,
        .vertex_buffers = { game.ship.pos, game.ship.norm },
    });
    taca::draw(0, game.ship.idx_len, 1);
    // Stats
    taca::text_align(LEFT, TOP);
    taca::text_draw(string::tformat("Score: %s", game.score), 10, 10);
    taca::text_align(RIGHT, TOP);
    float right = window.size[0] - 10;
    taca::text_draw(string::tformat("Time: %d", window.size[1]), right, 10);
}

macro slice_as_slice($Type, slice) {
    assert((slice.len * $sizeof(slice[0])) % $Type.sizeof == 0);
    return (($Type*)(void*)slice)[:slice.len * $sizeof(slice[0]) / $Type.sizeof];
}

macro value_as_slice($Type, value) {
    return (($Type*)&value)[:$sizeof(value)];
}

Game game;

struct Game {
    Vec3f pos;
    int score;
    MeshBuf ship;
}

struct MeshBuf {
    Buffer idx;
    usz idx_len;
    Buffer norm;
    Buffer pos;
}

struct Uniforms {
    Matrix4f proj;
    Matrix4f view;
}
