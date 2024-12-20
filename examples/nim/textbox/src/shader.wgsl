struct Uniforms {
  color: vec4f,
  frames: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

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
  let time = uniforms.frames;
  let scale1 = decorate((in.pos.xy + time * 1e-1) * 1e-2);
  let scale2 = decorate((in.pos.xy + time * 2e-1) * 5e-2);
  return in.color * uniforms.color * (0.5 * scale1 + 0.5 * scale2);
}

fn decorate(pos: vec2f) -> f32 {
  return smoothstep(-1.0, 1.0, sin(pos.x + pos.y));
}
