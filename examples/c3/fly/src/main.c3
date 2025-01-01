module fly @private;
import glb;
import std::math;
import taca;

Game game;

fn void! start() @export("start") {
    taca::print("Hi from C3!");
    taca::title_set("Asteroid Tunnel (Taca Demo)");
    // TODO Apparently, !! doesn't crash wasm?
    glb::Mesh danger = glb::load($embed("boulder.glb"))!;
    glb::Mesh light = glb::load($embed("light.glb"))!;
    glb::Mesh ship = glb::load($embed("ship.glb"))!;
    glb::Mesh target = glb::load($embed("gem.glb"))!;
    // taca::print(string::tformat("pos: %s", slice_as_slice(float[3], ship.pos)));
    Shader fragment = taca::shader_new($embed("../out/fragment.spv"));
    Shader vertex = taca::shader_new($embed("../out/vertex.spv"));
    Pipeline pipeline = taca::pipeline_new({
        .depth_test = true,
        .fragment = { .shader = fragment },
        .vertex = { .shader = vertex },
        .vertex_buffers = {
            {},
            { .first_attribute = 1 },
            { .first_attribute = 2, .step = INSTANCE },
        },
        // .instance_buffers = ...
    });
    Pcg32Random random;
    // This was also previously the title, and I've gotten used to the previous
    // seed, so keep it for now.
    random.set_seed("Flyin'");
    game = {
        .dangers = {
            .buf = buf_mesh(danger, pipeline, DANGER_LIMIT),
        },
        .light = buf_mesh(light, pipeline),
        .pause = 1,
        .pos = { 0, -2, 0 },
        .random = random,
        .ship = buf_mesh(ship, pipeline),
        .targets = {
            .buf = buf_mesh(target, pipeline, TARGET_LIMIT),
        },
    };
    update_multis(&game);
}

fn void update(EventKind event) @export("update") {
    @pool() {
        switch (event) {
            case FRAME: update_frame(&game);
            case KEY: update_key(&game);
        }
    };
}
