#pragma once

#include "app.hpp"
#include <array>
#include <taca.hpp>
#include <vec.hpp>

namespace music {

struct BandInfo {
    bool active;
    vec::Vec2f offset_bands;
    vec::Vec2f offset_cell;
    vec::Vec2f scale_bands;
    vec::Vec2f scale_cell;
};

constexpr auto bands_light = 0.9f;

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
    auto grid_pos_frac =
        floor(music_pos_frac * grid_count) / grid_count + 0.5 / grid_count;
    auto grid_pos = grid_pos_frac * music_size + margin;
    auto table_margin = Vec2f{0, margin[1]};
    auto table_size = Vec2f{window_size[0], music_size[1]};
    return {
        .active = active,
        .offset_bands = (2 * table_margin + table_size) / window_size - 1,
        .offset_cell = 2 * (grid_pos / window_size) - 1,
        .scale_bands = table_size / window_size,
        .scale_cell = music_size / grid_count / window_size,
    };
}

auto draw(App& app) -> void {
    auto& instance_values = app.draw_info.instance_values;
    auto bands = calc_bands(app);
    instance_values.clear();
    if (bands.active) {
        instance_values.push_back({
            .offset = {bands.offset_cell[0], bands.offset_bands[1]},
            .scale = {bands.scale_cell[0], bands.scale_bands[1]},
            .light = bands_light,
        });
        instance_values.push_back({
            .offset = {bands.offset_bands[0], bands.offset_cell[1]},
            .scale = {bands.scale_bands[0], bands.scale_cell[1]},
            .light = bands_light,
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
    taca::text_draw("Music Box", 50, 0);
}

} // namespace music
