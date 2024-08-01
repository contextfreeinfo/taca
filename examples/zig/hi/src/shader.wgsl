struct Uniforms {
  aspect: vec2f,
  pointer: vec2f,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
};

@vertex
fn vertex_main(
  @location(0) in_pos: vec2f,
  @location(1) in_color: vec4f,
) -> VertexOutput {
  var out: VertexOutput;
  out.position = vec4f(in_pos * uniforms.aspect, 0.0, 1.0);
  out.color = in_color;
  return out;
}

@fragment
fn fragment_main(
  in: VertexOutput,
) -> @location(0) vec4f {
  let distance = length(uniforms.pointer - in.position.xy);
  let shine = 1.0 - min(1e-2 * distance, 1.0);
  return vec4f(in.color.rgb + shine, 1.0);
}
