// Originally based on https://www.shadertoy.com/view/flBGD3
// But drastically changed.

#version 450

precision mediump float;

// tint of metal
#define COL vec3(1, 0.85, 0.8)
// intensity of smudges
#define SMUDGES 0.06

layout (location = 0) out vec4 fragColor;

vec3 gammaRamp(vec3 c) {
    c.x = pow(c.x, 2.2);
    c.y = pow(c.y, 2.2);
    c.z = pow(c.z, 2.2);
    return c;
}

float hash(vec2 p) {
    return fract(sin(mod(dot(p, vec2(127.1, 311.7)), 3.14)) * 43758.5453123);
}

float noise(vec2 p) {
    // p += 10;
    vec2 i = floor(p);
    vec2 f = fract(p);
    float a = hash(i);
    float b = hash(i + vec2(1, 0));
    float c = hash(i + vec2(0, 1));
    float d = hash(i + vec2(1, 1));
    vec2 u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

float mag(vec3 v) {
    return sqrt((v * v).x + (v * v).y + (v * v).z);
}

void main() {
    vec2 p = gl_FragCoord.xy;
    float smudges = 0.0;
    smudges = 1.2 * noise(p + smudges);
    smudges += 0.8 * noise(vec2(p.x * 0.1, p.y) * 1e-2 + smudges);
    smudges *= SMUDGES;
    vec3 col = COL;
    col = gammaRamp(vec3(mix(col, vec3(1.0 - mag(col)), smudges)));
    col *= 0.8;
    fragColor = vec4(col, 1.0);
}
