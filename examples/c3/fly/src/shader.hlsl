struct VSInput {
    float4 position : POSITION;
};

struct PSInput {
    float4 position : SV_POSITION;
};

// Vertex Shader
PSInput vertex_main(VSInput input) {
    PSInput output;
    output.position = input.position;
    return output;
}

// Pixel Shader
float4 fragment_main(PSInput input) : SV_TARGET {
    return float4(0.0, 1.0, 0.0, 1.0); // Outputs green color
}
