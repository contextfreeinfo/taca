module fly @private;
import glb;
import taca;

Game game;

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
        .pos = { 0, -2, 0 },
        .score = 0,
        .ship = ship_buf,
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
