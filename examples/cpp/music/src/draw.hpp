#pragma once

#include "app.hpp"
#include <array>
#include <taca.hpp>
#include <vec.hpp>

namespace music {

auto calc_bands(App& app) -> DrawInstance {
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
    auto margin = Vec2f{0, 40};
    auto music_size = window_size - 2 * margin;
    auto music_pos_frac = (pointer - margin) / music_size;
    auto all_in = (0 <= music_pos_frac[0] && music_pos_frac[0] <= 1) &&
        (0 <= music_pos_frac[1] && music_pos_frac[1] <= 1);
    auto grid_count = Vec2f{max_ticks, max_pitches};
    auto grid_pos_frac = floor(music_pos_frac * grid_count) / grid_count +
        0.5 / grid_count;
    auto grid_pos = grid_pos_frac * music_size + margin;
    return {
        // .offset = 2 * pointer - 1,
        .offset = 2 * (grid_pos / window_size) - 1,
        .scale = music_size / window_size / grid_count,
        .light = all_in ? 0.9f : 0,
    };
}

auto draw(App& app) -> void {
    auto& instance_values = app.draw_info.instance_values;
    auto bands = calc_bands(app);
    instance_values.clear();
    if (bands.light) {
        instance_values.push_back({
            .offset = {bands.offset[0], 0},
            .scale = {bands.scale[0], 1},
            .light = bands.light,
        });
        instance_values.push_back({
            .offset = {0, bands.offset[1]},
            .scale = {1, bands.scale[1]},
            .light = bands.light,
        });
    }
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
    taca::text_draw("Music Box", 8, 0);
}

} // namespace music
