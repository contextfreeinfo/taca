#pragma once

#include <cstdint>
#include <span>

namespace music {

#include "musicbox-mp3.c"

std::span<std::uint8_t> musicbox_mp3 = {src_musicbox_mp3, src_musicbox_mp3_len};

}
