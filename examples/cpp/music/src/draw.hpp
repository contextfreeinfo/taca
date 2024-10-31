#pragma once

#include "app.hpp"
#include "markers.hpp"
#include <array>
#include <taca.hpp>

namespace music {

constexpr auto background_light = 0.8f;
constexpr auto band_light = 0.9f;
constexpr auto button_scale_factor = 0.7f;
constexpr auto marker_light = 0.4f;
constexpr auto note_light = 1.2f;
constexpr auto play_band_light = 0.95f;

auto draw(App& app) -> void {
    using namespace vec;
    auto& instance_values = app.draw_info.instance_values;
    auto bands = calc_bands(app);
    // Back button info in advance because split.
    auto button_scale = Vec2f{
        bands.margin[1] * bands.window_size[1] / bands.window_size[0],
        bands.margin[1],
    };
    auto back_instance = DrawInstance{
        .offset = Vec2f{-1, -1} + button_scale * Vec2f{4, 1},
        .scale = -button_scale_factor * button_scale,
        .light = note_light,
    };
    // Rectangles.
    instance_values.clear();
    // Background.
    instance_values.push_back({
        .scale = {1, 1},
        .light = background_light,
    });
    // Play band.
    const auto& play_info = app.play_info;
    auto tick = static_cast<float>(play_info.tick);
    auto subtick =
        static_cast<float>(play_info.frames_until_tick) / frames_per_tick;
    if (play_info.tick) {
        // Past the first, so back up.
        tick -= subtick;
    } else if (subtick) {
        // Past the end before wrapping back.
        tick = max_ticks - subtick;
    }
    instance_values.push_back({
        .offset =
            {
                2 * tick * bands.cell_scale[0] + bands.cell_start[0],
                bands.bands_offset[1],
            },
        .scale = {bands.cell_scale[0], bands.bands_scale[1]},
        .light = play_band_light,
    });
    // Highlight bands.
    if (bands.cell_index[0].has_value()) {
        instance_values.push_back({
            .offset = {bands.cell_offset[0], bands.bands_offset[1]},
            .scale = {bands.cell_scale[0], bands.bands_scale[1]},
            .light = band_light,
        });
    }
    if (bands.cell_index[1].has_value()) {
        instance_values.push_back({
            .offset = {bands.bands_offset[0], bands.cell_offset[1]},
            .scale = {bands.bands_scale[0], bands.cell_scale[1]},
            .light = band_light,
        });
    }
    // Back button rectangle. Yes, this split is hacky.
    auto back_rect = DrawInstance{
        .offset = back_instance.offset +
            back_instance.scale * button_scale_factor * Vec2f{1, 0},
        .scale = back_instance.scale / Vec2f{3, 1},
        .light = back_instance.light,
    };
    instance_values.push_back(back_rect);
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
                .light = note_light,
            });
        }
    }
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
    // Triangles.
    instance_values.clear();
    instance_values.push_back({
        .offset = Vec2f{-1, -1} + button_scale * Vec2f{1.5, 1},
        .scale = button_scale_factor * button_scale,
        .light = note_light,
    });
    instance_values.push_back(back_instance);
    taca::buffer_update(
        app.draw_info.instance_tri_buffer,
        std::as_bytes(std::span{instance_values}),
        0
    );
    auto tri_buffers = std::to_array(
        {app.draw_info.vertex_tri_buffer, app.draw_info.instance_tri_buffer}
    );
    taca::buffers_apply({
        .vertex_buffers = std::span{tri_buffers},
        .index_buffer = app.draw_info.index_buffer,
    });
    taca::draw(0, 3, instance_values.size());
    // // Text
    // taca::text_align(taca::TextAlignX::Left, taca::TextAlignY::Top);
    // taca::text_draw("Music Box", 50, 0);
}

} // namespace music
