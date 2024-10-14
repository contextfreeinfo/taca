#include "musicbox-mp3.hpp"
#include "taca.hpp"

namespace music {

void start() {
    taca::print("Hi from C++!");
    taca::sound_decode(musicbox_mp3);
}

__attribute__((export_name("update")))
void update() {
    //
}

}

// Even if I say -Wl,--no-entry, I still get a _start, and the overall size is
// larger, so just use main. Maybe I'm just missing some option.
int main() {
    music::start();
}
