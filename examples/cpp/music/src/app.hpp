#pragma once

#include <taca.hpp>
#include <vector>

namespace music {

// Rects include notes, highlight bands, background, what else?
// Max extra might should be just 3?
constexpr std::size_t max_extra_rects = 10;
constexpr std::size_t max_pitches = 25;
constexpr std::size_t max_ticks = 32;

struct DrawInstance {
    std::array<float, 2> offset;
    std::array<float, 2> scale;
    float light;
};

struct DrawInfo {
    taca::Buffer index_buffer;
    taca::Buffer vertex_buffer;
    taca::Buffer instance_buffer;
    std::vector<DrawInstance> instance_values;
};

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
    DrawInfo draw_info;
    bool ready;
    Song song;
    bool was_pressed;
    taca::WindowState window_state;
};

} // namespace music
