#pragma once

#include "taca.h"
#include <array>
#include <cstdint>
#include <span>
#include <string_view>

namespace taca {

// Enums, redone as enum struct

enum struct BufferKind : std::uint32_t {
    Vertex = taca_BufferKind_Vertex,
    Index = taca_BufferKind_Index,
};

enum struct EventKind : std::uint32_t {
    Frame = taca_EventKind_Frame,
    Key = taca_EventKind_Key,
    TasksDone = taca_EventKind_TasksDone,
    Press = taca_EventKind_Press,
    Release = taca_EventKind_Release,
};

enum struct Key : std::uint32_t {
    None = taca_Key_None,
    ArrowUp = taca_Key_ArrowUp,
    ArrowDown = taca_Key_ArrowDown,
    ArrowLeft = taca_Key_ArrowLeft,
    ArrowRight = taca_Key_ArrowRight,
    Space = taca_Key_Space,
    Escape = taca_Key_Escape,
};

enum struct SoundRateKind : std::uint32_t {
    Semitones = taca_SoundRateKind_Semitones,
    Factor = taca_SoundRateKind_Factor,
};

enum struct SoundVolumeKind : std::uint32_t {
    Decibels = taca_SoundVolumeKind_Decibels,
    Factor = taca_SoundVolumeKind_Factor,
};

enum struct Step : std::uint32_t {
    Vertex = taca_Step_Vertex,
    Instance = taca_Step_Instance,
};

enum struct TextAlignX {
    Left = taca_TextAlignX_Left,
    Center = taca_TextAlignX_Center,
    Right = taca_TextAlignX_Right,
};

enum struct TextAlignY {
    Baseline = taca_TextAlignY_Baseline,
    Top = taca_TextAlignY_Top,
    Middle = taca_TextAlignY_Middle,
    Bottom = taca_TextAlignY_Bottom,
};

// Aliases

using AttributeInfo = taca_AttributeInfo;
using Buffer = taca_Buffer;
using Pipeline = taca_Pipeline;
using Shader = taca_Shader;
using Sound = taca_Sound;
using SoundPlay = taca_SoundPlay;
using WindowState = taca_WindowState;
using Vec2 = taca_Vec2;

using ByteSpan = std::span<const std::byte>;

// Structs with redone insides

struct BufferInfo {
    std::size_t first_attribute;
    Step step;
    std::size_t stride;
};

struct Buffers {
    std::span<Buffer> vertex_buffers;
    Buffer index_buffer;
};

struct KeyEvent {
    bool pressed;
    Key key;
    // TODO Instead a single u32 code point?
    // TODO Do we need to support multiple code points as a string?
    // TODO Require passing in sized buffer for storage?
    std::array<std::byte, 4> text;
};

struct PipelineShaderInfo {
    std::string_view entry;
    Shader shader;
};

struct PipelineInfo {
    bool depth_test;
    PipelineShaderInfo fragment;
    PipelineShaderInfo vertex;
    std::span<AttributeInfo> vertex_attributes;
    std::span<BufferInfo> vertex_buffers;
};

struct SoundPlayInfo {
    Sound sound;
    float delay;
    float rate;
    SoundRateKind rate_kind;
    float volume;
    SoundVolumeKind volume_kind;
};

// Helpers

auto span_sized(std::size_t size) -> std::span<const std::byte> {
    // This is abusive from a C++ perspective. Only use for Taca needs.
    return {static_cast<const std::byte*>(nullptr), size};
}

auto to_taca(ByteSpan bytes) -> taca_ByteSpan {
    // I'm not sure C++ guarantees field order on std::span, so this is for
    // being explicit.
    return {reinterpret_cast<const taca_byte*>(bytes.data()), bytes.size()};
}

auto to_taca(std::string_view text) -> taca_StringView {
    return {text.data(), text.size()};
}

// Main api

auto buffer_new(BufferKind kind, ByteSpan bytes) -> Buffer {
    return taca_buffer_new(static_cast<taca_BufferKind>(kind), to_taca(bytes));
}

auto buffer_update(Buffer buffer, ByteSpan bytes, std::size_t buffer_offset)
    -> void {
    taca_buffer_update(buffer, to_taca(bytes), buffer_offset);
}

auto buffers_apply(Buffers buffers) -> void {
    taca_buffers_apply({
        .vertex_buffers =
            {buffers.vertex_buffers.data(), buffers.vertex_buffers.size()},
        .index_buffer = buffers.index_buffer,
    });
}

auto draw(
    std::uint32_t item_begin,
    std::uint32_t item_count,
    std::uint32_t instance_count
) -> void {
    taca_draw(item_begin, item_count, instance_count);
}

auto key_event() -> KeyEvent {
    auto event = taca_key_event();
    return reinterpret_cast<KeyEvent&>(event);
}

auto pipeline_new(PipelineInfo info) -> Pipeline {
    // All this for fear that std::span field order might be unpromised.
    auto out = taca_PipelineInfo{
        .depth_test = info.depth_test,
        .fragment =
            {
                .entry = to_taca(info.fragment.entry),
                .shader = info.fragment.shader,
            },
        .vertex =
            {
                .entry = to_taca(info.vertex.entry),
                .shader = info.vertex.shader,
            },
        .vertex_attributes =
            {info.vertex_attributes.data(), info.vertex_attributes.size()},
        .vertex_buffers =
            {
                .data = reinterpret_cast<const taca_BufferInfo*>(
                    info.vertex_buffers.data()
                ),
                .size = info.vertex_buffers.size(),
            },
    };
    return taca_pipeline_new(&out);
}

auto print(std::string_view text) -> void {
    taca_print(to_taca(text));
}

auto shader_new(ByteSpan bytes) -> Shader {
    return taca_shader_new(to_taca(bytes));
}

auto sound_decode(ByteSpan bytes) -> Sound {
    return taca_sound_decode(to_taca(bytes));
}

auto sound_play(const SoundPlayInfo& info) -> SoundPlay {
    auto out = reinterpret_cast<const taca_SoundPlayInfo&>(info);
    return taca_sound_play(&out);
}

auto text_align(TextAlignX x, TextAlignY y) -> void {
    taca_text_align(
        static_cast<taca_TextAlignX>(x),
        static_cast<taca_TextAlignY>(y)
    );
}

auto text_draw(std::string_view bytes, float x, float y) -> void {
    taca_text_draw(to_taca(bytes), x, y);
}

auto title_update(std::string_view text) -> void {
    taca_title_update(to_taca(text));
}

auto window_state() -> WindowState {
    return taca_window_state();
}

} // namespace taca
