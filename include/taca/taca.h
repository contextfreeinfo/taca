#pragma once

#if defined(taca_SHARED_LIBRARY)
#    if defined(_WIN32)
#        if defined(taca_IMPLEMENTATION)
#            define taca_EXPORT _declspec(dllexport)
#        else
#            define taca_EXPORT _declspec(dllimport)
#        endif
#    else
#        if defined(taca_IMPLEMENTATION)
#            define taca_EXPORT _attribute_((visibility("default")))
#        else
#            define taca_EXPORT
#        endif
#    endif
#else
#    define taca_EXPORT
#endif

#include <stdbool.h>
#include <stdint.h>

#include "gpu.h"

// TODO Replace all with script to generate json.

typedef enum taca_KeyCode {
    taca_KeyCode_Undefined = 0,
    taca_KeyCode_Left = 1,
    taca_KeyCode_Up = 2,
    taca_KeyCode_Right = 3,
    taca_KeyCode_Down = 4,
    taca_KeyCode_Force32 = 0x7FFFFFFF
} taca_KeyCode;

typedef struct taca_Vec2 {
    int32_t x;
    int32_t y;
} taca_Vec2;

typedef enum taca_WindowEventType {
    taca_WindowEventType_Close = 1,
    taca_WindowEventType_Key = 2,
    taca_WindowEventType_Redraw = 3,
    taca_WindowEventType_Resize = 4,
    taca_WindowEventType_Force32 = 0x7FFFFFFF
} taca_WindowEventType;

typedef struct taca_KeyEvent {
    taca_KeyCode code;
    bool pressed;
} taca_KeyEvent;

// For now, call other functions to get details.
typedef void (*taca_WindowListenCallback)(taca_WindowEventType type, void* userdata);

// TODO Some init call giving a buffer to work with?

taca_EXPORT taca_KeyEvent taca_keyEvent(void);
taca_EXPORT taca_Vec2 taca_windowInnerSize(void);
// TODO Use exported function and just register userdata here!!
taca_EXPORT void taca_windowListen(taca_WindowListenCallback callback, void* userdata);
taca_EXPORT void taca_windowSetTitle(const char* title);
