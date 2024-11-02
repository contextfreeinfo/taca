#pragma once

#include "app.hpp"
#include "markers.hpp"
#include <array>
#include <taca.hpp>

namespace music {

constexpr auto background_light = 0.8f;
constexpr auto band_light = 0.9f;
constexpr auto button_scale_factor = 0.7f;
constexpr auto marker_light = 0.5f;
constexpr auto note_light = 1.2f;
constexpr auto play_band_light = 0.95f;

auto draw_marker(
    std::vector<DrawInstance>& instances,
    const std::span<const DrawInstance>& parts,
    const BandInfo& bands,
    vec::Vec2f offset,
    vec::Vec2f suboffset = vec::Vec2f{0, 0}
) {
    using namespace vec;
    for (auto part : parts) {
        part.offset =
            2 * ((part.offset + suboffset) / 3 + offset) * bands.cell_scale +
            bands.cell_start;
        if (part.scale[0] == 0) {
            part.scale[0] = 1;
        }
        if (part.scale[1] == 0) {
            part.scale[1] = 1;
        }
        part.scale = part.scale * bands.cell_scale / 3 * 0.8;
        part.light = marker_light;
        instances.push_back(part);
    }
}

auto draw(App& app) -> void {
    using namespace vec;
    auto& instances = app.draw_info.instance_values;
    auto bands = calc_bands(app);
    // Back button info in advance because split.
    auto back_instance = DrawInstance{
        .offset = bands.button_back_offset,
        .scale = -button_scale_factor * bands.button_scale,
        .light = note_light,
    };
    // Rectangles.
    instances.clear();
    // Background.
    instances.push_back({
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
    instances.push_back({
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
        instances.push_back({
            .offset = {bands.cell_offset[0], bands.bands_offset[1]},
            .scale = {bands.cell_scale[0], bands.bands_scale[1]},
            .light = band_light,
        });
    }
    if (bands.cell_index[1].has_value()) {
        instances.push_back({
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
    instances.push_back(back_rect);
    // Notes.
    for (std::size_t t = 0; t < app.song.ticks.size(); t += 1) {
        const auto& tick = app.song.ticks[t];
        for (const auto& note : tick.notes) {
            auto offset = Vec2f{
                static_cast<float>(t),
                semitones_default - note.semitones,
            };
            instances.push_back({
                .offset = 2 * offset * bands.cell_scale + bands.cell_start,
                .scale = 0.9 * bands.cell_scale,
                .light = note_light,
            });
        }
    }
    // Pitch markers.
    for (std::size_t i = 0; i < max_pitches; i += 1) {
        auto parts = std::span<const DrawInstance>{};
        auto pitch = (24 - i) % 12;
        switch (pitch) {
            case 0: {
                parts = std::span{markers::note1};
                break;
            }
            case 2: {
                parts = std::span{markers::note2};
                break;
            }
            case 4: {
                parts = std::span{markers::note3};
                break;
            }
            case 5: {
                parts = std::span{markers::note4};
                break;
            }
            case 7: {
                parts = std::span{markers::note5};
                break;
            }
            case 9: {
                parts = std::span{markers::note6};
                break;
            }
            // Gap of only 2, so maybe 7 not needed?
            case 11: {
                parts = std::span{markers::note7};
                break;
            }
            default:
                continue;
        }
        draw_marker(instances, parts, bands, {-1, static_cast<float>(i)});
    }
    for (std::size_t i = 0; i < max_pitches; i += 12) {
        draw_marker(
            instances,
            std::span{markers::note1},
            bands,
            {-1, static_cast<float>(i)}
        );
    }
    for (std::size_t i = 12 - 4; i < max_pitches; i += 12) {
        draw_marker(
            instances,
            std::span{markers::note3},
            bands,
            {-1, static_cast<float>(i)}
        );
    }
    for (std::size_t i = 12 - 7; i < max_pitches; i += 12) {
        draw_marker(
            instances,
            std::span{markers::note5},
            bands,
            {-1, static_cast<float>(i)}
        );
    }
    // Tick markers.
    for (std::size_t i = 0; i < max_ticks; i += 1) {
        auto on3 = i % 3 == 0;
        auto on4 = i % 4 == 0;
        if (on3) {
            draw_marker(
                instances,
                std::span{markers::tick3},
                bands,
                {static_cast<float>(i), -1},
                Vec2f{on4 ? 0.5f : 0, 0}
            );
        }
        if (on4) {
            draw_marker(
                instances,
                std::span{markers::tick4},
                bands,
                {static_cast<float>(i), -1},
                Vec2f{on3 ? -0.5f : 0, 0}
            );
        }
    }
    // Buffers and drawing.
    taca::buffer_update(
        app.draw_info.instance_buffer,
        std::as_bytes(std::span{instances}),
        0
    );
    auto vertex_buffers = std::to_array(
        {app.draw_info.vertex_buffer, app.draw_info.instance_buffer}
    );
    taca::buffers_apply({
        .vertex_buffers = std::span{vertex_buffers},
        .index_buffer = app.draw_info.index_buffer,
    });
    taca::draw(0, 6, instances.size());
    // Triangles.
    instances.clear();
    instances.push_back({
        .offset = bands.button_play_offset,
        .scale = button_scale_factor * bands.button_scale,
        .light = note_light,
    });
    instances.push_back(back_instance);
    taca::buffer_update(
        app.draw_info.instance_tri_buffer,
        std::as_bytes(std::span{instances}),
        0
    );
    auto tri_buffers = std::to_array(
        {app.draw_info.vertex_tri_buffer, app.draw_info.instance_tri_buffer}
    );
    taca::buffers_apply({
        .vertex_buffers = std::span{tri_buffers},
        .index_buffer = app.draw_info.index_buffer,
    });
    taca::draw(0, 3, instances.size());
    // // Text
    // taca::text_align(taca::TextAlignX::Left, taca::TextAlignY::Top);
    // taca::text_draw("Music Box", 50, 0);
}

} // namespace music
