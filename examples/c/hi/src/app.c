#include <taca.h>

void exports_taca_core_app_update(exports_taca_core_app_event_t* event) {
    taca_string_t message;
    taca_string_set(&message, "Hi there!");
    taca_core_console_print(&message);
}
