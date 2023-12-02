struct Uniforms {
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
    time: f32,
    position: vec3f,
}

// Variable in the *uniform* address space
// The memory location of the uniform is given by a pair of a *bind group* and
// a *binding*.
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

//@group(0) @binding(1) var textureSampler: sampler;
// @group(0) @binding(1) var texture: texture_2d<f32>;

/**
 * A structure with fields labeled with vertex attribute locations can be used
 * as input to the entry point of a shader.
 */
struct VertexInput {
    @location(0) position: vec3f,
    @location(1) normal: vec3f,
    @location(2) color: vec3f,
};

/**
 * A structure with fields labeled with builtins and locations can also be used
 * as *output* of the vertex shader, which is also the input of the fragment
 * shader.
 */
struct VertexOutput {
    @builtin(position) position: vec4f,
    // The location here does not refer to a vertex attribute, it just means
    // that this field must be handled by the rasterizer.
    // (It can also refer to another field of another struct that would be used
    // as input to the fragment shader.)
    @location(0) normal: vec3f,
    @location(1) color: vec3f,
};

const tau = 6.2831855;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let angle = uniforms.time;
    let ca = cos(angle);
    let sa = sin(angle);
    let rotation = transpose(mat4x4<f32>(
        ca, sa, 0.0, 0.0,
        -sa, ca, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ));
	let tf_obj = rotation;
    // let tf_obj = rotation * transpose(mat4x4<f32>(
    //     1.0, 0.0, 0.0, uniforms.position.x,
    //     0.0, 1.0, 0.0, uniforms.position.y,
    //     0.0, 0.0, 1.0, uniforms.position.z,
    //     0.0, 0.0, 0.0, 1.0,
    // ));
    let pos =
        uniforms.projection *
        uniforms.view *
        tf_obj * vec4<f32>(0.3 * in.position, 1.0)
		+ vec4f(uniforms.position, 1.0);
    out.position = vec4f(pos.x, pos.y, pos.z * 0.5 + 0.5, 1.0);
    // Fails. Maybe things still wrong in the projection matrix?
    // out.position = vec4f(pos.xyz, 1.0);
    out.normal = (rotation * vec4<f32>(in.normal, 1.0)).xyz;
    // out.normal = in.normal * 0.5 + 0.5;
    out.color = in.color; // forward to the fragment shader
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let light_direction = vec3<f32>(-0.5, 0.5, 0.1);
    let shading = max(0.0, dot(light_direction, in.normal)) * 0.8 + 0.2;
    var color = in.color;
    color = color * 0.6 + 0.4;
    color *= shading;
    // Hack texture.
    // color = 0.97 * color + 0.03 * textureLoad(texture, vec2<i32>(in.position.xy / 10.0), 0).rgb;
    // Convert approximate srgb color space.
    // TODO Only if srgb format! TODO Need uniform to indicate that?
    return vec4f(pow(color, vec3f(2.2)), 1.0);
}
