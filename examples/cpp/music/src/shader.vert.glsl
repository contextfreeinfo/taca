#version 450

precision mediump float;

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 offset;
layout (location = 2) in float scale;
layout (location = 3) in float light;

layout (location = 0) out float fragLight;

void main() {
    // gl_Position = vec4(scale * pos + offset, 0.0, 1.0);
    gl_Position = vec4(pos, 0.0, 1.0);
    fragLight = light;
}
