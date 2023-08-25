// Translated to wgsl from: https://www.shadertoy.com/view/MtcGDf

// Description : Array and textureless GLSL 2D/3D/4D simplex
//               noise functions.
//      Author : Ian McEwan, Ashima Arts.
//  Maintainer : stegu
//     Lastmod : 20110822 (ijm)
//     License : Copyright (C) 2011 Ashima Arts. All rights reserved.
//               Distributed under the MIT License. See LICENSE file.
//               https://github.com/ashima/webgl-noise
//               https://github.com/stegu/webgl-noise

fn mod289v3(x: vec3f) -> vec3f {
  return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn mod289v4(x: vec4f) -> vec4f {
  return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn permute(x: vec4f) -> vec4f {
  return mod289v4(((x * 34.0) + 1.0) * x);
}

fn taylorInvSqrt(r: vec4f) -> vec4f {
  return 1.79284291400159 - 0.85373472095314 * r;
}

fn snoise(v: vec3f) -> f32 {
  let C = vec2f(1.0 / 6.0, 1.0 / 3.0);
  let D = vec4f(0.0, 0.5, 1.0, 2.0);

  // First corner
  var i = floor(v + dot(v, C.yyy));
  let x0 = v - i + dot(i, C.xxx);

  // Other corners
  let g = step(x0.yzx, x0.xyz);
  let l = 1.0 - g;
  let i1 = min(g.xyz, l.zxy);
  let i2 = max(g.xyz, l.zxy);

  //   x0 = x0 - 0.0 + 0.0 * C.xxx;
  //   x1 = x0 - i1  + 1.0 * C.xxx;
  //   x2 = x0 - i2  + 2.0 * C.xxx;
  //   x3 = x0 - 1.0 + 3.0 * C.xxx;
  let x1 = x0 - i1 + C.xxx;
  let x2 = x0 - i2 + C.yyy; // 2.0*C.x = 1/3 = C.y
  let x3 = x0 - D.yyy;      // -1.0+3.0*C.x = -0.5 = -D.y

  // Permutations
  i = mod289v3(i);
  let p = permute(
    permute(
      permute(i.z + vec4f(0.0, i1.z, i2.z, 1.0))
      + i.y + vec4f(0.0, i1.y, i2.y, 1.0)
    )
    + i.x + vec4f(0.0, i1.x, i2.x, 1.0)
  );

  // Gradients: 7x7 points over a square, mapped onto an octahedron.
  // The ring size 17*17 = 289 is close to a multiple of 49 (49*6 = 294)
  let n_ = 0.142857142857; // 1.0/7.0
  let ns = n_ * D.wyz - D.xzx;

  let j = p - 49.0 * floor(p * ns.z * ns.z);  //  mod(p,7*7)

  let x_ = floor(j * ns.z);
  let y_ = floor(j - 7.0 * x_ );    // mod(j,N)

  let x = x_ * ns.x + ns.yyyy;
  let y = y_ * ns.x + ns.yyyy;
  let h = 1.0 - abs(x) - abs(y);

  let b0 = vec4(x.xy, y.xy);
  let b1 = vec4(x.zw, y.zw);

  //vec4 s0 = vec4(lessThan(b0,0.0))*2.0 - 1.0;
  //vec4 s1 = vec4(lessThan(b1,0.0))*2.0 - 1.0;
  let s0 = floor(b0)*2.0 + 1.0;
  let s1 = floor(b1)*2.0 + 1.0;
  let sh = -step(h, vec4(0.0));

  let a0 = b0.xzyw + s0.xzyw*sh.xxyy;
  let a1 = b1.xzyw + s1.xzyw*sh.zzww;

  var p0 = vec3f(a0.xy,h.x);
  var p1 = vec3f(a0.zw,h.y);
  var p2 = vec3f(a1.xy,h.z);
  var p3 = vec3f(a1.zw,h.w);

  // Normalise gradients
  let norm = taylorInvSqrt(vec4(dot(p0,p0), dot(p1,p1), dot(p2, p2), dot(p3,p3)));
  p0 *= norm.x;
  p1 *= norm.y;
  p2 *= norm.z;
  p3 *= norm.w;

  // Mix final noise value
  var m = max(0.6 - vec4f(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3)), vec4f(0.0));
  m = m * m;
  return 42.0 * dot(
    m * m,
    vec4f(dot(p0, x0), dot(p1, x1), dot(p2, x2), dot(p3, x3)),
  );
}

// float tilingNoise(vec2 position, float size) {
//   float value = snoise(vec3(position * size, 0.0));

//   float wrapx = snoise(vec3(position * size - vec2(size, 0.0), 0.0));
//   value = mix(value, wrapx, max(0.0, position.x * size - (size - 1.0)));

//   float wrapy = snoise(vec3(position * size - vec2(0.0, size), 0.0));
//   float wrapxy = snoise(vec3(position * size - vec2(size, size), 0.0));
//   wrapy = mix(wrapy, wrapxy, max(0.0, position.x * size - (size - 1.0)));
// 	return mix(value, wrapy, max(0.0, position.y * size - (size - 1.0)));
// }

// void initialize(out vec4 fragColor, in vec2 fragCoord) {
//   vec2 uv = fragCoord / iResolution.xy;

//   const int octaves = 6;

//   float value = 0.0;
//   float maxValue = 0.0;
//   for (float octave = 0.0; octave < float(octaves); octave++) {
//     value += pow(2.0, -octave) * tilingNoise(uv, 8.0 * pow(2.0, octave));
//       maxValue += pow(2.0, -octave);
//   }

//   maxValue *= 0.5;

//   fragColor = vec4(0.5 * (1.0 + value / maxValue) * vec3(1.0), 1.0);
//   // fragColor.g = iResolution.x;
// }

// void mainImage(out vec4 fragColor, in vec2 fragCoord) {
//     fragColor = texture(iChannel0, fragCoord / iResolution.xy);
//     if (fragColor.g != iResolution.x) {
//     	initialize(fragColor, fragCoord);
//     }
// }
