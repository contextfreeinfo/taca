module fly @private;
import std::math;
import taca;

fn void update_control(Game* game) {
    Vec3f move;
    if (game.control.up) move.y = 1;
    if (game.control.down) move.y = -1;
    if (game.control.left) move.x = 1;
    if (game.control.right) move.x = -1;
    game.pos += 1e-1 * move.normalize();
    // // Try to avoid the singular point. TODO Why still hitting it?
    // if (math::abs(game.pos.x) < speed) game.pos.x += move.x;
    // if (math::abs(game.pos.y) < speed) game.pos.y += move.y;
    if (game.pos.length() > MOVE_LIMIT) {
        game.pos = MOVE_LIMIT * game.pos.normalize();
    }
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

fn void update_light(Game* game) {
    game.light_angle += 3e-1 * math::PI / 60;
    // Says only for integers.
    // game.light_angle %= 2 * math::PI;
    game.light_angle = game.light_angle % (2 * math::PI);
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
        // Reset extra data.
        multi.extras[multi.count] = {};
        // Choose position.
        float angle = 2 * math::PI * random::next_float(&game.random);
        // Set a minimum radius, and also bias toward edges with exponent.
        float radius = 1 - 0.3 * math::pow(random::next_float(&game.random), 2);
        radius *= MOVE_LIMIT;
        float z = scale * random::next_float(&game.random) + start;
        multi.offsets[multi.count] = {
            radius * math::cos(angle),
            radius * math::sin(angle),
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
