struct VSInput {
    float4 position : POSITION;
};

struct PSInput {
    float4 position : SV_POSITION;
};

PSInput vertex_main(VSInput input) {
    PSInput output;
    output.position = input.position;
    return output;
}

float4 fragment_main(PSInput input) : SV_TARGET {
    return float4(0.0, 1.0, 0.0, 1.0);
}
