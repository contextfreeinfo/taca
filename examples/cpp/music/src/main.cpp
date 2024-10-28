#include "app.hpp"
#include "control.hpp"
#include "draw.hpp"
#include "resources.hpp"
// #include <stdfloat> // apparently not available yet
#include <taca.hpp>

namespace music {

// Init fields to zero.
App app = {};

auto start() -> void {
    taca::title_update("Music Box (Taca Demo)");
    taca::print("Hi from C++!");
    // Sound
    // This is a D6 or maybe D7.
    app.ding = taca::sound_decode(musicbox_data);
    // Pipeline
    auto fragment = taca::shader_new(shader_frag_data);
    auto vertex = taca::shader_new(shader_vert_data);
    auto vertex_buffers = std::to_array<taca::BufferInfo>({
        {},
        {.first_attribute = 1, .step = taca::Step::Instance},
    });
    taca::pipeline_new({
        .depth_test = true,
        .fragment = {.entry = "main", .shader = fragment},
        .vertex = {.entry = "main", .shader = vertex},
        .vertex_buffers = std::span(vertex_buffers),
    });
    // Buffers
    auto rect_indices = std::to_array<std::uint16_t>({0, 1, 2, 1, 3, 2});
    app.draw_info.index_buffer = taca::buffer_new(
        taca::BufferKind::Index,
        std::as_bytes(std::span(rect_indices))
    );
    auto rect_vertices = std::to_array<std::array<float, 2>>(
        {{-1, -1}, {-1, 1}, {1, -1}, {1, 1.0}}
    );
    app.draw_info.vertex_buffer = taca::buffer_new(
        taca::BufferKind::Vertex,
        std::as_bytes(std::span(rect_vertices))
    );
    app.draw_info.instance_buffer = taca::buffer_new(
        taca::BufferKind::Vertex,
        taca::span_sized(
            (max_pitches * max_ticks + max_extra_rects) * sizeof(DrawInstance)
        )
    );
}

// clang-format off
__attribute__((export_name("update")))
// clang-format on
auto update(taca::EventKind event) -> void {
    if (!app.ready) {
        if (event == taca::EventKind::TasksDone) {
            // taca::print("sounds loaded");
            app.ready = true;
        }
        return;
    }
    switch (event) {
        case taca::EventKind::Frame: {
            app.window_state = taca::window_state();
            update_control(app);
            draw(app);
            break;
        }
        case taca::EventKind::Key: {
            auto event = taca::key_event();
            if (event.pressed) {
                update_key(app, event);
            }
            break;
        }
        default:
    }
}

} // namespace music

// Even if I say -Wl,--no-entry, I still get a _start, and the overall size is
// larger, so just use main. Maybe I'm just missing some option.
auto main() -> int {
    music::start();
}
