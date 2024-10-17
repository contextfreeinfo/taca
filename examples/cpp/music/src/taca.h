#pragma once

#include <stddef.h>
#include <stdint.h>

// Enums

// Note that size in C requires C23.
typedef enum /* : uint32_t */ {
    taca_EventKind_Frame,
    taca_EventKind_Key,
    taca_EventKind_TasksDone,
} taca_EventKind;

typedef enum {
    taca_Key_None,
    taca_Key_ArrowUp,
    taca_Key_ArrowDown,
    taca_Key_ArrowLeft,
    taca_Key_ArrowRight,
    taca_Key_Space,
    taca_Key_Escape,
} taca_Key;

// Handles

typedef size_t taca_Sound;
typedef size_t taca_SoundPlay;

// Supports

typedef struct {
    const uint8_t* data;
    size_t size;
} taca_BytesView;

typedef struct {
    const char* data;
    size_t size;
} taca_StringView;

typedef struct {
    float x;
    float y;
} taca_Vec2;

// Primaries

typedef struct {
    taca_Key key;
    bool pressed;
} taca_KeyEvent;

typedef struct {
    // TODO Playback rate or time or other things.
    taca_Sound sound;
} taca_SoundPlayInfo;

typedef struct {
    taca_Vec2 pointer;
    uint32_t press;
    taca_Vec2 size;
} taca_WindowState;

// Functions

#ifdef __cplusplus
extern "C" {
#endif

// clang-format off

__attribute__((import_name("taca_key_event")))
taca_KeyEvent taca_key_event(void);

// __attribute__((import_module("taca"), import_name("print")))
__attribute__((import_name("taca_print")))
void taca_print(taca_StringView text);

__attribute__((import_name("taca_sound_decode")))
taca_Sound taca_sound_decode(taca_BytesView bytes);

__attribute__((import_name("taca_sound_play")))
// TODO Without explicit pointer, and if only one field, this gets passed as the field value.
taca_SoundPlay taca_sound_play(const taca_SoundPlayInfo* info);

__attribute__((import_name("taca_title_update")))
void taca_title_update(taca_StringView text);

__attribute__((import_name("taca_window_state")))
taca_WindowState taca_window_state(void);

// clang-format on

#ifdef __cplusplus
}
#endif
