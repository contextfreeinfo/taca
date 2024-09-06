module fly @private;
import std::math;
import taca;

fn void update_control(Game* game) {
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
    game.count += 1;
}

fn void update_key(Game* game) {
    KeyEvent event = taca::key_event();
    // taca::print(string::tformat("key: %s %s", event.key, event.pressed));
    switch (event.key) {
        case ARROW_UP: game.control.up = event.pressed;
        case ARROW_DOWN: game.control.down = event.pressed;
        case ARROW_LEFT: game.control.left = event.pressed;
        case ARROW_RIGHT: game.control.right = event.pressed;
        case SPACE: if (event.pressed) game.paused = !game.paused;
        default:
    }
}

fn void update_multi(Game* game, Multi* multi, bool prune_hit) {
    // Prune finished targets.
    // Empties allow O(N) processing including moved slots.
    usz[] empties = mem::temp_new_array(usz, multi.count);
    usz empty_count = 0;
    usz empty_index = 0;
    // Pruned is the actual pruned count, excluding moves.
    // TODO Check also for collisions.
    usz pruned_count = 0;
    foreach (i : Range { 0, multi.count }) {
        bool hit = game.pos.distance(multi.offsets[i]) < 1;
        if (hit) {
            if (prune_hit) {
                game.score += 1;
            } else {
                if (!multi.extras[i].hit) {
                    game.score -= 1;
                    multi.extras[i].hit = true;
                }
                Vec2f kick = game.pos.xy - ((Vec3f)multi.offsets[i]).xy;
                kick = 0.1 * kick.normalize();
                game.pos.x += kick.x;
                game.pos.y += kick.y;
                multi.extras[i].kick = -kick;
            }
        }
        if (multi.offsets[i][2] < CAMERA_Z || prune_hit && hit) {
            // Prune this one.
            empties[empty_count] = i;
            empty_count += 1;
            pruned_count += 1;
        } else if (empty_index < empty_count) {
            // Move to a pruned space.
            usz to = empties[empty_index];
            multi.extras[to] = multi.extras[i];
            multi.offsets[to] = multi.offsets[i];
            empties[empty_count] = i;
            empty_count += 1;
            empty_index += 1;
        }
    }
    multi.count -= pruned_count;
    // Move targets.
    foreach (i : Range { 0, multi.count }) {
        multi.offsets[i][0] += multi.extras[i].kick.x;
        multi.offsets[i][1] += multi.extras[i].kick.y;
        multi.offsets[i][2] -= 0.5;
    }
    // Spawn new targets.
    float start = multi.count ? 300 : 100;
    float scale = multi.count ? 300 : 600;
    while (multi.count < multi.buf.limit) {
        multi.extras[multi.count] = {};
        multi.offsets[multi.count] = {
            // TODO Sample polar coords?
            MOVE_LIMIT * (2 * random::next_float(&game.random) - 1),
            MOVE_LIMIT * (2 * random::next_float(&game.random) - 1),
            scale * random::next_float(&game.random) + start,
        };
        // taca::print(string::tformat(
        //     "===> %s: %s",
        //     multi.count,
        //     multi.offsets[multi.count],
        // ));
        multi.count += 1;
    }
}
