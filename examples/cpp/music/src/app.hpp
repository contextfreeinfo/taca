#pragma once

#include <cstdio>
#include <taca.hpp>
#include <vec.hpp>
#include <vector>

namespace music {

// The top of our scale is C7, and our sample is D6.
constexpr auto semitones_default = 11.0f;

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
    // Overly flexible for the current app, but meh.
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

struct BandInfo {
    bool active;
    vec::Vec2f bands_offset;
    vec::Vec2f bands_scale;
    std::array<size_t, 2> cell_index;
    vec::Vec2f cell_offset;
    vec::Vec2f cell_scale;
    vec::Vec2f cell_start;
};

auto calc_bands(App& app) -> BandInfo {
    using namespace vec;
    auto window_size = Vec2f{
        app.window_state.size.x,
        app.window_state.size.y,
    };
    // auto window_size = std::to_array<float>({1, 1});
    auto pointer = Vec2f{
        app.window_state.pointer.x,
        app.window_state.pointer.y,
    };
    // auto pointer = pointer_px / window_size_px;
    auto margin = Vec2f{40, 40};
    auto music_size = window_size - margin;
    auto music_pos_frac = (pointer - margin) / music_size;
    auto active = (0 <= music_pos_frac[0] && music_pos_frac[0] <= 1) &&
        (0 <= music_pos_frac[1] && music_pos_frac[1] <= 1);
    auto grid_count = Vec2f{max_ticks, max_pitches};
    auto cell = floor(music_pos_frac * grid_count);
    auto grid_pos_frac = cell / grid_count + 0.5 / grid_count;
    auto grid_pos = grid_pos_frac * music_size + margin;
    auto table_margin = Vec2f{0, margin[1]};
    auto table_size = Vec2f{window_size[0], music_size[1]};
    auto cell_scale = music_size / grid_count / window_size;
    return {
        .active = active,
        .bands_offset = (2 * table_margin + table_size) / window_size - 1,
        .bands_scale = table_size / window_size,
        .cell_index =
            {static_cast<size_t>(cell[0]), static_cast<size_t>(cell[1])},
        .cell_offset = 2 * (grid_pos / window_size) - 1,
        .cell_scale = cell_scale,
        .cell_start = 2 * margin / window_size - 1 + cell_scale,
    };
}

template <typename... Args> void print(const char* format, Args... args) {
    char buffer[100];
    std::snprintf(buffer, sizeof(buffer), format, args...);
    taca::print(buffer);
}

} // namespace music
