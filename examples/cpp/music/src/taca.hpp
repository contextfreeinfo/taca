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

struct KeyEvent {
    bool pressed;
    Key key;
    std::array<std::uint8_t, 4> text;
};

using Sound = taca_Sound;
using SoundPlay = taca_SoundPlay;
using SoundPlayInfo = taca_SoundPlayInfo;
using WindowState = taca_WindowState;
using Vec2 = taca_Vec2;

KeyEvent key_event() {
    auto event = taca_key_event();
    return {.pressed = event.pressed, .key = static_cast<Key>(event.key)};
}

void print(std::string_view text) {
    taca_print({text.data(), text.size()});
}

Sound sound_decode(std::span<std::uint8_t> text) {
    return taca_sound_decode({text.data(), text.size()});
}

SoundPlay sound_play(const SoundPlayInfo& info) {
    return taca_sound_play(&info);
}

void title_update(std::string_view text) {
    taca_title_update({text.data(), text.size()});
}

WindowState window_state() {
    return taca_window_state();
}

} // namespace taca
