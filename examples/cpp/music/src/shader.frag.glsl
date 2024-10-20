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
#define SMUDGES 0.03
// intensity of the brush
#define BRUSH 0.08
// offset of noise
#define OFFSET 10.0

// layout(location = 0) in vec2 vTexCoord;
layout(location = 0) out vec4 fragColor;

vec4 gammaRamp(vec4 c) {
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
    return vec2(
        r(n.x * 23.62 - 300.0 + n.y * 34.35),
        r(n.x * 45.13 + 256.0 + n.y * 38.89)
    );
}

float worley(vec2 n, float s) {
    float dis = 2.0;
    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            vec2 p = floor(n / s) + vec2(x, y);
            float d = length(r(p) + vec2(x, y) - fract(n / s));
            if (dis > d) {
                dis = d;
            }
        }
    }
    return 1.0 - dis;
}

// copy from https://www.shadertoy.com/view/4sc3z2

#define MOD3 vec3(.1031, .11369, .13787)

vec3 hash33(vec3 p3) {
    p3 = fract(p3 * MOD3);
    p3 += dot(p3, p3.yxz + 19.19);
    return -1.0 +
        2.0 *
        fract(vec3(
            (p3.x + p3.y) * p3.z,
            (p3.x + p3.z) * p3.y,
            (p3.y + p3.z) * p3.x
        ));
}
float perlin_noise(vec3 p) {
    p += OFFSET;
    vec3 pi = floor(p);
    vec3 pf = p - pi;
    vec3 w = pf * pf * (3.0 - 2.0 * pf);
    float mix000 = dot(pf - vec3(0, 0, 0), hash33(pi + vec3(0, 0, 0)));
    float mix100 = dot(pf - vec3(1, 0, 0), hash33(pi + vec3(1, 0, 0)));
    float mix001 = dot(pf - vec3(0, 0, 1), hash33(pi + vec3(0, 0, 1)));
    float mix101 = dot(pf - vec3(1, 0, 1), hash33(pi + vec3(1, 0, 1)));
    float mix1 = mix(mix(mix000, mix100, w.x), mix(mix001, mix101, w.x), w.z);
    float mix010 = dot(pf - vec3(0, 1, 0), hash33(pi + vec3(0, 1, 0)));
    float mix110 = dot(pf - vec3(1, 1, 0), hash33(pi + vec3(1, 1, 0)));
    float mix011 = dot(pf - vec3(0, 1, 1), hash33(pi + vec3(0, 1, 1)));
    float mix111 = dot(pf - vec3(1, 1, 1), hash33(pi + vec3(1, 1, 1)));
    float mix2 = mix(mix(mix010, mix110, w.x), mix(mix011, mix111, w.x), w.z);
    return mix(mix1, mix2, w.y);
}

float mixNoise(vec3 uv) {
    float dis =
        (1.0 + perlin_noise(vec3(uv.x, uv.y, uv.z) * 8.0)) *
        (1.0 + worley(uv.xy, 32.0));
    return dis / 4.0;
}

float mag(vec3 v) {
    return sqrt((v * v).x + (v * v).y + (v * v).z);
}

void main() {
    // vec2 p = fragCoord.xy / iResolution.xy;
    vec2 p = gl_FragCoord.xy;
    float dis = mixNoise(vec3(vec2(p.x * 100.0, p.y), 0));
    vec3 col = vec3(dis * BRUSH + (1.0 - BRUSH)) * COL;
    float smudges =
        perlin_noise(vec3(p * 20.0 + perlin_noise(vec3(p * 30.0, 0)), 0)) *
        SMUDGES;
    fragColor = gammaRamp(vec4(mix(col, vec3(1.0 - mag(col)), smudges), 1.0));
}
