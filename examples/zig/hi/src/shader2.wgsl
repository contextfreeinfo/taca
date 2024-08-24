struct Uniforms {
  aspect: vec2f,
  pointer: vec2f,
  count: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
};

// Main shader

@vertex
fn vertex_main(
  @location(0) in_pos: vec2f,
  @location(1) in_color: vec4f,
) -> VertexOutput {
  var out: VertexOutput;
  let scale = sin(uniforms.count * 1e-2);
  out.position = vec4f(in_pos * uniforms.aspect / scale, 0, 1);
  out.color = in_color;
  return out;
}

@fragment
fn fragment_main(
  in: VertexOutput,
) -> @location(0) vec4f {
  let distance = length(uniforms.pointer - in.position.xy);
  let shine = 1.0 - min(1e-2 * distance, 1.0);
  return vec4f(in.color.rgb + shine, 1);
}

// Shader for additional instanced decoration

@vertex
fn vertex_decor(
  @location(0) point: vec2f,
  @location(1) center: vec2f,
) -> @builtin(position) vec4f {
  let r = uniforms.count * 1e-2;
  let cos_r = cos(r);
  let sin_r = sin(r);
  let rot = mat2x2f(vec2f(cos_r, sin_r), vec2f(-sin_r, cos_r));
  // Rotate around the origin then again after translation.
  let result = rot * (0.2 * rot * point + 0.7 * center);
  return vec4f(result * uniforms.aspect, 0, 1);
}

@fragment
fn fragment_decor(
  @builtin(position) position: vec4f,
) -> @location(0) vec4f {
  // If we don't reference uniforms, naga leaves them out, which causes trouble.
  // TODO Solve the missing uniforms problem.
  let phase = 0.5 + 0.2 * sin(uniforms.count * 2e-2);
  return vec4f(1, 1, 0.8, phase);
}
