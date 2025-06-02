#include <taca.h>

void int_to_str(int val, char* buf, size_t buf_size);

void exports_taca_core_app_update(exports_taca_core_app_event_t* event) {
    taca_string_t name = {0};
    taca_string_set(&name, "app.json");
    taca_string_t value = {0};
    if (!taca_core_archive_get_text(&name, &value)) {
        taca_string_dup(&value, "(no value found)");
    }
    taca_core_console_print(&value);
    taca_string_free(&value);
    // Try storage.
    taca_core_storage_own_store_t store = taca_core_storage_access();
    taca_string_set(&name, "hi.bin");
    taca_core_storage_borrow_store_t store_borrowed =
        taca_core_storage_borrow_store(store);
    // Extra access to see what happens.
    taca_core_storage_own_store_t store0 = taca_core_storage_access();
    taca_core_storage_own_bytes_t bytes =
        taca_core_storage_method_store_get_bytes(store_borrowed, &name);
    char buffer[100];
    taca_string_set(&value, "store then bytes");
    taca_core_console_print(&value);
    int_to_str(store.__handle, buffer, sizeof(buffer));
    taca_string_set(&value, buffer);
    taca_core_console_print(&value);
    int_to_str(bytes.__handle, buffer, sizeof(buffer));
    taca_string_set(&value, buffer);
    taca_core_console_print(&value);
}

void int_to_str(int val, char* buf, size_t buf_size) {
    size_t len = 0;
    int temp = val;
    if (val == 0) {
        if (buf_size > 1) {
            buf[0] = '0';
            buf[1] = '\0';
        }
        return;
    }
    // Count digits
    while (temp) {
        len++;
        temp /= 10;
    }
    if (len >= buf_size) {
        // truncate or just null-terminate
        buf[0] = '\0';
        return;
    }
    buf[len] = '\0';
    while (val) {
        buf[--len] = '0' + (val % 10);
        val /= 10;
    }
}
