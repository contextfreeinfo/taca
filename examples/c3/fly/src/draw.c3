module fly @private;
import std::math;
import taca;

fn void update_frame() {
    // Move
    update_multi(&game.dangers);
    update_multi(&game.targets);
    update_control();
    // Draw
    WindowState window = taca::window_state();
    game.score = (int)window.size[0];
    float fov = (float)math::deg_to_rad(45);
    float aspect = window.size[0] / window.size[1];
    // TODO Do we want up fixed or based on some rotation angle?
    Vec3f at = { 0, 0, 0 };
    Vec3f camera = { 0, 0, CAMERA_Z };
    Vec3f up = { 0, 1, 0 };
    Uniforms uniforms = {
        .proj = matrix4f_perspective(fov, aspect, 0.1, 1000).transpose(),
        .view = vector::matrix4f_look_at(camera, at, up).transpose(),
    };
    taca::uniforms_update(value_as_slice(char, uniforms));
    // Objects
    draw_multi(game.dangers);
    draw_multi(game.targets);
    // Ship
    taca::binding_apply({
        .index_buffer = game.ship.idx,
        .vertex_buffers = { game.ship.pos, game.ship.norm, game.ship.offset },
    });
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

fn void draw_multi(Multi multi) {
    taca::binding_apply({
        .index_buffer = multi.buf.idx,
        .vertex_buffers = { multi.buf.pos, multi.buf.norm, multi.buf.offset },
    });
    taca::buffer_update(
        multi.buf.offset, slice_as_slice(char, multi.offsets[:multi.count])
    );
    taca::draw(0, multi.buf.idx_len, multi.count);
}
