struct Uniforms {
  aspect: vec2f,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var diffuse_texture: texture_2d<f32>;
@group(0) @binding(2) var diffuse_sampler: sampler;

struct VertexOutput {
  @builtin(position) pos: vec4f,
  @location(1) uv: vec2f,
};

@vertex
fn vertex_main(
  @location(0) pos: vec2f,
  @location(1) offset: vec2f,
  @location(2) scale: vec2f,
  @location(3) source_offset: vec2f,
  @location(4) source_scale: vec2f,
) -> VertexOutput {
  var out: VertexOutput;
  out.pos = vec4f((pos * scale + offset) * uniforms.aspect, 0, 1);
  out.uv = pos * source_scale + source_offset;
  return out;
}

@fragment
fn fragment_main(
  in: VertexOutput,
) -> @location(0) vec4f {
  let color = textureSample(diffuse_texture, diffuse_sampler, in.uv);
  return color;
}
