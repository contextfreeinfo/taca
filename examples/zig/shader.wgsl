struct Uniforms {
	aspect: f32,
	time: f32,
}

// Variable in the *uniform* address space
// The memory location of the uniform is given by a pair of a *bind group* and
// a *binding*.
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

/**
 * A structure with fields labeled with vertex attribute locations can be used
 * as input to the entry point of a shader.
 */
struct VertexInput {
	@location(0) position: vec3f,
	@location(1) color: vec3f,
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
	@location(0) color: vec3f,
};

const tau = 6.2831855;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
	var out: VertexOutput;
	let angle = uniforms.time;
	let ca = cos(angle);
	let sa = sin(angle);
	let tf_obj = transpose(mat4x4<f32>(
		ca, sa, 0.0, 0.0,
		-sa, ca, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		0.0, 0.0, 0.0, 1.0,
	)) * transpose(mat4x4<f32>(
		1.0, 0.0, 0.0, 0.5,
		0.0, 1.0, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		0.0, 0.0, 0.0, 1.0,
	));
	let view = 3.0 * tau / 8.0;
	let cv = cos(view);
	let sv = sin(view);
	let rot_view = transpose(mat4x4<f32>(
		1.0, 0.0, 0.0, 0.0,
		0.0, cv, sv, 0.0,
		0.0, -sv, cv, 0.0,
		0.0, 0.0, 0.0, 1.0,
	));
	let pos = rot_view * tf_obj * vec4<f32>(0.3 * in.position, 1.0);
	out.position = vec4f(pos.x, pos.y * uniforms.aspect, pos.z * 0.5 + 0.5, 1.0);
	out.color = in.color; // forward to the fragment shader
	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
	// Convert approximate srgb color space.
	// TODO Only if srgb format! TODO Need uniform to indicate that?
	let linear_color = pow(in.color, vec3f(2.2));
	return 0.7 * vec4f(linear_color, 1.0) + 0.1;
}
