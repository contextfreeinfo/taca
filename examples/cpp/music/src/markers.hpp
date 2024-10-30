#pragma once

#include "app.hpp"
#include <array>

namespace music::marker {

constexpr auto note1 = std::to_array<DrawInstance>({
    {},
});

constexpr auto note3 = std::to_array<DrawInstance>({
    {
        .offset = {-1, 0},
    },
    {},
    {
        .offset = {1, 0},
    },
});

constexpr auto note5 = std::to_array<DrawInstance>({
    {
        .offset = {-1, -1},
    },
    {
        .offset = {0, -1},
    },
    {
        .offset = {-1, 1},
    },
    {
        .offset = {0, 1},
    },
    {
        .offset = {1, 0},
    },
});

constexpr auto tick3 = std::to_array<DrawInstance>({
    {
        .offset = {0, -1},
    },
    {},
    {
        .offset = {0, 1},
    },
});

constexpr auto tick4 = std::to_array<DrawInstance>({
    {
        .scale = {1, 3},
    },
});

} // namespace music::marker
