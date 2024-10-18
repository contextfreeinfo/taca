#pragma once

#include <taca.hpp>
#include <cstddef>
#include <bitset>
#include <vector>

namespace music {

constexpr std::size_t max_notes = 24;

struct Tick {
    std::bitset<max_notes> notes;
};

struct Song {
    float tempo;
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
