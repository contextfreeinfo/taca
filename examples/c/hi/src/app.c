#include <taca.h>

void exports_taca_core_app_update(exports_taca_core_app_event_t* event) {
    taca_string_t name = {0};
    taca_string_set(&name, "app.json");
    taca_string_t value = {0};
    if (!taca_core_archive_get_text(&name, &value)) {
        taca_string_dup(&value, "(no value found)");
    }
    taca_core_console_print(&value);
    taca_string_free(&value);
}
