#include "app.hpp"
#include "control.hpp"
#include "draw.hpp"
#include "resources.hpp"
#include "songs.hpp"
// #include <stdfloat> // apparently not available yet
#include <taca.hpp>

namespace music {

// Global state, but don't init here in case of C++ post-main weirdness.
App app;

// clang-format off
__attribute__((export_name("start")))
// clang-format on
auto start() -> void {
    // Initialize here.
    app = {};
    taca::title_update("Music Box (Taca Demo)");
    taca::print("Hi from C++!");
    // Sound
    // This is a D6 or maybe D7.
    app.ding = taca::sound_decode(musicbox_data);
    app.song = songs::basic();
    // song_print(app.song);
    // Pipeline
    auto fragment = taca::shader_new(shader_frag_data);
    auto vertex = taca::shader_new(shader_vert_data);
    auto vertex_buffers = std::to_array<taca::BufferInfo>({
        {},
        {.first_attribute = 1, .step = taca::Step::Instance},
    });
    taca::pipeline_new({
        .fragment = {.entry = "main", .shader = fragment},
        .vertex = {.entry = "main", .shader = vertex},
        .vertex_buffers = std::span(vertex_buffers),
    });
    // Buffers
    // Rectangle for most things.
    // By chance, rectangle start indices also work for tri indices.
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
    // Triangle for play & rewind.
    auto tri_vertices =
        std::to_array<std::array<float, 2>>({{-1, -1}, {1, 0}, {-1, 1}});
    app.draw_info.vertex_tri_buffer = taca::buffer_new(
        taca::BufferKind::Vertex,
        std::as_bytes(std::span(tri_vertices))
    );
    app.draw_info.instance_tri_buffer = taca::buffer_new(
        taca::BufferKind::Vertex,
        taca::span_sized(max_tris * sizeof(DrawInstance))
    );
    // song_print(app.song);
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

// So far, I fail to work around needing this with flags, but some cleanup
// happens after main, so best not to init anything inside it.
// I've tried -Wl,--no-entry such as in wasm4:
// https://github.com/aduros/wasm4/blob/979be845216ee9d613cb6555fb8b11c01bec39a0/cli/assets/templates/c/Makefile#L24
// But I get crashes running the wasm. I haven't worked out details.
auto main() -> int {}
