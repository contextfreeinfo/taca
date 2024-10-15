#include "app.hpp"
#include "musicbox-data.hpp"
#include <taca.hpp>

namespace music {

App app = {};

void start() {
    taca::print("Hi from C++!");
    app = {
        .ding = taca::sound_decode(musicbox_data),
    };
}

// clang-format off
__attribute__((export_name("update")))
// clang-format on
void update(taca::EventKind event) {
    switch (event) {
        case taca::EventKind::Frame: {
            break;
        }
        case taca::EventKind::Key: {
            break;
        }
        case taca::EventKind::TasksDone: {
            taca::print("sounds loaded");
            break;
        }
    }
}

} // namespace music

// Even if I say -Wl,--no-entry, I still get a _start, and the overall size is
// larger, so just use main. Maybe I'm just missing some option.
int main() {
    music::start();
}
