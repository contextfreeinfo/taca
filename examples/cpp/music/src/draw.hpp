#pragma once

#include "app.hpp"
#include <array>
#include <taca.hpp>
#include <vec.hpp>

namespace music {

auto calc_bands(App& app) -> vec::Vec2f {
    using namespace vec;
    auto window_size_px = Vec2f{
        app.window_state.size.x,
        app.window_state.size.y,
    };
    // auto window_size = std::to_array<float>({1, 1});
    auto pointer_px = Vec2f{
        app.window_state.pointer.x,
        app.window_state.pointer.y,
    };
    auto pointer = pointer_px / window_size_px;
    auto margin = Vec2f{0, 40} / window_size_px;
    auto music_size = Vec2f{1, 1 - 2 * margin[1]};
    auto music_start = Vec2f{0, margin[1]};
    auto music_pos = (pointer - music_start) / music_size;
    auto grid_count = Vec2f{max_ticks, max_pitches};
    auto grid_pos = floor(music_pos * grid_count) / grid_count;
    return grid_pos - Vec2f{0.5, 0.5} + music_start;
}

auto draw(App& app) -> void {
    auto& instance_values = app.draw_info.instance_values;
    auto bands = calc_bands(app);
    instance_values.clear();
    instance_values.push_back({
        .offset = bands,
        .scale = {0.05, 1},
        .light = 1,
    });
    instance_values.push_back({
        .offset = bands,
        .scale = {1, 0.05},
        .light = 1,
    });
    instance_values.push_back({
        .scale = {1, 1},
        .light = 0.8,
    });
    taca::buffer_update(
        app.draw_info.instance_buffer,
        std::as_bytes(std::span{instance_values}),
        0
    );
    auto vertex_buffers = std::to_array(
        {app.draw_info.vertex_buffer, app.draw_info.instance_buffer}
    );
    taca::buffers_apply({
        .vertex_buffers = std::span{vertex_buffers},
        .index_buffer = app.draw_info.index_buffer,
    });
    taca::draw(0, 6, instance_values.size());
    // Text
    taca::text_align(taca::TextAlignX::Left, taca::TextAlignY::Top);
    taca::text_draw("Music Box", 3, 3);
}

} // namespace music
