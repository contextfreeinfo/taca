#pragma once

#include <cstdint>
#include <span>
#include <string_view>
#include "taca.h"

namespace taca {
extern "C" {

using Sound = taca_Sound;

void print(std::string_view text) {
    taca_print({ text.data(), text.size() });
}

Sound sound_decode(std::span<std::uint8_t> text) {
    return taca_sound_decode({ text.data(), text.size() });
}

}
}
