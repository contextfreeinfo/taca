struct Uniforms {
    position: vec3f,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2f,
};

struct VertexOutput {
    @builtin(position) position: vec4f,
};

fn noise3_plus(x: vec3f) -> f32 {
    // TODO Use rotations like here? https://www.shadertoy.com/view/XsX3zB
    return 0.0
        + 0.5 * snoise(1.0 * x)
        + 0.3 * snoise(2.0 * x)
        + 0.2 * snoise(4.0 * x);
        // + 0.1 * snoise(8.0 * x);
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4f(in.position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let noise = pow(
        // snoise(
        noise3_plus(
            0.01 * (
                vec3f(in.position.xy, 0.0) +
                uniforms.position.xzy * vec3f(1.0, 1.0, 0.05)
            ),
        ),
        2.0,
    );
    // var color = vec3f(abs(in.position.xy) * 0.001, 0.5);
    // color += (1.0 - color) * 0.25;
    // color *= noise;
    let color = vec3f(vec2f(noise), 1.0) * 0.5 + 0.2;
    // let color = vec3f(rg, 1.0 - min(rg.r + rg.g, 1.0)) * noise;
    // let color = starfield(in.position).rgb;
    return vec4f(pow(color, vec3f(2.2)), 1.0);
}
