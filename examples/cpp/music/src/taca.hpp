#pragma once
// export module something;

#include <string_view>

namespace taca {
extern "C" {

__attribute__((import_name("taca_print")))
void print(std::string_view text);

}
}
