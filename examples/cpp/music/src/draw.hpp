#pragma once

#include "app.hpp"
#include <array>
#include <taca.hpp>

namespace music {

constexpr auto bands_light = 0.9f;

auto draw(App& app) -> void {
    using namespace vec;
    auto& instance_values = app.draw_info.instance_values;
    auto bands = calc_bands(app);
    instance_values.clear();
    // Notes.
    for (std::size_t t = 0; t < app.song.ticks.size(); t += 1) {
        const auto& tick = app.song.ticks[t];
        for (const auto& note : tick.notes) {
            auto offset = Vec2f{
                static_cast<float>(t),
                semitones_default - note.semitones,
            };
            instance_values.push_back({
                .offset = 2 * offset * bands.cell_scale + bands.cell_start,
                .scale = 0.9 * bands.cell_scale,
                .light = 1.2,
            });
        }
    }
    // Highlight bands.
    if (bands.cell_index[0].has_value()) {
        instance_values.push_back({
            .offset = {bands.cell_offset[0], bands.bands_offset[1]},
            .scale = {bands.cell_scale[0], bands.bands_scale[1]},
            .light = bands_light,
        });
    }
    if (bands.cell_index[1].has_value()) {
        instance_values.push_back({
            .offset = {bands.bands_offset[0], bands.cell_offset[1]},
            .scale = {bands.bands_scale[0], bands.cell_scale[1]},
            .light = bands_light,
        });
    }
    // Background.
    instance_values.push_back({
        .scale = {1, 1},
        .light = 0.8,
    });
    // Buffers and drawing.
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
