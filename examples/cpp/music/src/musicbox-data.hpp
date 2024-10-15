#pragma once

#include <cstdint>
#include <span>

namespace music {

#include "musicbox-data.c"

std::span<std::uint8_t> musicbox_data = {
    src_musicbox_mp3,
    src_musicbox_mp3_len,
};

} // namespace music
