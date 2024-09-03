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
    taca::pipeline_new({
        .depth_test = true,
        .fragment = { .shader = fragment },
        .vertex = { .shader = vertex },
        .vertex_buffers = {
            {},
            { .first_attribute = 1 },
            { .first_attribute = 2, .step = INSTANCE },
        },
    });
    MeshBuf ship_buf = {
        .idx = taca::buffer_new(INDEX, ship.idx),
        .idx_len = ship.idx.len / ushort.sizeof,
        .norm = taca::buffer_new(VERTEX, ship.norm),
        .offset = taca::buffer_new(VERTEX, null[:3 * float.sizeof]),
        .pos = taca::buffer_new(VERTEX, ship.pos),
    };
    // taca::print(string::tformat("pos: %s", slice_as_slice(float[3], ship.pos)));
    game = {
        .pos = { 0, 0, 0 },
        .score = 0,
        .ship = ship_buf,
    };
}

fn void update(EventKind event) @export("update") {
    @pool() {
        switch (event) {
            case KEY: update_key();
            case TICK: update_tick();
        }
    };
}

fn void update_control() {
    float speed = 1e-1;
    if (game.control.up) game.pos.y += speed;
    if (game.control.down) game.pos.y -= speed;
    if (game.control.left) game.pos.x += speed;
    if (game.control.right) game.pos.x -= speed;
    float limit = 2;
    game.pos.x = max(-limit, min(game.pos.x, limit));
    game.pos.y = max(-limit, min(game.pos.y, limit));
}

fn void update_key() {
    KeyEvent event = taca::key_event();
    // taca::print(string::tformat("key: %s %s", event.key, event.pressed));
    switch (event.key) {
        case ARROW_UP: game.control.up = event.pressed;
        case ARROW_DOWN: game.control.down = event.pressed;
        case ARROW_LEFT: game.control.left = event.pressed;
        case ARROW_RIGHT: game.control.right = event.pressed;
        default:
    }
}

fn void update_tick() {
    WindowState window = taca::window_state();
    game.score = (int)window.size[0];
    float fov = (float)math::deg_to_rad(45);
    float aspect = window.size[0] / window.size[1];
    // TODO Do we want up fixed or based on some rotation angle?
    Vec3f at = { 0, 0, 0 };
    Vec3f up = { 0, 1, 0 };
    Uniforms uniforms = {
        .proj = matrix4f_perspective(fov, aspect, 0.1, 100).transpose(),
        .view = vector::matrix4f_look_at({ 0, 0, -8 }, at, up).transpose(),
    };
    taca::uniforms_update(value_as_slice(char, uniforms));
    taca::binding_apply({
        .index_buffer = game.ship.idx,
        .vertex_buffers = { game.ship.pos, game.ship.norm, game.ship.offset },
    });
    update_control();
    float[3] pos = game.pos;
    taca::buffer_update(game.ship.offset, value_as_slice(char, pos));
    taca::draw(0, game.ship.idx_len, 1);
    // Stats
    taca::text_align(LEFT, TOP);
    taca::text_draw(string::tformat("Score: %s", game.score), 10, 10);
    taca::text_align(RIGHT, TOP);
    float right = window.size[0] - 10;
    taca::text_draw(string::tformat("Time: %d", window.size[1]), right, 10);
    // Update
    game.count += 1;
}

macro slice_as_slice($Type, slice) {
    assert((slice.len * $sizeof(slice[0])) % $Type.sizeof == 0);
    return (($Type*)(void*)slice)[:slice.len * $sizeof(slice[0]) / $Type.sizeof];
}

macro value_as_slice($Type, value) {
    return (($Type*)&value)[:$sizeof(value)];
}

Game game;

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
