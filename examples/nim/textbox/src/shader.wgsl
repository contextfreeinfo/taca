struct VertexOutput {
  @builtin(position) pos: vec4f,
  @location(0) color: vec4f,
};

@vertex
fn vertex_main(
  @location(0) pos: vec2f,
  @location(1) color: vec4f,
  @location(2) offset: vec2f,
  @location(3) scale: vec2f,
) -> VertexOutput {
  var out: VertexOutput;
  out.pos = vec4f(scale * pos + offset, 0, 1);
  out.color = color;
  return out;
}

@fragment
fn fragment_main(
  in: VertexOutput,
) -> @location(0) vec4f {
  return in.color;
}
