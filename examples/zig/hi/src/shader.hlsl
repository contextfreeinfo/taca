cbuffer Uniforms : register(b0) {
    float2 pointer;
};

struct VertexOutput {
    float4 position : SV_Position; // Corrected to SV_Position
    float4 color : COLOR;
};

VertexOutput vs_main(float2 in_pos : POSITION, float4 in_color : COLOR) {
    VertexOutput output; // Renamed to output to avoid potential keyword issues
    output.position = float4(in_pos + pointer, 0.0, 1.0);
    output.color = in_color;
    return output;
}

float4 fs_main(VertexOutput input) : SV_Target { // Corrected to SV_Target
    float distance = length(pointer - input.position.xy);
    return input.color + 1e-3 * distance;
}
