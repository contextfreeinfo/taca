cbuffer Uniforms : register(b0)
{
    matrix view;
    matrix proj;
};

struct VSInput
{
    float3 position : POSITION;
    // TODO Other vertex attributes
};

struct PSInput
{
    float4 position : SV_POSITION;
    // TODO Other varying outputs
};

PSInput vertex_main(VSInput input)
{
    PSInput output;
    float4 pos_world = float4(input.position, 1);
    float4 pos_view = mul(pos_world, view);
    output.position = mul(pos_view, proj);
    return output;
}

float4 fragment_main(PSInput input) : SV_TARGET
{
    return float4(0, 1, 0, 1);
}
