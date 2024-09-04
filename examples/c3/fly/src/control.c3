module fly @private;
import std::math;
import taca;

fn void update_control() {
    float speed = 1e-1;
    Vec3f move;
    if (game.control.up) move.y = speed;
    if (game.control.down) move.y = -speed;
    if (game.control.left) move.x = speed;
    if (game.control.right) move.x = -speed;
    game.pos += move;
    // Try to avoid the singular point. TODO Why still hitting it?
    if (math::abs(game.pos.x) < speed) game.pos.x += move.x;
    if (math::abs(game.pos.y) < speed) game.pos.y += move.y;
    game.pos.x = max(-MOVE_LIMIT, min(game.pos.x, MOVE_LIMIT));
    game.pos.y = max(-MOVE_LIMIT, min(game.pos.y, MOVE_LIMIT));
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

fn void update_targets() {
    // Prune finished targets.
    // Empties allow O(N) processing including moved slots.
    usz empty_count = 0;
    usz empty_index = 0;
    // Pruned is the actual pruned count, excluding moves.
    usz pruned_count = 0;
    foreach (i : Range { 0, game.target_count }) {
        if (game.target_offsets[i][2] < CAMERA_Z) {
            // Prune this one.
            game.target_empties[empty_count] = i;
            empty_count += 1;
            pruned_count += 1;
        } else if (empty_index < empty_count) {
            // Move to a pruned space.
            game.target_offsets[game.target_empties[empty_index]] =
                game.target_offsets[i];
            game.target_empties[empty_count] = i;
            empty_count += 1;
            empty_index += 1;
        }
    }
    game.target_count -= pruned_count;
    // Move targets.
    foreach (i : Range { 0, game.target_count }) {
        game.target_offsets[i][2] -= 0.5;
    }
    // Spawn new targets.
    float start = game.target_count ? 300 : 100;
    float scale = game.target_count ? 300 : 600;
    while (game.target_count < game.targets.limit) {
        game.target_offsets[game.target_count] = {
            // TODO Sample polar coords?
            MOVE_LIMIT * (2 * random::next_float(&game.random) - 1),
            MOVE_LIMIT * (2 * random::next_float(&game.random) - 1),
            scale * random::next_float(&game.random) + start,
        };
        // taca::print(string::tformat(
        //     "===> %s: %s",
        //     game.target_count,
        //     game.target_offsets[game.target_count],
        // ));
        game.target_count += 1;
    }
}
