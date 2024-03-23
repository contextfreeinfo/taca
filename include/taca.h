#pragma once

#include <stddef.h>

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

// TODO Define in some IDL instead of here.

// TODO Some taca/math.h for vec & matrix ops.

#include "taca/draw.h"
#include "taca/window.h"
