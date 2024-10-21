// Modified from https://www.shadertoy.com/view/flBGD3

// Per: https://www.shadertoy.com/terms
// All the shaders you create in Shadertoy are owned by you. You decide which license applies to every shader you create. We recommend you paste your preferred license on top of your code, if you don't place a license on a shader, it will be protected by our default license:

// Creative Commons License
// This work is licensed under a Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License. 

#version 450

precision mediump float;

// tint of metal
#define COL vec3(1, 0.85, 0.8)
// intensity of smudges
#define SMUDGES 0.04
// intensity of the brush
#define BRUSH 0.1

// layout(location = 0) in vec2 vTexCoord;
layout (location = 0) out vec4 fragColor;

vec3 gammaRamp(vec3 c) {
    c.x = pow(c.x, 2.2);
    c.y = pow(c.y, 2.2);
    c.z = pow(c.z, 2.2);
    return c;
}

// copy from https://www.shadertoy.com/view/4l2GzW
float r(float n) {
    return fract(cos(n * 89.42) * 343.42);
}

vec2 r(vec2 n) {
    return vec2(r(n.x * 23.62 - 300.0 + n.y * 34.35), r(n.x * 45.13 + 256.0 + n.y * 38.89));
}

float worley(vec2 n, float s) {
    vec2 floor_ns = floor(n / s);
    vec2 fract_ns = fract(n / s);
    float dis = 2.0;
    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            vec2 p = floor_ns + vec2(x, y);
            float d = length(r(p) + vec2(x, y) - fract_ns);
            if (dis > d) {
                dis = d;
            }
        }
    }
    return 1.0 - dis;
}

// copy from https://www.shadertoy.com/view/4sc3z2

#define MOD3 vec3(.1031, .11369, .13787)

float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
}

float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    float a = hash(i);
    float b = hash(i + vec2(1, 0));
    float c = hash(i + vec2(0, 1));
    float d = hash(i + vec2(1, 1));
    vec2 u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

float mixNoise(vec2 uv) {
    float dis = uv.y + noise(vec2(uv.x * 10.0, 0)) * 2.0;
    dis = (1.0 + noise(vec2(uv.x, dis) * 8.0));
    dis *= (1.0 + (worley(uv.xy, 32.0) +
        0.5 * worley(2.0 * uv.xy, 32.0)));
    return dis / 4.0;
}

float mag(vec3 v) {
    return sqrt((v * v).x + (v * v).y + (v * v).z);
}

void main() {
    // vec2 p = fragCoord.xy / iResolution.xy;
    vec2 p = gl_FragCoord.xy / 1000;
    float dis = mixNoise(vec2(p.x * 100.0, p.y));
    vec3 col = vec3(dis * BRUSH + (1.0 - BRUSH)) * COL;
    float smudges = 0.0;
    smudges = noise(p * 50.0 + smudges);
    smudges = noise(p * 40.0 + smudges);
    smudges = noise(p * 30.0 + smudges);
    smudges = noise(p * 20.0 + smudges);
    smudges *= SMUDGES;
    col = gammaRamp(vec3(mix(col, vec3(1.0 - mag(col)), smudges)));
    col *= 0.9;
    fragColor = vec4(col, 1.0);
}
