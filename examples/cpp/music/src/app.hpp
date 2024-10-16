#pragma once

#include <taca.hpp>

namespace music {

struct App {
    taca::Sound ding;
    bool ready;
    bool was_pressed;
    taca::WindowState window_state;
};

} // namespace music
