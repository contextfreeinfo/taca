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

// TODO Define in some IDL instead of here.

// TODO Some taca/math.h for vec & matrix ops.

typedef struct taca_Vec4 {
    float x, y, z, w;
} taca_Vec4;

typedef struct taca_Rgba {
    float r, g, b, a;
} taca_Rgba;

typedef void* taca_Attribute;

typedef struct taca_AttributeLayout {
    // We infer the size of each attribute when going to GPU.
    taca_Attribute* items;
    size_t count;
} taca_AttributeLayout;

typedef struct taca_ShaderInput {
    void* uniforms;
    taca_AttributeLayout* attributes;
} taca_Pipeline;

typedef void (*taca_FragmentShader)(
    taca_ShaderInput* input,
    taca_Rgba* output
);

typedef void (*taca_VertexShader)(
    taca_ShaderInput* input,
    void* output
);

typedef struct taca_PipelineData {
    void* uniforms;
    taca_AttributeLayout attributes;
    size_t vertexCount;
} taca_Pipeline;

typedef struct taca_Pipeline {
    // TODO Error handling here also?
    taca_FragmentShader fragment;
    taca_VertexShader vertex;
} taca_Pipeline;

taca_EXPORT void taca_draw(
    taca_Pipeline* pipeline,
    taca_PipelineData* data
);
