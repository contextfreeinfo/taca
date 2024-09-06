module fly @private;
import std::math;
import taca;

fn void update_frame(Game* game) {
    WindowState window = taca::window_state();
    // Move
    // TODO If paused, only redraw if window size has changed?
    if (!game.pause) {
        update_light(game);
        update_multis(game);
        update_control(game);
    }
    if (game.pause) {
        if (game.pause > 1 && (Vec2f)window.size == game.window_size) {
            // Save on compute.
            return;
        }
        if (game.pause < 2) game.pause += 1;
        // taca::print("window size change on pause");
    }
    game.window_size = window.size;
    // Draw
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
    draw_light(game, &uniforms);
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
    if (game.pause) {
        taca::text_align(CENTER, TOP);
        taca::text_draw(
            "Space to Play     Arrows to Move",
            window.size[0] / 2,
            window.size[1] / 4,
        );
    }
}

fn void draw_light(Game* game, Uniforms* uniforms) {
    uniforms.lit = 0.8;
    uniforms.color = { 1, 1, 1 };
    Vec3f pos = 1.2 * MOVE_LIMIT * Vec3f {
        math::cos(game.light_angle),
        math::sin(game.light_angle),
        0,
    };
    uniforms.light = -pos;
    taca::binding_apply({
        .index_buffer = game.light.idx,
        .vertex_buffers = {
            game.light.pos, game.light.norm, game.light.offset
        },
    });
    float[3] pos_array = pos;
    taca::buffer_update(game.light.offset, value_as_slice(char, pos_array));
    taca::uniforms_update(value_as_slice(char, *uniforms));
    taca::draw(0, game.light.idx_len, 1);
    uniforms.light = pos;
    uniforms.lit = 0.5;
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
