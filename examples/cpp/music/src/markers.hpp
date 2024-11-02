#pragma once

#include "app.hpp"
#include <array>

namespace music::markers {

constexpr auto note1 = std::to_array<DrawInstance>({
    {},
});

constexpr auto note2 = std::to_array<DrawInstance>({
    {
        .offset = {-0.5, 0},
    },
    {
        .offset = {0.5, 0},
    },
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

constexpr auto note4 = std::to_array<DrawInstance>({
    {
        .offset = {-0.5, -0.5},
    },
    {
        .offset = {0.5, -0.5},
    },
    {
        .offset = {-0.5, 0.5},
    },
    {
        .offset = {0.5, 0.5},
    },
});

constexpr auto note5 = std::to_array<DrawInstance>({
    {
        .offset = {-1, -0.5},
    },
    {
        .offset = {0, -0.5},
    },
    {
        .offset = {-1, 0.5},
    },
    {
        .offset = {0, 0.5},
    },
    {
        .offset = {1, 0},
    },
});

constexpr auto note6 = std::to_array<DrawInstance>({
    {
        .offset = {-1, -0.5},
    },
    {
        .offset = {0, -0.5},
    },
    {
        .offset = {1, -0.5},
    },
    {
        .offset = {-1, 0.5},
    },
    {
        .offset = {0, 0.5},
    },
    {
        .offset = {1, 0.5},
    },
});

constexpr auto note7 = std::to_array<DrawInstance>({
    {
        .offset = {-1, -1},
    },
    {
        .offset = {-1, 0},
    },
    {
        .offset = {-1, 1},
    },
    {
        .offset = {0, -1},
    },
    {
        .offset = {0, 0},
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

} // namespace music::markers
