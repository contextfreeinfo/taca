#pragma once

#include <stddef.h>
#include <stdint.h>

typedef struct {
    const uint8_t* data;
    size_t size;
} taca_bytes_view;

typedef struct {
    const char* data;
    size_t size;
} taca_string_view;

#ifdef __cplusplus
extern "C" {
#endif

typedef size_t taca_Sound;

__attribute__((import_name("taca_sound_decode")))
taca_Sound taca_sound_decode(taca_bytes_view bytes);

// __attribute__((import_module("taca"), import_name("print")))
__attribute__((import_name("taca_print")))
void taca_print(taca_string_view text);

#ifdef __cplusplus
}
#endif
