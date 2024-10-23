#pragma once

#include <taca.hpp>
#include <vector>

namespace music {

constexpr std::size_t background_begin = 0;
constexpr std::size_t highlight_begin = 1;
constexpr std::size_t notes_begin = 3;
constexpr std::size_t max_notes = 24;
constexpr std::size_t max_ticks = 1 << 10;

struct DrawInstance {
    std::array<float, 2> offset;
    float scale;
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
