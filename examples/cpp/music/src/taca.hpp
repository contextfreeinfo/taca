#pragma once

#include "taca.h"
#include <array>
#include <cstdint>
#include <span>
#include <string_view>

namespace taca {

enum struct EventKind : std::uint32_t {
    Frame = taca_EventKind_Frame,
    Key = taca_EventKind_Key,
    TasksDone = taca_EventKind_TasksDone,
};

enum struct Key : std::uint32_t {
    None,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Space,
    Escape,
};

enum struct SoundRateKind : std::uint32_t {
    Semitones,
    Factor,
};

enum struct Step : std::uint32_t {
    Vertex,
    Instance,
};

using AttributeInfo = taca_AttributeInfo;
using Pipeline = taca_Pipeline;
using Shader = taca_Shader;
using Sound = taca_Sound;
using SoundPlay = taca_SoundPlay;
using WindowState = taca_WindowState;
using Vec2 = taca_Vec2;

struct BufferInfo {
    std::size_t first_attribute;
    Step step;
    std::size_t stride;
};

struct KeyEvent {
    bool pressed;
    Key key;
    std::array<std::uint8_t, 4> text;
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
    float rate;
    SoundRateKind rate_kind;
};

auto key_event() -> KeyEvent {
    auto event = taca_key_event();
    return reinterpret_cast<KeyEvent&>(event);
}

auto pipeline_new(PipelineInfo info) -> Pipeline {
    auto out = reinterpret_cast<const taca_PipelineInfo&>(info);
    return taca_pipeline_new(&out);
}

auto print(std::string_view text) -> void {
    taca_print({text.data(), text.size()});
}

auto shader_new(std::span<std::uint8_t> bytes) -> Shader {
    return taca_shader_new({bytes.data(), bytes.size()});
}

auto sound_decode(std::span<std::uint8_t> bytes) -> Sound {
    return taca_sound_decode({bytes.data(), bytes.size()});
}

auto sound_play(const SoundPlayInfo& info) -> SoundPlay {
    auto out = reinterpret_cast<const taca_SoundPlayInfo&>(info);
    return taca_sound_play(&out);
}

auto title_update(std::string_view text) -> void {
    taca_title_update({text.data(), text.size()});
}

auto window_state() -> WindowState {
    return taca_window_state();
}

} // namespace taca
