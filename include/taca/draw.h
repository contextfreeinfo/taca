#pragma once

// TODO We need to enforce layouts for all these structs.

typedef struct taca_Vec4 {
    float x, y, z, w;
} taca_Vec4;

typedef struct taca_Rgba {
    float r, g, b, a;
} taca_Rgba;

typedef const void* taca_Attribute;

// See:
// https://www.w3.org/TR/WGSL/#builtin-inputs-outputs

typedef struct taca_ShaderInput {
    void* uniforms;
    const taca_Attribute* attributes;
} taca_ShaderInput;

typedef struct taca_VertexOutput {
    taca_Vec4 position;
    size_t clipDistancesCount; // Must be compile-time constant.
    float clipDistances[8];
    // Additional opaque data might follow.
} taca_VertexOutput;

typedef void (*taca_FragmentShader)(
    const taca_ShaderInput* input,
    taca_Rgba* output
);

typedef void (*taca_VertexShader)(
    const taca_ShaderInput* input,
    taca_VertexOutput* output
);

typedef struct taca_AttributeLayout {
    taca_Attribute start;
    size_t stride;
} taca_AttributeLayout;

typedef struct taca_PipelineData {
    const void* uniforms;
    const taca_AttributeLayout* attributes;
    size_t attributeCount;
    size_t vertexCount;
    size_t vertexOutSize;
} taca_PipelineData;

typedef struct taca_Pipeline {
    // TODO Error handling here also?
    taca_FragmentShader fragment;
    taca_VertexShader vertex;
} taca_Pipeline;

taca_EXPORT void taca_draw(
    const taca_Pipeline* pipeline,
    const taca_PipelineData* data
);
