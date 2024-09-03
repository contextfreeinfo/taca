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
    float limit = 2;
    game.pos.x = max(-limit, min(game.pos.x, limit));
    game.pos.y = max(-limit, min(game.pos.y, limit));
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
