struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
  @location(0) in_pos: vec2<f32>,
  @location(1) in_color: vec4<f32>
) -> VertexOutput {
  var out: VertexOutput;
  out.position = vec4<f32>(in_pos, 0.0, 1.0);
  out.color = in_color;
  return out;
}

@fragment
fn fs_main(
  @location(0) color: vec4<f32>
) -> @location(0) vec4<f32> {
  return color;
}
