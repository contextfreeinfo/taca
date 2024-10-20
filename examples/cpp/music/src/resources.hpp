#pragma once

#include <cstdint>
#include <span>

namespace music {

// Inside the namespace on purpose.
#include "musicbox-data.c"
#include "shader.frag.c"
#include "shader.vert.c"

auto musicbox_data = std::span<std::uint8_t>{
    src_musicbox_ogg,
    src_musicbox_ogg_len,
};

auto shader_frag_data = std::span<std::uint8_t>{
    out_shader_frag_spv,
    out_shader_frag_spv_len,
};

auto shader_vert_data = std::span<std::uint8_t>{
    out_shader_vert_spv,
    out_shader_vert_spv_len,
};

} // namespace music
