#pragma once

#include <cstdint>
#include <span>

namespace music {

#include "musicbox-data.c"

std::span<std::uint8_t> musicbox_data = {
    src_musicbox_ogg,
    src_musicbox_ogg_len,
};

} // namespace music
