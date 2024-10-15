#pragma once

#include <stddef.h>
#include <stdint.h>

typedef struct {
    const uint8_t* data;
    size_t size;
} taca_BytesView;

typedef struct {
    const char* data;
    size_t size;
} taca_StringView;

// Note that size in C requires C23.
typedef enum : uint32_t {
    taca_EventKind_Frame,
    taca_EventKind_Key,
    taca_EventKind_TasksDone,
} taca_EventKind;

#ifdef __cplusplus
extern "C" {
#endif

typedef size_t taca_Sound;

// clang-format off

__attribute__((import_name("taca_sound_decode")))
taca_Sound taca_sound_decode(taca_BytesView bytes);

// __attribute__((import_module("taca"), import_name("print")))
__attribute__((import_name("taca_print")))
void taca_print(taca_StringView text);

// clang-format on

#ifdef __cplusplus
}
#endif
