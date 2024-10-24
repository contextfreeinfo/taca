#pragma once

#include "app.hpp"
#include <taca.hpp>

namespace music {

auto draw(App& app) -> void {
    auto& instance_values = app.draw_info.instance_values;
    instance_values.clear();
    instance_values.push_back({
        .offset = {0, 0},
        .scale = {0.05, 1},
        .light = 1,
    });
    instance_values.push_back({
        .offset = {0, 0},
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
}

} // namespace music
