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

#include <stdint.h>

typedef struct tac_Vec2 {
    int32_t x;
    int32_t y;
} tac_Vec2;

typedef enum tac_WindowEventType {
    tac_WindowEventType_Close = 1,
    tac_WindowEventType_Redraw = 2,
    tac_WindowEventType_Resize = 3,
    tac_WindowEventType_Force32 = 0x7FFFFFFF
} tac_WindowEventType;

typedef struct tac_WindowEvent {
    tac_WindowEventType type;
    union {
        struct {
            tac_Vec2 size;
        } resize;
    };
} tac_WindowEvent;

typedef void (*tac_WindowListenCallback)(tac_WindowEvent event, void* userdata);

tac_EXPORT tac_Vec2 tac_windowGetSize(void);
tac_EXPORT void tac_windowListen(tac_WindowListenCallback callback, void* userdata);
tac_EXPORT void tac_windowSetTitle(const char* title);

#endif // TACANA_H_
