#pragma once

#include "app.hpp"
#include <taca.hpp>

namespace music {

void play_ding(const App& app) {
    taca::sound_play({.sound = app.ding});
}

void update_control(App& app) {
    // TODO Press event instead of this hacking.
    if (app.window_state.press && !app.was_pressed) {
        play_ding(app);
    }
    app.was_pressed = app.window_state.press;
}

} // namespace music
