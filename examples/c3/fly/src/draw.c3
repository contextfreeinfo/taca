module fly @private;
import std::math;
import taca;

fn void update_frame(Game* game) {
    // Move
    if (!game.paused) {
        update_multi(game, &game.dangers, false);
        update_multi(game, &game.targets, true);
        update_control(game);
    }
    // Draw
    WindowState window = taca::window_state();
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
    // Objects
    uniforms.color = { 0.3, 0.3, 0.4 };
    taca::uniforms_update(value_as_slice(char, uniforms));
    draw_multi(game.dangers);
    uniforms.color = { 0.7, 1, 0.2 };
    taca::uniforms_update(value_as_slice(char, uniforms));
    draw_multi(game.targets);
    // Ship
    uniforms.color = { 0.6, 0.7, 1 };
    taca::uniforms_update(value_as_slice(char, uniforms));
    taca::binding_apply({
        .index_buffer = game.ship.idx,
        .vertex_buffers = { game.ship.pos, game.ship.norm, game.ship.offset },
    });
    float[3] pos = game.pos;
    taca::buffer_update(game.ship.offset, value_as_slice(char, pos));
    taca::draw(0, game.ship.idx_len, 1);
    // Stats
    taca::text_align(LEFT, TOP);
    taca::text_draw(string::tformat("%s", game.score), 10, 10);
    taca::text_align(RIGHT, TOP);
    float right = window.size[0] - 10;
    int seconds = game.count / 60;
    int minutes = seconds / 60;
    seconds %= 60;
    taca::text_draw(string::tformat("%d:%02d", minutes, seconds), right, 10);
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
