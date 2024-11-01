#pragma once

#include "app.hpp"

namespace music::songs {

auto basic() -> Song {
    return {
        .ticks =
            {
                {.notes = {{.semitones = 10}}},
                {.notes = {{.semitones = 9}}},
                {},
                {.notes = {{.semitones = 4}}},
                {},
                {},
                {},
                {.notes = {{.semitones = 8}, {.semitones = -4}}},
                {},
                {},
                {.notes = {{.semitones = 1}}},
                {},
                {},
                {},
                {.notes = {{.semitones = 4}}},
                {.notes = {{.semitones = 3}}},
                {},
                {},
                {},
                {},
                {},
                {.notes = {{.semitones = -4}}},
                {.notes = {{.semitones = -13}}},
                {},
                {},
                {},
                {},
                {},
                {},
                {.notes = {{.semitones = -11}}},
            },
    };
}

} // namespace music::songs
