let f;
const v = typeof TextDecoder < "u" ? new TextDecoder("utf-8", { ignoreBOM: !0, fatal: !0 }) : { decode: () => {
  throw Error("TextDecoder not available");
} };
typeof TextDecoder < "u" && v.decode();
let y = null;
function m() {
  return (y === null || y.byteLength === 0) && (y = new Uint8Array(f.memory.buffer)), y;
}
function U(n, e) {
  return n = n >>> 0, v.decode(m().subarray(n, n + e));
}
let g = 0;
function S(n, e) {
  const t = e(n.length * 1, 1) >>> 0;
  return m().set(n, t / 1), g = n.length, t;
}
let w = null;
function x() {
  return (w === null || w.byteLength === 0) && (w = new Int32Array(f.memory.buffer)), w;
}
function I(n, e) {
  return n = n >>> 0, m().subarray(n / 1, n / 1 + e);
}
function D(n) {
  try {
    const i = f.__wbindgen_add_to_stack_pointer(-16), o = S(n, f.__wbindgen_export_0), s = g;
    f.lz4Decompress(i, o, s);
    var e = x()[i / 4 + 0], t = x()[i / 4 + 1], r = I(e, t).slice();
    return f.__wbindgen_export_1(e, t * 1, 1), r;
  } finally {
    f.__wbindgen_add_to_stack_pointer(16);
  }
}
function N(n) {
  const e = S(n, f.__wbindgen_export_0), t = g, r = f.shaderNew(e, t);
  return B.__wrap(r);
}
function M(n, e) {
  if (!(n instanceof e))
    throw new Error(`expected instance of ${e.name}`);
  return n.ptr;
}
const A = typeof TextEncoder < "u" ? new TextEncoder("utf-8") : { encode: () => {
  throw Error("TextEncoder not available");
} }, O = typeof A.encodeInto == "function" ? function(n, e) {
  return A.encodeInto(n, e);
} : function(n, e) {
  const t = A.encode(n);
  return e.set(t), {
    read: n.length,
    written: t.length
  };
};
function z(n, e, t) {
  if (t === void 0) {
    const a = A.encode(n), c = e(a.length, 1) >>> 0;
    return m().subarray(c, c + a.length).set(a), g = a.length, c;
  }
  let r = n.length, i = e(r, 1) >>> 0;
  const o = m();
  let s = 0;
  for (; s < r; s++) {
    const a = n.charCodeAt(s);
    if (a > 127) break;
    o[i + s] = a;
  }
  if (s !== r) {
    s !== 0 && (n = n.slice(s)), i = t(i, r, r = s + n.length * 3, 1) >>> 0;
    const a = m().subarray(i + s, i + r), c = O(n, a);
    s += c.written, i = t(i, r, s, 1) >>> 0;
  }
  return g = s, i;
}
function R(n, e, t) {
  let r, i;
  try {
    const a = f.__wbindgen_add_to_stack_pointer(-16);
    M(n, B);
    const c = z(t, f.__wbindgen_export_0, f.__wbindgen_export_2), u = g;
    f.shaderToGlsl(a, n.__wbg_ptr, e, c, u);
    var o = x()[a / 4 + 0], s = x()[a / 4 + 1];
    return r = o, i = s, U(o, s);
  } finally {
    f.__wbindgen_add_to_stack_pointer(16), f.__wbindgen_export_1(r, i, 1);
  }
}
const F = Object.freeze({ Vertex: 0, 0: "Vertex", Fragment: 1, 1: "Fragment" }), E = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((n) => f.__wbg_shader_free(n >>> 0));
class B {
  static __wrap(e) {
    e = e >>> 0;
    const t = Object.create(B.prototype);
    return t.__wbg_ptr = e, E.register(t, t.__wbg_ptr, t), t;
  }
  __destroy_into_raw() {
    const e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, E.unregister(this), e;
  }
  free() {
    const e = this.__destroy_into_raw();
    f.__wbg_shader_free(e);
  }
}
async function L(n, e) {
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
function P() {
  const n = {};
  return n.wbg = {}, n.wbg.__wbindgen_throw = function(e, t) {
    throw new Error(U(e, t));
  }, n;
}
function W(n, e) {
  return f = n.exports, T.__wbindgen_wasm_module = e, w = null, y = null, f;
}
async function T(n) {
  if (f !== void 0) return f;
  typeof n > "u" && (n = new URL("taca.wasm", import.meta.url));
  const e = P();
  (typeof n == "string" || typeof Request == "function" && n instanceof Request || typeof URL == "function" && n instanceof URL) && (n = fetch(n));
  const { instance: t, module: r } = await L(await n, e);
  return W(t, r);
}
async function q(n) {
  const [e] = await Promise.all([n.wasm ?? Y(), T()]);
  e && await k({ ...n, wasm: e });
}
class V {
  constructor(e) {
    const t = this.canvas = e.canvas;
    t.addEventListener("mousemove", (r) => {
      const i = t.getBoundingClientRect();
      this.pointerPos = [r.clientX - i.left, r.clientY - i.top];
    }), this.config = e, this.gl = e.canvas.getContext("webgl2"), this.resizeCanvas(), new ResizeObserver(() => this.resizeNeeded = !0).observe(e.canvas);
  }
  #r(e) {
    const t = [], { gl: r } = this, i = r.getProgramParameter(e, r.ACTIVE_ATTRIBUTES);
    for (let o = 0; o < i; o += 1) {
      const s = r.getActiveAttrib(e, o) ?? l(), a = r.getAttribLocation(e, s.name);
      t.push({ count: s.size, loc: a, type: s.type });
    }
    return t.sort((o, s) => o.loc - s.loc), this.#a(t), t;
  }
  bufferNew(e, t, r) {
    const i = this.memoryView(r, 12), o = p(i, 0), s = p(i, 4), a = p(i, 8), c = this.memoryBytes().subarray(o, o + s), { gl: u } = this, _ = u.createBuffer();
    _ || l(), this.buffers.push({
      buffer: _,
      itemSize: a,
      kind: ["vertex", "index"][e]
    });
    const h = [u.ARRAY_BUFFER, u.ELEMENT_ARRAY_BUFFER][e] ?? l();
    u.bindBuffer(h, _);
    const d = [u.STATIC_DRAW, u.DYNAMIC_DRAW, u.STREAM_DRAW][t] ?? l();
    return u.bufferData(h, c, d), this.buffers.length;
  }
  buffers = [];
  canvas;
  config;
  draw(e, t, r) {
    this.#n();
    const { gl: i } = this;
    this.vertexArray || (this.vertexArray = this.vertexArrays[0] ?? l(), i.bindVertexArray(this.vertexArray)), i.drawElements(i.TRIANGLES, t, i.UNSIGNED_SHORT, e);
  }
  exports = void 0;
  frameCommit() {
    this.passBegun = !1, this.pipeline = this.vertexArray = null;
  }
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
  passBegin() {
    let { gl: e, resizeNeeded: t } = this;
    t && this.resizeCanvas(), e.clearColor(0, 0, 0, 1), e.clear(e.COLOR_BUFFER_BIT | e.DEPTH_BUFFER_BIT);
  }
  passBegun = !1;
  pipeline = null;
  pipelineApply(e) {
    let { gl: t, pipelines: r } = this;
    const i = this.pipeline = r[e - 1] ?? l();
    t.useProgram(i.program);
  }
  #i() {
    const {
      gl: e,
      pipelines: t,
      shaders: [r]
    } = this;
    if (t.length) return;
    const i = R(r, F.Vertex, "vs_main"), o = R(r, F.Fragment, "fs_main"), s = e.createProgram() ?? l(), a = (_, h) => {
      const d = e.createShader(_) ?? l();
      e.shaderSource(d, h), e.compileShader(d), e.getShaderParameter(d, e.COMPILE_STATUS) ?? l(e.getShaderInfoLog(d)), e.attachShader(s, d);
    };
    a(e.VERTEX_SHADER, i), a(e.FRAGMENT_SHADER, o), e.linkProgram(s), e.getProgramParameter(s, e.LINK_STATUS) ?? l(e.getProgramInfoLog(s));
    const c = this.#r(s), u = this.#s(s);
    t.push({ attributes: c, program: s, uniforms: u });
  }
  #n() {
    this.pipeline || (this.#i(), this.passBegun || this.passBegin(), this.pipelines.length == 1 && this.pipelineApply(1));
  }
  pipelines = [];
  pointerPos = [0, 0];
  readBytes(e) {
    const t = this.memoryBytes(), r = new DataView(t.buffer, e, 2 * 4), i = p(r, 0), o = p(r, 4);
    return t.subarray(i, i + o);
  }
  readString(e) {
    return G.decode(this.readBytes(e));
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
  uniformsApply(e) {
    this.#n();
    const { gl: t } = this;
    if (!this.uniformsBuffer) {
      const { pipeline: i } = this, o = t.createBuffer() ?? l(), { uniforms: s } = i;
      t.bindBuffer(t.UNIFORM_BUFFER, o), t.bufferData(t.UNIFORM_BUFFER, s.size, t.DYNAMIC_DRAW);
      for (let c = 0; c < s.count; c += 1)
        c != s.tacaIndex && t.bindBufferBase(t.UNIFORM_BUFFER, c, o);
      this.uniformsBuffer = o;
      const a = t.createBuffer() ?? l();
      t.bindBuffer(t.UNIFORM_BUFFER, a), t.bufferData(t.UNIFORM_BUFFER, s.tacaSize, t.DYNAMIC_DRAW), t.bindBufferBase(t.UNIFORM_BUFFER, s.tacaIndex, a), this.tacaBuffer = a, this.tacaBufferUpdate();
    }
    const r = this.readBytes(e);
    t.bindBuffer(t.UNIFORM_BUFFER, this.uniformsBuffer), t.bufferSubData(t.UNIFORM_BUFFER, 0, r);
  }
  uniformsBuffer = null;
  #s(e) {
    const { gl: t } = this, r = t.getProgramParameter(e, t.ACTIVE_UNIFORM_BLOCKS);
    let i = 0, o = 0, s = 0;
    for (let a = 0; a < r; a += 1) {
      const c = t.getActiveUniformBlockName(e, a), u = t.getActiveUniformBlockParameter(
        e,
        a,
        t.UNIFORM_BLOCK_DATA_SIZE
      ) ?? l();
      c == "taca_uniform_block" ? (o = a, s = u) : (a > 0 && u != i && l(), i = u), t.uniformBlockBinding(e, a, a);
    }
    return { count: r, size: i, tacaIndex: o, tacaSize: s };
  }
  vertexArray = null;
  #a(e) {
    const { buffers: t, gl: r } = this;
    if (t.length == 2) {
      const i = r.createVertexArray() ?? l();
      r.bindVertexArray(i);
      try {
        const o = t.find((c) => c.kind == "vertex") ?? l();
        r.bindBuffer(r.ARRAY_BUFFER, o.buffer);
        let s = 0;
        for (const c of e) {
          const { loc: u } = c;
          r.enableVertexAttribArray(u);
          const [_, h] = {
            [r.FLOAT_VEC2]: [2, r.FLOAT],
            [r.FLOAT_VEC4]: [4, r.FLOAT]
          }[c.type] ?? l(), d = { [r.FLOAT]: 4 }[h] ?? l();
          s = Math.ceil(s / d) * d;
          let { itemSize: C } = o;
          r.vertexAttribPointer(u, _, h, !1, C, s), s += _ * d;
        }
        const a = t.find((c) => c.kind == "index") ?? l();
        r.bindBuffer(r.ELEMENT_ARRAY_BUFFER, a.buffer);
      } finally {
        r.bindVertexArray(null);
      }
      this.vertexArrays.push(i);
    }
  }
  vertexArrays = [];
}
function l(n) {
  throw Error(n ?? void 0);
}
function p(n, e) {
  return n.getUint32(e, !0);
}
async function k(n) {
  const e = n.wasm;
  n.wasm = void 0;
  const t = new Uint8Array(e), r = t[0] == 4 ? (
    // Presume lz4 because wasm starts with 0.
    D(t).buffer
  ) : e;
  let i = new V(n);
  const o = H(i);
  let { instance: s } = await WebAssembly.instantiate(r, { env: o });
  i.init(s);
  const a = i.exports;
  if (a.config && a.config(), a._start(), a.listen) {
    const c = () => {
      a.listen(0), requestAnimationFrame(c);
    };
    requestAnimationFrame(c);
  }
}
async function Y() {
  const n = new URL(window.location.href), t = new URLSearchParams(n.search).get("app");
  if (t)
    return await (await fetch(t)).arrayBuffer();
}
function H(n) {
  return {
    taca_RenderingContext_applyBindings(e, t) {
    },
    taca_RenderingContext_applyPipeline(e, t) {
    },
    taca_RenderingContext_applyUniforms(e, t) {
      n.uniformsApply(t);
    },
    taca_RenderingContext_beginPass(e) {
    },
    taca_RenderingContext_commitFrame(e) {
      n.frameCommit();
    },
    taca_RenderingContext_draw(e, t, r, i) {
      n.draw(t, r, i);
    },
    taca_RenderingContext_endPass(e) {
    },
    taca_RenderingContext_newBuffer(e, t, r, i) {
      return n.bufferNew(t, r, i);
    },
    taca_RenderingContext_newPipeline(e, t) {
      console.log("taca_RenderingContext_newPipeline");
    },
    taca_RenderingContext_newShader(e, t) {
      return n.shaders.push(N(n.readBytes(t))), n.shaders.length;
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
const G = new TextDecoder();
export {
  q as runApp
};
