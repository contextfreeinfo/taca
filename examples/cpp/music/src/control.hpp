#pragma once

#include "app.hpp"
#include <taca.hpp>

namespace music {

void update_control(App* app) {
    if (!app->ready) {
        return;
    }
    // TODO Press event instead of this hacking.
    if (app->window_state.press && !app->was_pressed) {
        taca::sound_play({.sound = app->ding});
    }
}

} // namespace music
