module fly @private;
import glb;
import std::math;
import taca;

fn void start() @export("start")
{
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
    taca::buffer_new(INDEX, { ship.idx, ship.idx.len });
    taca::buffer_new(VERTEX, { ship.pos, ship.pos.len });
    taca::buffer_new(VERTEX, { ship.norm, ship.norm.len });
    taca::pipeline_new({
        .fragment = { .shader = fragment },
        .vertex = { .shader = vertex },
    });
    game = {
        .pos = { 0, 4, -10 },
        .score = 0,
        .ship = ship,
    };
}

fn void update(int event) @export("update")
{
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
    taca::uniforms_set({ &uniforms, $sizeof(uniforms) });
    taca::draw(0, game.ship.idx.len / ushort.sizeof, 1);
    // Stats
    taca::text_align(LEFT, TOP);
    taca::text_draw(string::tformat("Score: %s", game.score), 10, 10);
    taca::text_align(RIGHT, TOP);
    float right = window.size[0] - 10;
    taca::text_draw(string::tformat("Time: %d", window.size[1]), right, 10);
}

Game game;

struct Game
{
    Vec3f pos;
    int score;
    Mesh ship;
}

struct Uniforms
{
    Matrix4f proj;
    Matrix4f view;
}
