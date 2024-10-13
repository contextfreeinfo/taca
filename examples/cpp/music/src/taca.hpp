#pragma once

#include <string_view>
#include "taca.h"

namespace taca {
extern "C" {

void print(std::string_view text) {
    taca_print({ text.data(), text.size() });
}

}
}
