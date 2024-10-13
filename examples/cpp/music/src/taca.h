#pragma once

#include <stddef.h>

typedef struct {
    const char* data;
    size_t size;
} taca_string_view;

#ifdef __cplusplus
extern "C" {
#endif

// __attribute__((__import_module__("taca"), __import_name__("print")))
__attribute__((__import_name__("taca_print")))
void taca_print(taca_string_view text);

#ifdef __cplusplus
}
#endif
