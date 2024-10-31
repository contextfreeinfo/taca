#pragma once

#include "app.hpp"
#include <taca.hpp>
#include <vec.hpp>

namespace music {

auto play_ding(const App& app, float semitones) -> void {
    taca::sound_play({
        .sound = app.ding,
        .rate = semitones,
        .rate_kind = taca::SoundRateKind::Semitones,
    });
}

auto rewind(PlayInfo& play_info) -> void {
    play_info.tick = 0;
    play_info.frames_until_tick = 0;
}

auto toggle_play(PlayInfo& play_info) -> void {
    play_info.playing = !play_info.playing;
}

auto update_click(App& app) -> void {
    using namespace vec;
    auto bands = calc_bands(app);
    // print(
    //     "index: %d %d %d",
    //     bands.cell_index[0].has_value(),
    //     bands.cell_index[1].has_value(),
    //     bands.active
    // );
    if (!(bands.cell_index[0].has_value() && bands.cell_index[1].has_value())) {
        if (bands.cell_index[0].has_value()) {
            app.play_info.tick = *bands.cell_index[0];
            app.play_info.frames_until_tick = 0;
        } else {
            auto pointer = bands.pointer;
            auto extent = bands.button_scale;
            if (inside(pointer, bands.button_play_offset, extent)) {
                toggle_play(app.play_info);
            } else if (inside(pointer, bands.button_back_offset, extent)) {
                rewind(app.play_info);
            }
        }
        return;
    }
    auto cell = vec::map<std::size_t>(bands.cell_index, [](auto index) {
        return index.value_or(0);
    });
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

auto update_press(App& app) -> void {
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

auto update_play(App& app) -> void {
    auto& play_info = app.play_info;
    if (play_info.playing) {
        if (play_info.frames_until_tick) {
            play_info.frames_until_tick -= 1;
        } else {
            // print("Tick: %zu", play_info.tick);
            // Half second per tick at the moment.
            // TODO Use tempo control from song or app or something.
            play_info.frames_until_tick = frames_per_tick - 1;
            if (play_info.tick < app.song.ticks.size()) {
                const auto& tick = app.song.ticks[play_info.tick];
                for (const auto& note : tick.notes) {
                    play_ding(app, note.semitones);
                }
            }
            play_info.tick = (play_info.tick + 1) % max_ticks;
        }
    }
}

auto update_control(App& app) -> void {
    update_play(app);
    update_press(app);
}

auto update_key(App& app, taca::KeyEvent event) -> void {
    switch (event.key) {
        case taca::Key::Escape: {
            rewind(app.play_info);
            break;
        }
        case taca::Key::Space: {
            toggle_play(app.play_info);
            break;
        }
        default:
    }
}

} // namespace music
