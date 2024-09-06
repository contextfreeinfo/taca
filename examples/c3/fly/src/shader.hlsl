cbuffer Uniforms : register(b0) {
    float4x4 view;
    float4x4 proj;
    float3 color;
    float3 light;
    float lit;
};

struct VSInput {
    float3 pos : POSITION;
    float3 norm : NORMAL;
    float3 offset : TEXCOORD0;
};

struct PSInput {
    float4 pos : SV_POSITION;
    float bright : TEXCOORD0;
};

#define PI 3.14159265359

PSInput vertex_main(VSInput input) {
    PSInput output;
    // Always rotate original +y toward the center.
    float angle = atan2(input.offset.y, input.offset.x) + PI / 2;
    float c = cos(angle);
    float s = sin(angle);
    float3x3 rot = float3x3(
        c, -s, 0,
        s, c, 0,
        0, 0, 1
    );
    float3 pos = mul(rot, input.pos) + input.offset;
    float4 pos_world = float4(pos, 1);
    float4 pos_proj = mul(proj, pos_world);
    output.pos = mul(view, pos_proj);
    // Also rotate norm.
    float3 norm = mul(rot, input.norm);
    float bright = dot(norm, normalize(light));
    output.bright = (1 - lit) * bright + lit;
    return output;
}

float4 fragment_main(PSInput input) : SV_TARGET {
    float3 shaded = (color * input.bright) * 0.8 + 0.2;
    return float4(shaded, 1);
}
