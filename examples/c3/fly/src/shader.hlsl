cbuffer Uniforms : register(b0) {
    float4x4 view;
    float4x4 proj;
};

struct VSInput {
    float3 pos : POSITION;
    float3 norm : NORMAL;
    // TODO Other vertex attributes
};

struct PSInput {
    float4 pos : SV_POSITION;
    float norm : TEXCOORD0;
    // TODO Other varying outputs
};

PSInput vertex_main(VSInput input) {
    PSInput output;
    float4 pos_world = float4(input.pos, 1);
    float4 pos_proj = mul(proj, pos_world);
    output.pos = mul(view, pos_proj);
    output.norm = dot(input.norm, normalize(float3(1, 1, 0)));
    output.norm = 0.5 * output.norm + 0.5;
    return output;
}

float4 fragment_main(PSInput input) : SV_TARGET {
    float3 color = (float3(0, 1, 0) * input.norm) * 0.5 + 0.25;
    return float4(color, 1);
}
