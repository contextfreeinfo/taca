@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
	var p = vec2<f32>(0.0, 0.0);
	if (in_vertex_index == 0u) {
		p = vec2<f32>(-0.5, -0.5);
	} else if (in_vertex_index == 1u) {
		p = vec2<f32>(0.5, -0.5);
	} else {
		p = vec2<f32>(0.0, 0.5);
	}
	return vec4<f32>(p, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.3, 0.7, 1.0);
}
