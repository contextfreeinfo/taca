// Originally based on https://www.shadertoy.com/view/flBGD3
// But drastically changed.

#version 450

precision mediump float;

// tint of metal
#define COL vec3(1, 0.85, 0.7)
// intensity of smudges
#define SMUDGES 0.06

layout (location = 0) in float fragLight;

layout (location = 0) out vec4 fragColor;

vec3 gammaRamp(vec3 c) {
    return pow(c, vec3(2.2));
}

// See also:
// https://thebookofshaders.com/11/

float hash(vec2 p) {
    return fract(sin(mod(dot(p, vec2(127.1, 311.7)), 3.14)) * 43758.5453123);
}

float noise(vec2 p) {
    vec2 whole = floor(p);
    float a = hash(whole);
    float b = hash(whole + vec2(1, 0));
    float c = hash(whole + vec2(0, 1));
    float d = hash(whole + vec2(1, 1));
    vec2 u = smoothstep(0, 1, fract(p));
    return mix(a, b, u.x) + (c - a) * u.y * (1 - u.x) + (d - b) * u.x * u.y;
}

void main() {
    vec2 p = gl_FragCoord.xy;
    float smudges = 0.0;
    smudges = 1.2 * noise(p + smudges);
    smudges += 0.8 * noise(vec2(p.x * 0.1, p.y) * 1e-2 + smudges);
    smudges *= SMUDGES;
    vec3 col = COL;
    col = vec3(mix(col, vec3(1.0 - length(col)), smudges));
    col = gammaRamp(col);
    // Pops more after gamma.
    col *= fragLight;
    fragColor = vec4(col, 1.0);
}
