#pragma once

#include <taca.hpp>
#include <vector>

namespace music {

struct Note {
    float semitones;
};

struct Tick {
    std::vector<Note> notes;
};

struct Song {
    float ticks_per_second;
    std::vector<Tick> ticks;
};

struct App {
    taca::Sound ding;
    bool ready;
    Song song;
    bool was_pressed;
    taca::WindowState window_state;
};

} // namespace music
