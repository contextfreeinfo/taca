struct VertexInput {
    @location(0) position: vec2f,
};

struct VertexOutput {
    @builtin(position) position: vec4f,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4f(in.position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let noise = pow(perlinNoise3((vec3f(in.position.xy, 0.0) - 1900.0) * 0.01), 10.0);
    let color = vec3f(abs(in.position.xy) * 0.001, 0.5) * noise;
    return vec4f(pow(color, vec3f(2.2)), 1.0);
}

// Perlin Nose.
// MIT License. © Stefan Gustavson, Munrocket
//
fn permute4(x: vec4f) -> vec4f { return ((x * 34. + 1.) * x) % vec4f(289.); }
fn taylorInvSqrt4(r: vec4f) -> vec4f { return 1.79284291400159 - 0.85373472095314 * r; }
fn fade3(t: vec3f) -> vec3f { return t * t * t * (t * (t * 6. - 15.) + 10.); }

fn perlinNoise3(P: vec3f) -> f32 {
  var Pi0 : vec3f = floor(P); // Integer part for indexing
  var Pi1 : vec3f = Pi0 + vec3f(1.); // Integer part + 1
  Pi0 = Pi0 % vec3f(289.);
  Pi1 = Pi1 % vec3f(289.);
  let Pf0 = fract(P); // Fractional part for interpolation
  let Pf1 = Pf0 - vec3f(1.); // Fractional part - 1.
  let ix = vec4f(Pi0.x, Pi1.x, Pi0.x, Pi1.x);
  let iy = vec4f(Pi0.yy, Pi1.yy);
  let iz0 = Pi0.zzzz;
  let iz1 = Pi1.zzzz;

  let ixy = permute4(permute4(ix) + iy);
  let ixy0 = permute4(ixy + iz0);
  let ixy1 = permute4(ixy + iz1);

  var gx0: vec4f = ixy0 / 7.;
  var gy0: vec4f = fract(floor(gx0) / 7.) - 0.5;
  gx0 = fract(gx0);
  var gz0: vec4f = vec4f(0.5) - abs(gx0) - abs(gy0);
  var sz0: vec4f = step(gz0, vec4f(0.));
  gx0 = gx0 + sz0 * (step(vec4f(0.), gx0) - 0.5);
  gy0 = gy0 + sz0 * (step(vec4f(0.), gy0) - 0.5);

  var gx1: vec4f = ixy1 / 7.;
  var gy1: vec4f = fract(floor(gx1) / 7.) - 0.5;
  gx1 = fract(gx1);
  var gz1: vec4f = vec4f(0.5) - abs(gx1) - abs(gy1);
  var sz1: vec4f = step(gz1, vec4f(0.));
  gx1 = gx1 - sz1 * (step(vec4f(0.), gx1) - 0.5);
  gy1 = gy1 - sz1 * (step(vec4f(0.), gy1) - 0.5);

  var g000: vec3f = vec3f(gx0.x, gy0.x, gz0.x);
  var g100: vec3f = vec3f(gx0.y, gy0.y, gz0.y);
  var g010: vec3f = vec3f(gx0.z, gy0.z, gz0.z);
  var g110: vec3f = vec3f(gx0.w, gy0.w, gz0.w);
  var g001: vec3f = vec3f(gx1.x, gy1.x, gz1.x);
  var g101: vec3f = vec3f(gx1.y, gy1.y, gz1.y);
  var g011: vec3f = vec3f(gx1.z, gy1.z, gz1.z);
  var g111: vec3f = vec3f(gx1.w, gy1.w, gz1.w);

  let norm0 = taylorInvSqrt4(
      vec4f(dot(g000, g000), dot(g010, g010), dot(g100, g100), dot(g110, g110)));
  g000 = g000 * norm0.x;
  g010 = g010 * norm0.y;
  g100 = g100 * norm0.z;
  g110 = g110 * norm0.w;
  let norm1 = taylorInvSqrt4(
      vec4f(dot(g001, g001), dot(g011, g011), dot(g101, g101), dot(g111, g111)));
  g001 = g001 * norm1.x;
  g011 = g011 * norm1.y;
  g101 = g101 * norm1.z;
  g111 = g111 * norm1.w;

  let n000 = dot(g000, Pf0);
  let n100 = dot(g100, vec3f(Pf1.x, Pf0.yz));
  let n010 = dot(g010, vec3f(Pf0.x, Pf1.y, Pf0.z));
  let n110 = dot(g110, vec3f(Pf1.xy, Pf0.z));
  let n001 = dot(g001, vec3f(Pf0.xy, Pf1.z));
  let n101 = dot(g101, vec3f(Pf1.x, Pf0.y, Pf1.z));
  let n011 = dot(g011, vec3f(Pf0.x, Pf1.yz));
  let n111 = dot(g111, Pf1);

  var fade_xyz: vec3f = fade3(Pf0);
  let temp = vec4f(f32(fade_xyz.z)); // simplify after chrome bug fix
  let n_z = mix(vec4f(n000, n100, n010, n110), vec4f(n001, n101, n011, n111), temp);
  let n_yz = mix(n_z.xy, n_z.zw, vec2f(f32(fade_xyz.y))); // simplify after chrome bug fix
  let n_xyz = mix(n_yz.x, n_yz.y, fade_xyz.x);
  return 2.2 * n_xyz;
}
