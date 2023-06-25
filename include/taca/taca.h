#ifndef TACANA_H_
#define TACANA_H_

#if defined(tac_SHARED_LIBRARY)
#    if defined(_WIN32)
#        if defined(tac_IMPLEMENTATION)
#            define tac_EXPORT _declspec(dllexport)
#        else
#            define tac_EXPORT _declspec(dllimport)
#        endif
#    else
#        if defined(tac_IMPLEMENTATION)
#            define tac_EXPORT _attribute_((visibility("default")))
#        else
#            define tac_EXPORT
#        endif
#    endif
#else
#    define tac_EXPORT
#endif

#include <stdbool.h>
#include <stdint.h>

// TODO Replace all with script to generate json.

typedef enum tac_KeyCode {
    tac_KeyCode_Undefined = 0,
    tac_KeyCode_Left = 1,
    tac_KeyCode_Up = 2,
    tac_KeyCode_Right = 3,
    tac_KeyCode_Down = 4,
    tac_KeyCode_Force32 = 0x7FFFFFFF
} tac_KeyCode;

typedef struct tac_Vec2 {
    int32_t x;
    int32_t y;
} tac_Vec2;

typedef enum tac_WindowEventType {
    tac_WindowEventType_Close = 1,
    tac_WindowEventType_Key = 2,
    tac_WindowEventType_Redraw = 3,
    tac_WindowEventType_Resize = 4,
    tac_WindowEventType_Force32 = 0x7FFFFFFF
} tac_WindowEventType;

typedef struct tac_KeyEvent {
    tac_KeyCode code;
    bool pressed;
} tac_KeyEvent;

// For now, call other functions to get details.
typedef void (*tac_WindowListenCallback)(tac_WindowEventType type, void* userdata);

tac_EXPORT tac_KeyEvent tac_keyEvent(void);
tac_EXPORT tac_Vec2 tac_windowInnerSize(void);
// TODO Use exported function and just register userdata here?
tac_EXPORT void tac_windowListen(tac_WindowListenCallback callback, void* userdata);
tac_EXPORT void tac_windowSetTitle(const char* title);

#endif // TACANA_H_
