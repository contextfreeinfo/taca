#pragma once

#include "taca.h"
#include <cstdint>
#include <span>
#include <string_view>

namespace taca {

enum struct EventKind : std::uint32_t {
    Frame = taca_EventKind_Frame,
    Key = taca_EventKind_Key,
    TasksDone = taca_EventKind_TasksDone,
};

using Sound = taca_Sound;

void print(std::string_view text) {
    taca_print({text.data(), text.size()});
}

Sound sound_decode(std::span<std::uint8_t> text) {
    return taca_sound_decode({text.data(), text.size()});
}

} // namespace taca
