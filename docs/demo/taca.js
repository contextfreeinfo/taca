let d;
const U = typeof TextDecoder < "u" ? new TextDecoder("utf-8", { ignoreBOM: !0, fatal: !0 }) : { decode: () => {
  throw Error("TextDecoder not available");
} };
typeof TextDecoder < "u" && U.decode();
let y = null;
function w() {
  return (y === null || y.byteLength === 0) && (y = new Uint8Array(d.memory.buffer)), y;
}
function S(n, e) {
  return n = n >>> 0, U.decode(w().subarray(n, n + e));
}
let p = 0;
function C(n, e) {
  const t = e(n.length * 1, 1) >>> 0;
  return w().set(n, t / 1), p = n.length, t;
}
let g = null;
function R() {
  return (g === null || g.byteLength === 0) && (g = new Int32Array(d.memory.buffer)), g;
}
function P(n, e) {
  return n = n >>> 0, w().subarray(n / 1, n / 1 + e);
}
function N(n) {
  try {
    const i = d.__wbindgen_add_to_stack_pointer(-16), o = C(n, d.__wbindgen_export_0), s = p;
    d.lz4Decompress(i, o, s);
    var e = R()[i / 4 + 0], t = R()[i / 4 + 1], r = P(e, t).slice();
    return d.__wbindgen_export_1(e, t * 1, 1), r;
  } finally {
    d.__wbindgen_add_to_stack_pointer(16);
  }
}
function z(n) {
  const e = C(n, d.__wbindgen_export_0), t = p, r = d.shaderNew(e, t);
  return T.__wrap(r);
}
function M(n, e) {
  if (!(n instanceof e))
    throw new Error(`expected instance of ${e.name}`);
  return n.ptr;
}
const A = typeof TextEncoder < "u" ? new TextEncoder("utf-8") : { encode: () => {
  throw Error("TextEncoder not available");
} }, L = typeof A.encodeInto == "function" ? function(n, e) {
  return A.encodeInto(n, e);
} : function(n, e) {
  const t = A.encode(n);
  return e.set(t), {
    read: n.length,
    written: t.length
  };
};
function O(n, e, t) {
  if (t === void 0) {
    const a = A.encode(n), f = e(a.length, 1) >>> 0;
    return w().subarray(f, f + a.length).set(a), p = a.length, f;
  }
  let r = n.length, i = e(r, 1) >>> 0;
  const o = w();
  let s = 0;
  for (; s < r; s++) {
    const a = n.charCodeAt(s);
    if (a > 127) break;
    o[i + s] = a;
  }
  if (s !== r) {
    s !== 0 && (n = n.slice(s)), i = t(i, r, r = s + n.length * 3, 1) >>> 0;
    const a = w().subarray(i + s, i + r), f = L(n, a);
    s += f.written, i = t(i, r, s, 1) >>> 0;
  }
  return p = s, i;
}
function B(n, e, t) {
  let r, i;
  try {
    const a = d.__wbindgen_add_to_stack_pointer(-16);
    M(n, T);
    const f = O(t, d.__wbindgen_export_0, d.__wbindgen_export_2), c = p;
    d.shaderToGlsl(a, n.__wbg_ptr, e, f, c);
    var o = R()[a / 4 + 0], s = R()[a / 4 + 1];
    return r = o, i = s, S(o, s);
  } finally {
    d.__wbindgen_add_to_stack_pointer(16), d.__wbindgen_export_1(r, i, 1);
  }
}
const E = Object.freeze({ Vertex: 0, 0: "Vertex", Fragment: 1, 1: "Fragment" }), F = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((n) => d.__wbg_shader_free(n >>> 0));
class T {
  static __wrap(e) {
    e = e >>> 0;
    const t = Object.create(T.prototype);
    return t.__wbg_ptr = e, F.register(t, t.__wbg_ptr, t), t;
  }
  __destroy_into_raw() {
    const e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, F.unregister(this), e;
  }
  free() {
    const e = this.__destroy_into_raw();
    d.__wbg_shader_free(e);
  }
}
async function W(n, e) {
  if (typeof Response == "function" && n instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming == "function")
      try {
        return await WebAssembly.instantiateStreaming(n, e);
      } catch (r) {
        if (n.headers.get("Content-Type") != "application/wasm")
          console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", r);
        else
          throw r;
      }
    const t = await n.arrayBuffer();
    return await WebAssembly.instantiate(t, e);
  } else {
    const t = await WebAssembly.instantiate(n, e);
    return t instanceof WebAssembly.Instance ? { instance: t, module: n } : t;
  }
}
function V() {
  const n = {};
  return n.wbg = {}, n.wbg.__wbindgen_throw = function(e, t) {
    throw new Error(S(e, t));
  }, n;
}
function k(n, e) {
  return d = n.exports, I.__wbindgen_wasm_module = e, g = null, y = null, d;
}
async function I(n) {
  if (d !== void 0) return d;
  typeof n > "u" && (n = new URL("taca.wasm", import.meta.url));
  const e = V();
  (typeof n == "string" || typeof Request == "function" && n instanceof Request || typeof URL == "function" && n instanceof URL) && (n = fetch(n));
  const { instance: t, module: r } = await W(await n, e);
  return k(t, r);
}
function u(n) {
  throw Error(n ?? void 0);
}
class X {
  constructor(e) {
    this.gl = e;
    const t = D(
      e,
      Y,
      G
    );
    this.program = t, this.drawInfoBuffer = e.createBuffer() ?? u(), this.drawInfoIndex = e.getUniformBlockIndex(t, "drawInfo") ?? u(), e.uniformBlockBinding(t, this.drawInfoIndex, v), this.sampler = e.getUniformLocation(t, "sampler") ?? u();
    const r = e.createVertexArray() ?? u();
    this.vertexArray = r;
    const i = new Uint16Array([0, 1, 2, 0, 2, 3]);
    e.bindVertexArray(r), [
      [1, 1, -1, 1, -1, -1, 1, -1],
      [1, 1, 0, 1, 0, 0, 1, 0]
    ].forEach((s, a) => {
      const f = e.createBuffer() ?? u();
      e.bindBuffer(e.ARRAY_BUFFER, f), e.bufferData(e.ARRAY_BUFFER, new Float32Array(s), e.STATIC_DRAW), e.vertexAttribPointer(a, 2, e.FLOAT, !1, 0, 0), e.enableVertexAttribArray(a);
    });
    const o = e.createBuffer() ?? u();
    e.bindBuffer(e.ELEMENT_ARRAY_BUFFER, o), e.bufferData(e.ELEMENT_ARRAY_BUFFER, i, e.STATIC_DRAW), e.bindVertexArray(null);
  }
  draw(e, t, r, i, o, s, a) {
    const { drawInfoBuffer: f, gl: c, program: l, sampler: _, vertexArray: h } = this, m = new Float32Array([
      t,
      r,
      i,
      o,
      a[0],
      a[1],
      s[0],
      s[1]
    ]);
    c.useProgram(l), c.bindVertexArray(h);
    try {
      c.bindBuffer(c.UNIFORM_BUFFER, f), c.bufferData(c.UNIFORM_BUFFER, m, c.STREAM_DRAW), c.bindBufferBase(c.UNIFORM_BUFFER, v, f), c.activeTexture(c.TEXTURE0), c.bindTexture(c.TEXTURE_2D, e), c.uniform1i(_, 0), c.drawElements(c.TRIANGLES, 6, c.UNSIGNED_SHORT, 0);
    } finally {
      c.bindVertexArray(null);
    }
  }
  drawInfoBuffer;
  drawInfoIndex;
  // TODO Need to use this?
  gl;
  program;
  sampler;
  vertexArray;
}
function D(n, e, t) {
  const r = n.createProgram() ?? u(), i = (o, s) => {
    const a = n.createShader(o) ?? u();
    n.shaderSource(a, s), n.compileShader(a), n.getShaderParameter(a, n.COMPILE_STATUS) ?? u(n.getShaderInfoLog(a)), n.attachShader(r, a);
  };
  return i(n.VERTEX_SHADER, e), i(n.FRAGMENT_SHADER, t), n.linkProgram(r), n.getProgramParameter(r, n.LINK_STATUS) ?? u(n.getProgramInfoLog(r)), r;
}
const v = 0, G = `#version 300 es
precision mediump float;
in vec2 vTexCoord;
out vec4 outColor;
uniform drawInfo {
  vec2 canvasSize;
  vec2 drawPos;
  vec2 drawSize;
  vec2 textureSize;
};
uniform sampler2D sampler;
void main() {
  outColor = texture(sampler, vTexCoord * drawSize / textureSize);
}
`, Y = `#version 300 es
precision mediump float;
layout(location = 0) in vec2 framePos;
layout(location = 1) in vec2 texCoord;
uniform drawInfo {
  vec2 canvasSize;
  vec2 drawPos;
  vec2 drawSize;
  vec2 textureSize;
};
out vec2 vTexCoord;
void main() {
  vec2 pos = framePos * drawSize * 0.5 + drawPos;
  pos.y = canvasSize.y - pos.y;
  pos = (pos / canvasSize) * 2.0 - 1.0;
  gl_Position = vec4(pos, 0.0, 1.0);
  vTexCoord = texCoord;
}
`;
async function Z(n) {
  const [e] = await Promise.all([n.wasm ?? j(), I()]);
  e && await q({ ...n, wasm: e });
}
class H {
  constructor(e) {
    const t = this.canvas = e.canvas;
    t.addEventListener("mousemove", (r) => {
      const i = t.getBoundingClientRect();
      this.pointerPos = [r.clientX - i.left, r.clientY - i.top];
    }), this.config = e, this.gl = e.canvas.getContext("webgl2"), this.texturePipeline = new X(this.gl), this.resizeCanvas(), new ResizeObserver(() => this.resizeNeeded = !0).observe(e.canvas);
  }
  #n(e) {
    const t = [], { gl: r } = this, i = r.getProgramParameter(e, r.ACTIVE_ATTRIBUTES);
    for (let o = 0; o < i; o += 1) {
      const s = r.getActiveAttrib(e, o) ?? u(), a = r.getAttribLocation(e, s.name);
      t.push({ count: s.size, loc: a, type: s.type });
    }
    return t.sort((o, s) => o.loc - s.loc), this.#a(t), t;
  }
  bufferNew(e, t, r) {
    const i = this.memoryView(r, 12), o = x(i, 0), s = x(i, 4), a = x(i, 8), f = this.memoryBytes().subarray(o, o + s), { gl: c } = this;
    c.enable(c.BLEND), c.blendFunc(c.SRC_ALPHA, c.ONE_MINUS_SRC_ALPHA);
    const l = c.createBuffer();
    l || u(), this.buffers.push({
      buffer: l,
      itemSize: a,
      kind: ["vertex", "index"][e]
    });
    const _ = [c.ARRAY_BUFFER, c.ELEMENT_ARRAY_BUFFER][e] ?? u();
    c.bindBuffer(_, l);
    const h = [c.STATIC_DRAW, c.DYNAMIC_DRAW, c.STREAM_DRAW][t] ?? u();
    return c.bufferData(_, f, h), this.buffers.length;
  }
  buffers = [];
  canvas;
  config;
  draw(e, t, r) {
    this.#r();
    const { gl: i } = this;
    this.vertexArray || (this.vertexArray = this.vertexArrays[0] ?? u(), i.bindVertexArray(this.vertexArray)), i.drawElements(i.TRIANGLES, t, i.UNSIGNED_SHORT, e);
  }
  drawText(e, t, r) {
    e && (e != this.textTextureText && (this.textTexture = this.textDraw(e, this.textTexture || void 0), this.textTextureText = e), this.drawTexture(this.textTexture, t, r));
  }
  drawTexture(e, t, r) {
    const {
      canvas: { clientWidth: i, clientHeight: o },
      gl: s,
      pipeline: a,
      textures: f
    } = this, { size: c, texture: l, usedSize: _ } = f[e - 1];
    this.texturePipeline.draw(
      l,
      i,
      o,
      t,
      r,
      c,
      _
    ), a && s.useProgram(a.program);
  }
  exports = void 0;
  frameCommit() {
    this.passBegun = !1, this.pipeline = this.vertexArray = null;
  }
  frameCount = 0;
  frameEnd() {
    this.passBegun && this.frameCommit();
    const e = 1e3;
    if (this.frameCount += 1, this.frameCount = this.frameCount % e, !this.frameCount) {
      const t = Date.now(), r = (t - this.frameTimeBegin) * 1e-3, i = e / r;
      console.log(`fps: ${i}`), this.frameTimeBegin = t;
    }
  }
  frameTimeBegin = Date.now();
  gl;
  init(e) {
    this.exports = e.exports, this.memory = e.exports.memory;
  }
  memory = void 0;
  #e = null;
  #t = null;
  memoryBytes() {
    return this.#e != this.memory.buffer && (this.#e = this.memory.buffer, this.#t = new Uint8Array(this.#e)), this.#t;
  }
  memoryView(e, t) {
    return new DataView(this.memory.buffer, e, t);
  }
  offscreen = new OffscreenCanvas(1, 1);
  offscreenContext = this.offscreen.getContext("2d") ?? u();
  passBegin() {
    let { gl: e, resizeNeeded: t } = this;
    t && this.resizeCanvas(), e.clearColor(0, 0, 0, 1), e.clear(e.COLOR_BUFFER_BIT | e.DEPTH_BUFFER_BIT), this.passBegun = !0;
  }
  passBegun = !1;
  pipeline = null;
  pipelineApply(e) {
    let { gl: t, pipelines: r } = this;
    const i = this.pipeline = r[e - 1] ?? u();
    t.useProgram(i.program);
  }
  #i() {
    const {
      gl: e,
      pipelines: t,
      shaders: [r]
    } = this;
    if (t.length) return;
    const i = B(r, E.Vertex, "vertex_main"), o = B(r, E.Fragment, "fragment_main"), s = D(e, i, o), a = this.#n(s), f = this.#s(s);
    t.push({ attributes: a, program: s, uniforms: f });
  }
  #r() {
    this.pipeline || (this.#i(), this.passBegun || this.passBegin(), this.pipelines.length == 1 && this.pipelineApply(1));
  }
  pipelines = [];
  pointerPos = [0, 0];
  readBytes(e) {
    const t = this.memoryBytes(), r = new DataView(t.buffer, e, 2 * 4), i = x(r, 0), o = x(r, 4);
    return t.subarray(i, i + o);
  }
  readString(e) {
    return $.decode(this.readBytes(e));
  }
  resizeCanvas() {
    const { canvas: e } = this.config;
    e.width = e.clientWidth, e.height = e.clientHeight, this.gl.viewport(0, 0, e.width, e.height), this.resizeNeeded = !1, this.tacaBufferUpdate();
  }
  resizeNeeded = !1;
  shaders = [];
  tacaBuffer = null;
  tacaBufferUpdate() {
    const { canvas: e } = this.config;
    if (this.tacaBuffer) {
      const { gl: t } = this;
      for (const r of this.pipelines) {
        t.bindBuffer(t.UNIFORM_BUFFER, this.tacaBuffer);
        const i = new Uint8Array(r.uniforms.tacaSize), o = new DataView(i.buffer);
        o.setFloat32(0, e.width, !0), o.setFloat32(4, e.height, !0), t.bufferSubData(t.UNIFORM_BUFFER, 0, i);
      }
    }
  }
  textDraw(e, t) {
    const { gl: r, offscreen: i, offscreenContext: o, textures: s } = this, a = "30px sans-serif";
    o.font = a;
    const f = o.measureText(e), c = f.width, l = f.fontBoundingBoxAscent + f.fontBoundingBoxDescent;
    i.width < c && (i.width = Math.ceil(c)), i.height < l && (i.height = Math.ceil(l)), o.clearRect(0, 0, i.width, i.height), o.fillStyle = "white", o.font = a, o.textBaseline = "bottom", o.fillText(e, 0, l);
    let _ = !t, h;
    if (t) {
      const m = s[t - 1];
      m.size[0] < i.width || m.size[1] < i.height ? (r.deleteTexture(m.texture), _ = !0) : (h = m.texture, m.usedSize = [c, l]);
    }
    if (_) {
      h = r.createTexture() ?? u();
      const m = {
        size: [i.width, i.height],
        texture: h,
        usedSize: [c, l]
      };
      t || (t = s.length + 1), s[t - 1] = m;
    }
    return r.bindTexture(r.TEXTURE_2D, h), _ && (r.texParameteri(r.TEXTURE_2D, r.TEXTURE_WRAP_S, r.CLAMP_TO_EDGE), r.texParameteri(r.TEXTURE_2D, r.TEXTURE_WRAP_T, r.CLAMP_TO_EDGE), r.texParameteri(r.TEXTURE_2D, r.TEXTURE_MIN_FILTER, r.LINEAR), r.texParameteri(r.TEXTURE_2D, r.TEXTURE_MAG_FILTER, r.LINEAR)), r.texImage2D(
      r.TEXTURE_2D,
      0,
      r.RGBA,
      r.RGBA,
      r.UNSIGNED_BYTE,
      i
    ), t;
  }
  textTexture = 0;
  textTextureText = "";
  texturePipeline;
  textures = [];
  uniformsApply(e) {
    this.#r();
    const { gl: t } = this;
    if (!this.uniformsBuffer) {
      const { pipeline: i } = this, o = t.createBuffer() ?? u(), { uniforms: s } = i;
      t.bindBuffer(t.UNIFORM_BUFFER, o), t.bufferData(t.UNIFORM_BUFFER, s.size, t.STREAM_DRAW);
      for (let f = 0; f < s.count; f += 1)
        f != s.tacaIndex && t.bindBufferBase(t.UNIFORM_BUFFER, f + 1, o);
      this.uniformsBuffer = o;
      const a = t.createBuffer() ?? u();
      t.bindBuffer(t.UNIFORM_BUFFER, a), t.bufferData(t.UNIFORM_BUFFER, s.tacaSize, t.STREAM_DRAW), t.bindBufferBase(t.UNIFORM_BUFFER, s.tacaIndex + 1, a), this.tacaBuffer = a, this.tacaBufferUpdate();
    }
    const r = this.readBytes(e);
    t.bindBuffer(t.UNIFORM_BUFFER, this.uniformsBuffer), t.bufferSubData(t.UNIFORM_BUFFER, 0, r);
  }
  uniformsBuffer = null;
  #s(e) {
    const { gl: t } = this, r = t.getProgramParameter(e, t.ACTIVE_UNIFORM_BLOCKS);
    let i = 0, o = 0, s = 0;
    for (let a = 0; a < r; a += 1) {
      const f = t.getActiveUniformBlockName(e, a), c = t.getActiveUniformBlockParameter(
        e,
        a,
        t.UNIFORM_BLOCK_DATA_SIZE
      ) ?? u();
      f == "taca_uniform_block" ? (o = a, s = c) : (a > 0 && c != i && u(), i = c), t.uniformBlockBinding(e, a, a + 1);
    }
    return { count: r, size: i, tacaIndex: o, tacaSize: s };
  }
  vertexArray = null;
  #a(e) {
    const { buffers: t, gl: r } = this;
    if (t.length == 2) {
      const i = r.createVertexArray() ?? u();
      r.bindVertexArray(i);
      try {
        const o = t.find((f) => f.kind == "vertex") ?? u();
        r.bindBuffer(r.ARRAY_BUFFER, o.buffer);
        let s = 0;
        for (const f of e) {
          const { loc: c } = f;
          r.enableVertexAttribArray(c);
          const [l, _] = {
            [r.FLOAT_VEC2]: [2, r.FLOAT],
            [r.FLOAT_VEC4]: [4, r.FLOAT]
          }[f.type] ?? u(), h = { [r.FLOAT]: 4 }[_] ?? u();
          s = Math.ceil(s / h) * h;
          let { itemSize: m } = o;
          r.vertexAttribPointer(c, l, _, !1, m, s), s += l * h;
        }
        const a = t.find((f) => f.kind == "index") ?? u();
        r.bindBuffer(r.ELEMENT_ARRAY_BUFFER, a.buffer);
      } finally {
        r.bindVertexArray(null);
      }
      this.vertexArrays.push(i);
    }
  }
  vertexArrays = [];
}
function x(n, e) {
  return n.getUint32(e, !0);
}
async function q(n) {
  const e = n.wasm;
  n.wasm = void 0;
  const t = new Uint8Array(e), r = t[0] == 4 ? (
    // Presume lz4 because wasm starts with 0.
    N(t).buffer
  ) : e;
  let i = new H(n);
  const o = K(i);
  let { instance: s } = await WebAssembly.instantiate(r, { env: o });
  i.init(s);
  const a = i.exports;
  if (a._start(), a.listen) {
    const f = () => {
      try {
        a.listen(0);
      } finally {
        i.frameEnd();
      }
      requestAnimationFrame(f);
    };
    requestAnimationFrame(f);
  }
}
async function j() {
  const n = new URL(window.location.href), t = new URLSearchParams(n.search).get("app");
  if (t)
    return await (await fetch(t)).arrayBuffer();
}
function K(n) {
  return {
    taca_RenderingContext_applyBindings(e) {
    },
    taca_RenderingContext_applyPipeline(e) {
    },
    taca_RenderingContext_applyUniforms(e) {
      n.uniformsApply(e);
    },
    taca_RenderingContext_beginPass() {
    },
    taca_RenderingContext_commitFrame() {
      n.frameCommit();
    },
    taca_RenderingContext_draw(e, t, r) {
      n.draw(e, t, r);
    },
    taca_RenderingContext_drawText(e, t, r) {
      n.drawText(n.readString(e), t, r);
    },
    taca_RenderingContext_drawTexture(e, t, r) {
      n.drawTexture(e, t, r);
    },
    taca_RenderingContext_endPass() {
    },
    taca_RenderingContext_newBuffer(e, t, r) {
      return n.bufferNew(e, t, r);
    },
    taca_RenderingContext_newPipeline(e) {
      console.log("taca_RenderingContext_newPipeline");
    },
    taca_RenderingContext_newShader(e) {
      return n.shaders.push(z(n.readBytes(e))), n.shaders.length;
    },
    taca_Text_draw(e) {
      return n.textDraw(n.readString(e));
    },
    taca_Window_newRenderingContext() {
      return 1;
    },
    taca_Window_print(e) {
      console.log(n.readString(e));
    },
    taca_Window_setTitle(e) {
      document.title = n.readString(e);
    },
    taca_Window_state(e) {
      const { clientWidth: t, clientHeight: r } = n.canvas, [i, o] = n.pointerPos, s = n.memoryView(e, 4 * 4);
      b(s, 0, i), b(s, 4, o), b(s, 8, t), b(s, 12, r);
    }
  };
}
function b(n, e, t) {
  return n.setFloat32(e, t, !0);
}
const $ = new TextDecoder();
export {
  Z as runApp
};
