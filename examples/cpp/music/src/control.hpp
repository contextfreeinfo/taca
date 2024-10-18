#pragma once

#include "app.hpp"
#include <taca.hpp>

namespace music {

void play_ding(const App& app, float semitones) {
    taca::sound_play({
        .sound = app.ding,
        .rate = semitones,
        .rate_kind = taca::SoundRateKind::Semitones,
    });
}

void update_control(App& app) {
    // TODO Press event instead of this hacking.
    if (app.window_state.press && !app.was_pressed) {
        auto scale =
            1 - 2 * app.window_state.pointer.y / app.window_state.size.y;
        play_ding(app, 12 * scale);
    }
    app.was_pressed = app.window_state.press;
}

} // namespace music
