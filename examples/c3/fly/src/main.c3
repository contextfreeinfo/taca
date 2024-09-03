module fly @private;
import glb;
import std::math;
import taca;

Game game;
const char[] TITLE = "Flyin'";

fn void! start() @export("start") {
    taca::print("Hi from C3!");
    taca::title_set(TITLE);
    // TODO Apparently, !! doesn't crash wasm?
    Mesh light = glb::load($embed("light.glb"))!;
    Mesh ship = glb::load($embed("ship.glb"))!;
    Mesh target = glb::load($embed("target.glb"))!;
    // taca::print(string::tformat("pos: %s", slice_as_slice(float[3], ship.pos)));
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
    Pcg32Random random;
    random.set_seed(TITLE);
    game = {
        .light = buf_mesh(light),
        .pos = { 0, -2, 0 },
        .random = random,
        .ship = buf_mesh(ship),
        .targets = buf_mesh(target, TARGET_LIMIT),
    };
}

fn void update(EventKind event) @export("update") {
    @pool() {
        switch (event) {
            case FRAME: update_frame();
            case KEY: update_key();
        }
    };
}
