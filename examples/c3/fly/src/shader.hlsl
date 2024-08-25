struct VSInput {
    float3 position : POSITION;
};

struct PSInput {
    float4 position : SV_POSITION;
};

PSInput vertex_main(VSInput input) {
    PSInput output;
    output.position = float4(input.position, 1);
    return output;
}

float4 fragment_main(PSInput input) : SV_TARGET {
    return float4(0.0, 1.0, 0.0, 1.0);
}
