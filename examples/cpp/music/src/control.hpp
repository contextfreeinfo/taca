#pragma once

#include "app.hpp"
#include <taca.hpp>

namespace music {

auto play_ding(const App& app, float semitones) -> void {
    taca::sound_play({
        .sound = app.ding,
        .rate = semitones,
        .rate_kind = taca::SoundRateKind::Semitones,
    });
}

auto update_click(App& app) -> void {
    auto bands = calc_bands(app);
    if (!bands.active) {
        return;
    }
    auto cell = bands.cell_index;
    auto& ticks = app.song.ticks;
    if (ticks.size() < cell[0] + 1) {
        ticks.resize(cell[0] + 1);
    }
    auto semitones = static_cast<float>(semitones_default) - cell[1];
    // print("Click: %zu %zu %f", cell[0], cell[1], semitones);
    auto& notes = ticks[cell[0]].notes;
    auto existing =
        std::find_if(notes.begin(), notes.end(), [semitones](Note& note) {
            // Equality on floats is brave, but we're pretty controlled on them.
            // Less brave would be to use ints or to use some epsilon.
            return note.semitones == semitones;
        });
    auto add = app.draw_mode == DrawMode::Start
        ? existing == notes.end()
        : app.draw_mode == DrawMode::Add;
    if (add) {
        app.draw_mode = DrawMode::Add;
        if (existing == notes.end()) {
            // Missing so add.
            notes.push_back({.semitones = semitones});
            play_ding(app, semitones);
        }
    } else {
        app.draw_mode = DrawMode::Remove;
        if (existing != notes.end()) {
            // Found so remove.
            notes.erase(existing);
        }
    }
}

auto update_control(App& app) -> void {
    if (app.window_state.press) {
        if (app.draw_mode == DrawMode::Start) {
            // TODO Press event instead of this hacking.
            if (!app.was_pressed) {
                update_click(app);
            }
        } else {
            update_click(app);
        }
    } else {
        app.draw_mode = DrawMode::Start;
    }
    app.was_pressed = app.window_state.press;
}

} // namespace music
