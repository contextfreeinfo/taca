let d;
const S = typeof TextDecoder < "u" ? new TextDecoder("utf-8", { ignoreBOM: !0, fatal: !0 }) : { decode: () => {
  throw Error("TextDecoder not available");
} };
typeof TextDecoder < "u" && S.decode();
let x = null;
function w() {
  return (x === null || x.byteLength === 0) && (x = new Uint8Array(d.memory.buffer)), x;
}
function C(n, e) {
  return n = n >>> 0, S.decode(w().subarray(n, n + e));
}
let p = 0;
function I(n, e) {
  const t = e(n.length * 1, 1) >>> 0;
  return w().set(n, t / 1), p = n.length, t;
}
let g = null;
function B() {
  return (g === null || g.byteLength === 0) && (g = new Int32Array(d.memory.buffer)), g;
}
function N(n, e) {
  return n = n >>> 0, w().subarray(n / 1, n / 1 + e);
}
function z(n) {
  try {
    const i = d.__wbindgen_add_to_stack_pointer(-16), a = I(n, d.__wbindgen_export_0), o = p;
    d.lz4Decompress(i, a, o);
    var e = B()[i / 4 + 0], t = B()[i / 4 + 1], r = N(e, t).slice();
    return d.__wbindgen_export_1(e, t * 1, 1), r;
  } finally {
    d.__wbindgen_add_to_stack_pointer(16);
  }
}
function M(n) {
  const e = I(n, d.__wbindgen_export_0), t = p, r = d.shaderNew(e, t);
  return A.__wrap(r);
}
function L(n, e) {
  if (!(n instanceof e))
    throw new Error(`expected instance of ${e.name}`);
  return n.ptr;
}
const R = typeof TextEncoder < "u" ? new TextEncoder("utf-8") : { encode: () => {
  throw Error("TextEncoder not available");
} }, O = typeof R.encodeInto == "function" ? function(n, e) {
  return R.encodeInto(n, e);
} : function(n, e) {
  const t = R.encode(n);
  return e.set(t), {
    read: n.length,
    written: t.length
  };
};
function W(n, e, t) {
  if (t === void 0) {
    const s = R.encode(n), f = e(s.length, 1) >>> 0;
    return w().subarray(f, f + s.length).set(s), p = s.length, f;
  }
  let r = n.length, i = e(r, 1) >>> 0;
  const a = w();
  let o = 0;
  for (; o < r; o++) {
    const s = n.charCodeAt(o);
    if (s > 127) break;
    a[i + o] = s;
  }
  if (o !== r) {
    o !== 0 && (n = n.slice(o)), i = t(i, r, r = o + n.length * 3, 1) >>> 0;
    const s = w().subarray(i + o, i + r), f = O(n, s);
    o += f.written, i = t(i, r, o, 1) >>> 0;
  }
  return p = o, i;
}
function E(n, e, t) {
  let r, i;
  try {
    const s = d.__wbindgen_add_to_stack_pointer(-16);
    L(n, A);
    const f = W(t, d.__wbindgen_export_0, d.__wbindgen_export_2), c = p;
    d.shaderToGlsl(s, n.__wbg_ptr, e, f, c);
    var a = B()[s / 4 + 0], o = B()[s / 4 + 1];
    return r = a, i = o, C(a, o);
  } finally {
    d.__wbindgen_add_to_stack_pointer(16), d.__wbindgen_export_1(r, i, 1);
  }
}
const F = Object.freeze({ Vertex: 0, 0: "Vertex", Fragment: 1, 1: "Fragment" }), v = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((n) => d.__wbg_shader_free(n >>> 0));
class A {
  static __wrap(e) {
    e = e >>> 0;
    const t = Object.create(A.prototype);
    return t.__wbg_ptr = e, v.register(t, t.__wbg_ptr, t), t;
  }
  __destroy_into_raw() {
    const e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, v.unregister(this), e;
  }
  free() {
    const e = this.__destroy_into_raw();
    d.__wbg_shader_free(e);
  }
}
async function V(n, e) {
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
function k() {
  const n = {};
  return n.wbg = {}, n.wbg.__wbindgen_throw = function(e, t) {
    throw new Error(C(e, t));
  }, n;
}
function X(n, e) {
  return d = n.exports, D.__wbindgen_wasm_module = e, g = null, x = null, d;
}
async function D(n) {
  if (d !== void 0) return d;
  typeof n > "u" && (n = new URL("taca.wasm", import.meta.url));
  const e = k();
  (typeof n == "string" || typeof Request == "function" && n instanceof Request || typeof URL == "function" && n instanceof URL) && (n = fetch(n));
  const { instance: t, module: r } = await V(await n, e);
  return X(t, r);
}
function u(n) {
  throw Error(n ?? void 0);
}
class G {
  constructor(e) {
    this.gl = e;
    const t = P(
      e,
      H,
      Y
    );
    this.program = t, this.drawInfoBuffer = e.createBuffer() ?? u(), this.drawInfoIndex = e.getUniformBlockIndex(t, "drawInfo") ?? u(), e.uniformBlockBinding(t, this.drawInfoIndex, U), this.sampler = e.getUniformLocation(t, "sampler") ?? u();
    const r = e.createVertexArray() ?? u();
    this.vertexArray = r;
    const i = new Uint16Array([0, 1, 2, 0, 2, 3]);
    e.bindVertexArray(r), [
      [1, 1, -1, 1, -1, -1, 1, -1],
      [1, 1, 0, 1, 0, 0, 1, 0]
    ].forEach((o, s) => {
      const f = e.createBuffer() ?? u();
      e.bindBuffer(e.ARRAY_BUFFER, f), e.bufferData(e.ARRAY_BUFFER, new Float32Array(o), e.STATIC_DRAW), e.vertexAttribPointer(s, 2, e.FLOAT, !1, 0, 0), e.enableVertexAttribArray(s);
    });
    const a = e.createBuffer() ?? u();
    e.bindBuffer(e.ELEMENT_ARRAY_BUFFER, a), e.bufferData(e.ELEMENT_ARRAY_BUFFER, i, e.STATIC_DRAW), e.bindVertexArray(null);
  }
  draw(e, t, r, i, a, o, s) {
    const { drawInfoBuffer: f, gl: c, program: l, sampler: h, vertexArray: m } = this, _ = new Float32Array([
      t,
      r,
      i,
      a,
      s[0],
      s[1],
      o[0],
      o[1]
    ]);
    c.useProgram(l), c.bindVertexArray(m);
    try {
      c.bindBuffer(c.UNIFORM_BUFFER, f), c.bufferData(c.UNIFORM_BUFFER, _, c.STREAM_DRAW), c.bindBufferBase(c.UNIFORM_BUFFER, U, f), c.activeTexture(c.TEXTURE0), c.bindTexture(c.TEXTURE_2D, e), c.uniform1i(h, 0), c.drawElements(c.TRIANGLES, 6, c.UNSIGNED_SHORT, 0);
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
function P(n, e, t) {
  const r = n.createProgram() ?? u(), i = (a, o) => {
    const s = n.createShader(a) ?? u();
    n.shaderSource(s, o), n.compileShader(s), n.getShaderParameter(s, n.COMPILE_STATUS) ?? u(n.getShaderInfoLog(s)), n.attachShader(r, s);
  };
  return i(n.VERTEX_SHADER, e), i(n.FRAGMENT_SHADER, t), n.linkProgram(r), n.getProgramParameter(r, n.LINK_STATUS) ?? u(n.getProgramInfoLog(r)), r;
}
const U = 0, Y = `#version 300 es
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
`, H = `#version 300 es
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
async function J(n) {
  const [e] = await Promise.all([
    K(n.code),
    D(n.runtimeWasm)
  ]);
  e && await j({ ...n, code: e });
}
class q {
  constructor(e) {
    const t = this.canvas = e.canvas;
    t.addEventListener("mousemove", (r) => {
      const i = t.getBoundingClientRect();
      this.pointerPos = [r.clientX - i.left, r.clientY - i.top];
    }), this.config = e, this.gl = e.canvas.getContext("webgl2"), this.texturePipeline = new G(this.gl), this.resizeCanvas(), new ResizeObserver(() => this.resizeNeeded = !0).observe(e.canvas);
  }
  #r(e) {
    const t = [], { gl: r } = this, i = r.getProgramParameter(e, r.ACTIVE_ATTRIBUTES);
    for (let a = 0; a < i; a += 1) {
      const o = r.getActiveAttrib(e, a) ?? u(), s = r.getAttribLocation(e, o.name);
      t.push({ count: o.size, loc: s, type: o.type });
    }
    return t.sort((a, o) => a.loc - o.loc), this.#o(t), t;
  }
  bufferNew(e, t) {
    const r = this.memoryView(t, 12), i = b(r, 0), a = b(r, 4), o = this.memoryBytes().subarray(i, i + a), { gl: s } = this;
    s.enable(s.BLEND), s.blendFunc(s.SRC_ALPHA, s.ONE_MINUS_SRC_ALPHA);
    const f = s.createBuffer();
    f || u(), this.buffers.push({
      buffer: f,
      kind: ["vertex", "index"][e]
    });
    const c = [s.ARRAY_BUFFER, s.ELEMENT_ARRAY_BUFFER][e] ?? u();
    s.bindBuffer(c, f);
    const l = i ? s.STATIC_DRAW : s.STREAM_DRAW;
    return s.bufferData(c, o, l), this.buffers.length;
  }
  buffers = [];
  canvas;
  config;
  draw(e, t, r) {
    this.#n();
    const { gl: i } = this;
    i.drawElements(i.TRIANGLES, t, i.UNSIGNED_SHORT, e);
  }
  drawText(e, t, r) {
    e && (e != this.textTextureText && (this.textTexture = this.textDraw(e, this.textTexture || void 0), this.textTextureText = e), this.drawTexture(this.textTexture, t, r));
  }
  drawTexture(e, t, r) {
    const {
      canvas: { clientWidth: i, clientHeight: a },
      gl: o,
      pipeline: s,
      textures: f
    } = this, { size: c, texture: l, usedSize: h } = f[e - 1];
    this.texturePipeline.draw(
      l,
      i,
      a,
      t,
      r,
      c,
      h
    ), s && o.useProgram(s.program);
  }
  exports = void 0;
  frameCommit() {
    this.passBegun = !1, this.pipeline = null;
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
  indexBuffer = null;
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
    const i = E(r, F.Vertex, "vertex_main"), a = E(
      r,
      F.Fragment,
      "fragment_main"
    ), o = P(e, i, a), s = this.#r(o), f = this.#s(o);
    t.push({ attributes: s, program: o, uniforms: f });
  }
  #n() {
    this.pipeline || (this.#i(), this.passBegun || this.passBegin(), this.pipelines.length == 1 && this.pipelineApply(1));
  }
  pipelines = [];
  pointerPos = [0, 0];
  readBytes(e) {
    const t = this.memoryBytes(), r = new DataView(t.buffer, e, 2 * 4), i = b(r, 0), a = b(r, 4);
    return t.subarray(i, i + a);
  }
  readString(e) {
    return Z.decode(this.readBytes(e));
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
        const i = new Uint8Array(r.uniforms.tacaSize), a = new DataView(i.buffer);
        a.setFloat32(0, e.width, !0), a.setFloat32(4, e.height, !0), t.bufferSubData(t.UNIFORM_BUFFER, 0, i);
      }
    }
  }
  textDraw(e, t) {
    const { gl: r, offscreen: i, offscreenContext: a, textures: o } = this, s = "30px sans-serif";
    a.font = s;
    const f = a.measureText(e), c = f.width, l = f.fontBoundingBoxAscent + f.fontBoundingBoxDescent;
    i.width < c && (i.width = Math.ceil(c)), i.height < l && (i.height = Math.ceil(l)), a.clearRect(0, 0, i.width, i.height), a.fillStyle = "white", a.font = s, a.textBaseline = "bottom", a.fillText(e, 0, l);
    let h = !t, m;
    if (t) {
      const _ = o[t - 1];
      _.size[0] < i.width || _.size[1] < i.height ? (r.deleteTexture(_.texture), h = !0) : (m = _.texture, _.usedSize = [c, l]);
    }
    if (h) {
      m = r.createTexture() ?? u();
      const _ = {
        size: [i.width, i.height],
        texture: m,
        usedSize: [c, l]
      };
      t || (t = o.length + 1), o[t - 1] = _;
    }
    return r.bindTexture(r.TEXTURE_2D, m), h && (r.texParameteri(r.TEXTURE_2D, r.TEXTURE_WRAP_S, r.CLAMP_TO_EDGE), r.texParameteri(r.TEXTURE_2D, r.TEXTURE_WRAP_T, r.CLAMP_TO_EDGE), r.texParameteri(r.TEXTURE_2D, r.TEXTURE_MIN_FILTER, r.LINEAR), r.texParameteri(r.TEXTURE_2D, r.TEXTURE_MAG_FILTER, r.LINEAR)), r.texImage2D(
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
    this.#n();
    const { gl: t } = this;
    if (!this.uniformsBuffer) {
      const { pipeline: i } = this, a = t.createBuffer() ?? u(), { uniforms: o } = i;
      t.bindBuffer(t.UNIFORM_BUFFER, a), t.bufferData(t.UNIFORM_BUFFER, o.size, t.STREAM_DRAW);
      for (let f = 0; f < o.count; f += 1)
        f != o.tacaIndex && t.bindBufferBase(t.UNIFORM_BUFFER, f + 1, a);
      this.uniformsBuffer = a;
      const s = t.createBuffer() ?? u();
      t.bindBuffer(t.UNIFORM_BUFFER, s), t.bufferData(t.UNIFORM_BUFFER, o.tacaSize, t.STREAM_DRAW), t.bindBufferBase(t.UNIFORM_BUFFER, o.tacaIndex + 1, s), this.tacaBuffer = s, this.tacaBufferUpdate();
    }
    const r = this.readBytes(e);
    t.bindBuffer(t.UNIFORM_BUFFER, this.uniformsBuffer), t.bufferSubData(t.UNIFORM_BUFFER, 0, r);
  }
  uniformsBuffer = null;
  #s(e) {
    const { gl: t } = this, r = t.getProgramParameter(e, t.ACTIVE_UNIFORM_BLOCKS);
    let i = 0, a = 0, o = 0;
    for (let s = 0; s < r; s += 1) {
      const f = t.getActiveUniformBlockName(e, s), c = t.getActiveUniformBlockParameter(
        e,
        s,
        t.UNIFORM_BLOCK_DATA_SIZE
      ) ?? u();
      f == "taca_uniform_block" ? (a = s, o = c) : (s > 0 && c != i && u(), i = c), t.uniformBlockBinding(e, s, s + 1);
    }
    return { count: r, size: i, tacaIndex: a, tacaSize: o };
  }
  #o(e) {
    const { buffers: t, gl: r } = this;
    if (t.length == 2) {
      let i = function(f) {
        let c = 0;
        for (const l of e) {
          const { loc: h } = l, [m, _] = {
            [r.FLOAT_VEC2]: [2, r.FLOAT],
            [r.FLOAT_VEC4]: [4, r.FLOAT]
          }[l.type] ?? u(), T = { [r.FLOAT]: 4 }[_] ?? u();
          c = Math.ceil(c / T) * T, f(h, m, _, c), c += m * T;
        }
        return c;
      };
      const a = t.find((f) => f.kind == "vertex") ?? u();
      r.bindBuffer(r.ARRAY_BUFFER, a.buffer);
      const o = i(() => {
      });
      i((f, c, l, h) => {
        r.enableVertexAttribArray(f), r.vertexAttribPointer(f, c, l, !1, o, h);
      });
      const s = t.find((f) => f.kind == "index") ?? u();
      r.bindBuffer(r.ELEMENT_ARRAY_BUFFER, s.buffer);
    }
  }
  vertexBuffer = null;
}
function b(n, e) {
  return n.getUint32(e, !0);
}
async function j(n) {
  const e = n.code;
  n.code = void 0;
  const t = new Uint8Array(e), r = t[0] == 4 ? (
    // Presume lz4 because wasm starts with 0.
    z(t).buffer
  ) : e;
  let i = new q(n);
  const a = $(i);
  let { instance: o } = await WebAssembly.instantiate(r, { env: a });
  i.init(o);
  const s = i.exports;
  if (s._start(), s.listen) {
    const f = () => {
      try {
        s.listen(0);
      } finally {
        i.frameEnd();
      }
      requestAnimationFrame(f);
    };
    requestAnimationFrame(f);
  }
}
async function K(n) {
  return n instanceof ArrayBuffer ? n : await (await n).arrayBuffer();
}
function $(n) {
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
    taca_RenderingContext_newBuffer(e, t) {
      return n.bufferNew(e, t);
    },
    taca_RenderingContext_newPipeline(e) {
      console.log("taca_RenderingContext_newPipeline");
    },
    taca_RenderingContext_newShader(e) {
      return n.shaders.push(M(n.readBytes(e))), n.shaders.length;
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
      const { clientWidth: t, clientHeight: r } = n.canvas, [i, a] = n.pointerPos, o = n.memoryView(e, 4 * 4);
      y(o, 0, i), y(o, 4, a), y(o, 8, t), y(o, 12, r);
    }
  };
}
function y(n, e, t) {
  return n.setFloat32(e, t, !0);
}
const Z = new TextDecoder();
export {
  J as runApp
};
