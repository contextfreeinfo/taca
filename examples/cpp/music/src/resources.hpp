#pragma once

#include <cstdint>
#include <span>

namespace music {

// Inside the namespace on purpose.
#include "musicbox-data.c"
#include "shader-frag.c"

std::span<std::uint8_t> musicbox_data = {
    src_musicbox_ogg,
    src_musicbox_ogg_len,
};

std::span<std::uint8_t> shader_frag_data = {
    out_shader_frag_opt_spv,
    out_shader_frag_opt_spv_len,
};

} // namespace music
