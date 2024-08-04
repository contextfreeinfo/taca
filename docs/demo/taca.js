let d;
const S = typeof TextDecoder < "u" ? new TextDecoder("utf-8", { ignoreBOM: !0, fatal: !0 }) : { decode: () => {
  throw Error("TextDecoder not available");
} };
typeof TextDecoder < "u" && S.decode();
let g = null;
function w() {
  return (g === null || g.byteLength === 0) && (g = new Uint8Array(d.memory.buffer)), g;
}
function C(r, e) {
  return r = r >>> 0, S.decode(w().subarray(r, r + e));
}
let p = 0;
function I(r, e) {
  const t = e(r.length * 1, 1) >>> 0;
  return w().set(r, t / 1), p = r.length, t;
}
let x = null;
function B() {
  return (x === null || x.byteLength === 0) && (x = new Int32Array(d.memory.buffer)), x;
}
function N(r, e) {
  return r = r >>> 0, w().subarray(r / 1, r / 1 + e);
}
function z(r) {
  try {
    const i = d.__wbindgen_add_to_stack_pointer(-16), o = I(r, d.__wbindgen_export_0), a = p;
    d.lz4Decompress(i, o, a);
    var e = B()[i / 4 + 0], t = B()[i / 4 + 1], n = N(e, t).slice();
    return d.__wbindgen_export_1(e, t * 1, 1), n;
  } finally {
    d.__wbindgen_add_to_stack_pointer(16);
  }
}
function M(r) {
  const e = I(r, d.__wbindgen_export_0), t = p, n = d.shaderNew(e, t);
  return T.__wrap(n);
}
function L(r, e) {
  if (!(r instanceof e))
    throw new Error(`expected instance of ${e.name}`);
  return r.ptr;
}
const R = typeof TextEncoder < "u" ? new TextEncoder("utf-8") : { encode: () => {
  throw Error("TextEncoder not available");
} }, O = typeof R.encodeInto == "function" ? function(r, e) {
  return R.encodeInto(r, e);
} : function(r, e) {
  const t = R.encode(r);
  return e.set(t), {
    read: r.length,
    written: t.length
  };
};
function W(r, e, t) {
  if (t === void 0) {
    const s = R.encode(r), c = e(s.length, 1) >>> 0;
    return w().subarray(c, c + s.length).set(s), p = s.length, c;
  }
  let n = r.length, i = e(n, 1) >>> 0;
  const o = w();
  let a = 0;
  for (; a < n; a++) {
    const s = r.charCodeAt(a);
    if (s > 127) break;
    o[i + a] = s;
  }
  if (a !== n) {
    a !== 0 && (r = r.slice(a)), i = t(i, n, n = a + r.length * 3, 1) >>> 0;
    const s = w().subarray(i + a, i + n), c = O(r, s);
    a += c.written, i = t(i, n, a, 1) >>> 0;
  }
  return p = a, i;
}
function E(r, e, t) {
  let n, i;
  try {
    const s = d.__wbindgen_add_to_stack_pointer(-16);
    L(r, T);
    const c = W(t, d.__wbindgen_export_0, d.__wbindgen_export_2), f = p;
    d.shaderToGlsl(s, r.__wbg_ptr, e, c, f);
    var o = B()[s / 4 + 0], a = B()[s / 4 + 1];
    return n = o, i = a, C(o, a);
  } finally {
    d.__wbindgen_add_to_stack_pointer(16), d.__wbindgen_export_1(n, i, 1);
  }
}
const F = Object.freeze({ Vertex: 0, 0: "Vertex", Fragment: 1, 1: "Fragment" }), v = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((r) => d.__wbg_shader_free(r >>> 0));
class T {
  static __wrap(e) {
    e = e >>> 0;
    const t = Object.create(T.prototype);
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
async function V(r, e) {
  if (typeof Response == "function" && r instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming == "function")
      try {
        return await WebAssembly.instantiateStreaming(r, e);
      } catch (n) {
        if (r.headers.get("Content-Type") != "application/wasm")
          console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", n);
        else
          throw n;
      }
    const t = await r.arrayBuffer();
    return await WebAssembly.instantiate(t, e);
  } else {
    const t = await WebAssembly.instantiate(r, e);
    return t instanceof WebAssembly.Instance ? { instance: t, module: r } : t;
  }
}
function k() {
  const r = {};
  return r.wbg = {}, r.wbg.__wbindgen_throw = function(e, t) {
    throw new Error(C(e, t));
  }, r;
}
function X(r, e) {
  return d = r.exports, D.__wbindgen_wasm_module = e, x = null, g = null, d;
}
async function D(r) {
  if (d !== void 0) return d;
  typeof r > "u" && (r = new URL("taca.wasm", import.meta.url));
  const e = k();
  (typeof r == "string" || typeof Request == "function" && r instanceof Request || typeof URL == "function" && r instanceof URL) && (r = fetch(r));
  const { instance: t, module: n } = await V(await r, e);
  return X(t, n);
}
function u(r) {
  throw Error(r ?? void 0);
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
    const n = e.createVertexArray() ?? u();
    this.vertexArray = n;
    const i = new Uint16Array([0, 1, 2, 0, 2, 3]);
    e.bindVertexArray(n), [
      [1, 1, -1, 1, -1, -1, 1, -1],
      [1, 1, 0, 1, 0, 0, 1, 0]
    ].forEach((a, s) => {
      const c = e.createBuffer() ?? u();
      e.bindBuffer(e.ARRAY_BUFFER, c), e.bufferData(e.ARRAY_BUFFER, new Float32Array(a), e.STATIC_DRAW), e.vertexAttribPointer(s, 2, e.FLOAT, !1, 0, 0), e.enableVertexAttribArray(s);
    });
    const o = e.createBuffer() ?? u();
    e.bindBuffer(e.ELEMENT_ARRAY_BUFFER, o), e.bufferData(e.ELEMENT_ARRAY_BUFFER, i, e.STATIC_DRAW), e.bindVertexArray(null);
  }
  draw(e, t, n, i, o, a, s) {
    const { drawInfoBuffer: c, gl: f, program: l, sampler: h, vertexArray: m } = this, _ = new Float32Array([
      t,
      n,
      i,
      o,
      s[0],
      s[1],
      a[0],
      a[1]
    ]);
    f.useProgram(l), f.bindVertexArray(m);
    try {
      f.bindBuffer(f.UNIFORM_BUFFER, c), f.bufferData(f.UNIFORM_BUFFER, _, f.STREAM_DRAW), f.bindBufferBase(f.UNIFORM_BUFFER, U, c), f.activeTexture(f.TEXTURE0), f.bindTexture(f.TEXTURE_2D, e), f.uniform1i(h, 0), f.drawElements(f.TRIANGLES, 6, f.UNSIGNED_SHORT, 0);
    } finally {
      f.bindVertexArray(null);
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
function P(r, e, t) {
  const n = r.createProgram() ?? u(), i = (o, a) => {
    const s = r.createShader(o) ?? u();
    r.shaderSource(s, a), r.compileShader(s), r.getShaderParameter(s, r.COMPILE_STATUS) ?? u(r.getShaderInfoLog(s)), r.attachShader(n, s);
  };
  return i(r.VERTEX_SHADER, e), i(r.FRAGMENT_SHADER, t), r.linkProgram(n), r.getProgramParameter(n, r.LINK_STATUS) ?? u(r.getProgramInfoLog(n)), n;
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
async function J(r) {
  const [e] = await Promise.all([r.wasm ?? K(), D()]);
  e && await j({ ...r, wasm: e });
}
class q {
  constructor(e) {
    const t = this.canvas = e.canvas;
    t.addEventListener("mousemove", (n) => {
      const i = t.getBoundingClientRect();
      this.pointerPos = [n.clientX - i.left, n.clientY - i.top];
    }), this.config = e, this.gl = e.canvas.getContext("webgl2"), this.texturePipeline = new G(this.gl), this.resizeCanvas(), new ResizeObserver(() => this.resizeNeeded = !0).observe(e.canvas);
  }
  #r(e) {
    const t = [], { gl: n } = this, i = n.getProgramParameter(e, n.ACTIVE_ATTRIBUTES);
    for (let o = 0; o < i; o += 1) {
      const a = n.getActiveAttrib(e, o) ?? u(), s = n.getAttribLocation(e, a.name);
      t.push({ count: a.size, loc: s, type: a.type });
    }
    return t.sort((o, a) => o.loc - a.loc), this.#a(t), t;
  }
  bufferNew(e, t) {
    const n = this.memoryView(t, 12), i = b(n, 0), o = b(n, 4), a = this.memoryBytes().subarray(i, i + o), { gl: s } = this;
    s.enable(s.BLEND), s.blendFunc(s.SRC_ALPHA, s.ONE_MINUS_SRC_ALPHA);
    const c = s.createBuffer();
    c || u(), this.buffers.push({
      buffer: c,
      kind: ["vertex", "index"][e]
    });
    const f = [s.ARRAY_BUFFER, s.ELEMENT_ARRAY_BUFFER][e] ?? u();
    s.bindBuffer(f, c);
    const l = i ? s.STATIC_DRAW : s.STREAM_DRAW;
    return s.bufferData(f, a, l), this.buffers.length;
  }
  buffers = [];
  canvas;
  config;
  draw(e, t, n) {
    this.#n();
    const { gl: i } = this;
    i.drawElements(i.TRIANGLES, t, i.UNSIGNED_SHORT, e);
  }
  drawText(e, t, n) {
    e && (e != this.textTextureText && (this.textTexture = this.textDraw(e, this.textTexture || void 0), this.textTextureText = e), this.drawTexture(this.textTexture, t, n));
  }
  drawTexture(e, t, n) {
    const {
      canvas: { clientWidth: i, clientHeight: o },
      gl: a,
      pipeline: s,
      textures: c
    } = this, { size: f, texture: l, usedSize: h } = c[e - 1];
    this.texturePipeline.draw(
      l,
      i,
      o,
      t,
      n,
      f,
      h
    ), s && a.useProgram(s.program);
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
      const t = Date.now(), n = (t - this.frameTimeBegin) * 1e-3, i = e / n;
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
    let { gl: t, pipelines: n } = this;
    const i = this.pipeline = n[e - 1] ?? u();
    t.useProgram(i.program);
  }
  #i() {
    const {
      gl: e,
      pipelines: t,
      shaders: [n]
    } = this;
    if (t.length) return;
    const i = E(n, F.Vertex, "vertex_main"), o = E(
      n,
      F.Fragment,
      "fragment_main"
    ), a = P(e, i, o), s = this.#r(a), c = this.#s(a);
    t.push({ attributes: s, program: a, uniforms: c });
  }
  #n() {
    this.pipeline || (this.#i(), this.passBegun || this.passBegin(), this.pipelines.length == 1 && this.pipelineApply(1));
  }
  pipelines = [];
  pointerPos = [0, 0];
  readBytes(e) {
    const t = this.memoryBytes(), n = new DataView(t.buffer, e, 2 * 4), i = b(n, 0), o = b(n, 4);
    return t.subarray(i, i + o);
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
      for (const n of this.pipelines) {
        t.bindBuffer(t.UNIFORM_BUFFER, this.tacaBuffer);
        const i = new Uint8Array(n.uniforms.tacaSize), o = new DataView(i.buffer);
        o.setFloat32(0, e.width, !0), o.setFloat32(4, e.height, !0), t.bufferSubData(t.UNIFORM_BUFFER, 0, i);
      }
    }
  }
  textDraw(e, t) {
    const { gl: n, offscreen: i, offscreenContext: o, textures: a } = this, s = "30px sans-serif";
    o.font = s;
    const c = o.measureText(e), f = c.width, l = c.fontBoundingBoxAscent + c.fontBoundingBoxDescent;
    i.width < f && (i.width = Math.ceil(f)), i.height < l && (i.height = Math.ceil(l)), o.clearRect(0, 0, i.width, i.height), o.fillStyle = "white", o.font = s, o.textBaseline = "bottom", o.fillText(e, 0, l);
    let h = !t, m;
    if (t) {
      const _ = a[t - 1];
      _.size[0] < i.width || _.size[1] < i.height ? (n.deleteTexture(_.texture), h = !0) : (m = _.texture, _.usedSize = [f, l]);
    }
    if (h) {
      m = n.createTexture() ?? u();
      const _ = {
        size: [i.width, i.height],
        texture: m,
        usedSize: [f, l]
      };
      t || (t = a.length + 1), a[t - 1] = _;
    }
    return n.bindTexture(n.TEXTURE_2D, m), h && (n.texParameteri(n.TEXTURE_2D, n.TEXTURE_WRAP_S, n.CLAMP_TO_EDGE), n.texParameteri(n.TEXTURE_2D, n.TEXTURE_WRAP_T, n.CLAMP_TO_EDGE), n.texParameteri(n.TEXTURE_2D, n.TEXTURE_MIN_FILTER, n.LINEAR), n.texParameteri(n.TEXTURE_2D, n.TEXTURE_MAG_FILTER, n.LINEAR)), n.texImage2D(
      n.TEXTURE_2D,
      0,
      n.RGBA,
      n.RGBA,
      n.UNSIGNED_BYTE,
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
      const { pipeline: i } = this, o = t.createBuffer() ?? u(), { uniforms: a } = i;
      t.bindBuffer(t.UNIFORM_BUFFER, o), t.bufferData(t.UNIFORM_BUFFER, a.size, t.STREAM_DRAW);
      for (let c = 0; c < a.count; c += 1)
        c != a.tacaIndex && t.bindBufferBase(t.UNIFORM_BUFFER, c + 1, o);
      this.uniformsBuffer = o;
      const s = t.createBuffer() ?? u();
      t.bindBuffer(t.UNIFORM_BUFFER, s), t.bufferData(t.UNIFORM_BUFFER, a.tacaSize, t.STREAM_DRAW), t.bindBufferBase(t.UNIFORM_BUFFER, a.tacaIndex + 1, s), this.tacaBuffer = s, this.tacaBufferUpdate();
    }
    const n = this.readBytes(e);
    t.bindBuffer(t.UNIFORM_BUFFER, this.uniformsBuffer), t.bufferSubData(t.UNIFORM_BUFFER, 0, n);
  }
  uniformsBuffer = null;
  #s(e) {
    const { gl: t } = this, n = t.getProgramParameter(e, t.ACTIVE_UNIFORM_BLOCKS);
    let i = 0, o = 0, a = 0;
    for (let s = 0; s < n; s += 1) {
      const c = t.getActiveUniformBlockName(e, s), f = t.getActiveUniformBlockParameter(
        e,
        s,
        t.UNIFORM_BLOCK_DATA_SIZE
      ) ?? u();
      c == "taca_uniform_block" ? (o = s, a = f) : (s > 0 && f != i && u(), i = f), t.uniformBlockBinding(e, s, s + 1);
    }
    return { count: n, size: i, tacaIndex: o, tacaSize: a };
  }
  #a(e) {
    const { buffers: t, gl: n } = this;
    if (t.length == 2) {
      let i = function(c) {
        let f = 0;
        for (const l of e) {
          const { loc: h } = l, [m, _] = {
            [n.FLOAT_VEC2]: [2, n.FLOAT],
            [n.FLOAT_VEC4]: [4, n.FLOAT]
          }[l.type] ?? u(), A = { [n.FLOAT]: 4 }[_] ?? u();
          f = Math.ceil(f / A) * A, c(h, m, _, f), f += m * A;
        }
        return f;
      };
      const o = t.find((c) => c.kind == "vertex") ?? u();
      n.bindBuffer(n.ARRAY_BUFFER, o.buffer);
      const a = i(() => {
      });
      i((c, f, l, h) => {
        n.enableVertexAttribArray(c), n.vertexAttribPointer(c, f, l, !1, a, h);
      });
      const s = t.find((c) => c.kind == "index") ?? u();
      n.bindBuffer(n.ELEMENT_ARRAY_BUFFER, s.buffer);
    }
  }
  vertexBuffer = null;
}
function b(r, e) {
  return r.getUint32(e, !0);
}
async function j(r) {
  const e = r.wasm;
  r.wasm = void 0;
  const t = new Uint8Array(e), n = t[0] == 4 ? (
    // Presume lz4 because wasm starts with 0.
    z(t).buffer
  ) : e;
  let i = new q(r);
  const o = $(i);
  let { instance: a } = await WebAssembly.instantiate(n, { env: o });
  i.init(a);
  const s = i.exports;
  if (s._start(), s.listen) {
    const c = () => {
      try {
        s.listen(0);
      } finally {
        i.frameEnd();
      }
      requestAnimationFrame(c);
    };
    requestAnimationFrame(c);
  }
}
async function K() {
  const r = new URL(window.location.href), t = new URLSearchParams(r.search).get("app");
  if (t)
    return await (await fetch(t)).arrayBuffer();
}
function $(r) {
  return {
    taca_RenderingContext_applyBindings(e) {
    },
    taca_RenderingContext_applyPipeline(e) {
    },
    taca_RenderingContext_applyUniforms(e) {
      r.uniformsApply(e);
    },
    taca_RenderingContext_beginPass() {
    },
    taca_RenderingContext_commitFrame() {
      r.frameCommit();
    },
    taca_RenderingContext_draw(e, t, n) {
      r.draw(e, t, n);
    },
    taca_RenderingContext_drawText(e, t, n) {
      r.drawText(r.readString(e), t, n);
    },
    taca_RenderingContext_drawTexture(e, t, n) {
      r.drawTexture(e, t, n);
    },
    taca_RenderingContext_endPass() {
    },
    taca_RenderingContext_newBuffer(e, t) {
      return r.bufferNew(e, t);
    },
    taca_RenderingContext_newPipeline(e) {
      console.log("taca_RenderingContext_newPipeline");
    },
    taca_RenderingContext_newShader(e) {
      return r.shaders.push(M(r.readBytes(e))), r.shaders.length;
    },
    taca_Window_newRenderingContext() {
      return 1;
    },
    taca_Window_print(e) {
      console.log(r.readString(e));
    },
    taca_Window_setTitle(e) {
      document.title = r.readString(e);
    },
    taca_Window_state(e) {
      const { clientWidth: t, clientHeight: n } = r.canvas, [i, o] = r.pointerPos, a = r.memoryView(e, 4 * 4);
      y(a, 0, i), y(a, 4, o), y(a, 8, t), y(a, 12, n);
    }
  };
}
function y(r, e, t) {
  return r.setFloat32(e, t, !0);
}
const Z = new TextDecoder();
export {
  J as runApp
};
