import {
  default as cana,
  lz4Decompress,
  type Shader,
  shaderNew,
  shaderToGlsl,
  ShaderStage,
} from "../pkg/cana";

export interface AppConfig {
  canvas: HTMLCanvasElement;
  wasm?: ArrayBuffer | Promise<ArrayBuffer>;
}

export async function runApp(config: AppConfig) {
  const [appData] = await Promise.all([config.wasm ?? loadAppData(), cana()]);
  if (appData) {
    await loadApp({ ...config, wasm: appData });
  }
}

/** Allows for additional custom properties. */
class App {
  constructor(config: AppConfig) {
    const canvas = (this.canvas = config.canvas);
    canvas.addEventListener("mousemove", (event) => {
      const rect = canvas.getBoundingClientRect();
      this.pointerPos = [
        event.clientX - rect.left,
        event.clientY - rect.height,
      ];
    });
    this.config = config;
    this.gl = config.canvas.getContext("webgl2")!;
    // Resize will fail if we couldn't get a context.
    this.resizeCanvas();
    // TODO Track for deregistration needs?
    new ResizeObserver(() => this.resizeCanvas()).observe(config.canvas);
  }

  bufferNew(type: number, usage: number, info: number) {
    const infoBytes = this.memoryView(info, 3 * 4);
    const ptr = getU32(infoBytes, 0);
    const size = getU32(infoBytes, 4);
    const itemSize = getU32(infoBytes, 8);
    const data = this.memoryBytes().subarray(ptr, ptr + size);
    const gl = this.gl;
    const buffer = gl.createBuffer();
    buffer || fail();
    this.buffers.push({
      buffer: buffer!,
      itemSize,
      kind: ["vertex", "index"][type] as "vertex" | "index",
    });
    const target = [gl.ARRAY_BUFFER, gl.ELEMENT_ARRAY_BUFFER][type] ?? fail();
    gl.bindBuffer(target, buffer);
    const usageValue =
      [gl.STATIC_DRAW, gl.DYNAMIC_DRAW, gl.STREAM_DRAW][usage] ?? fail();
    gl.bufferData(target, data, usageValue);
    return this.buffers.length;
  }

  buffers: Buffer[] = [];

  canvas: HTMLCanvasElement;

  config: AppConfig;

  draw(itemBegin: number, itemCount: number, instanceCount: number) {
    this.#pipelinedEnsure();
  }

  exports: AppExports = undefined as any;

  frameCommit() {
    this.passBegun = false;
    this.pipeline = null;
  }

  gl: WebGL2RenderingContext;

  init(instance: WebAssembly.Instance) {
    this.exports = instance.exports as any;
    this.memory = instance.exports.memory as any;
  }

  memory: WebAssembly.Memory = undefined as any;

  #memoryBuffer: ArrayBuffer | null = null;
  #memoryBufferBytes: Uint8Array | null = null;

  memoryBytes() {
    if (this.#memoryBuffer != this.memory.buffer) {
      // Either on first access or on internal reallocation.
      this.#memoryBuffer = this.memory.buffer;
      this.#memoryBufferBytes = new Uint8Array(this.#memoryBuffer);
    }
    return this.#memoryBufferBytes!;
  }

  memoryView(ptr: number, len: number) {
    return new DataView(this.memory.buffer, ptr, len);
  }

  passBegin() {
    let { gl } = this;
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
  }

  passBegun = false;

  pipeline: Pipeline | null = null;

  pipelineApply(pipelinePtr: number) {
    let { gl, pipelines } = this;
    const pipeline = (this.pipeline = pipelines[pipelinePtr - 1] ?? fail());
    gl.useProgram(pipeline.program);
  }

  #pipelineEnsure() {
    const {
      gl,
      pipelines,
      shaders: [shader],
    } = this;
    if (pipelines.length) return;
    const vertex = shaderToGlsl(shader, ShaderStage.Vertex, "vs_main");
    const fragment = shaderToGlsl(shader, ShaderStage.Fragment, "fs_main");
    const program = gl.createProgram() ?? fail();
    const addShader = (type: number, source: string) => {
      const shader = gl.createShader(type) ?? fail();
      gl.shaderSource(shader, source);
      gl.compileShader(shader);
      gl.getShaderParameter(shader, gl.COMPILE_STATUS) ??
        fail(gl.getShaderInfoLog(shader));
      gl.attachShader(program, shader);
    };
    addShader(gl.VERTEX_SHADER, vertex);
    addShader(gl.FRAGMENT_SHADER, fragment);
    gl.linkProgram(program);
    gl.getProgramParameter(program, gl.LINK_STATUS) ??
      fail(gl.getProgramInfoLog(program));
    console.log(vertex);
    console.log(fragment);
    const attributes: Attribute[] = [];
    {
      const attribCount = gl.getProgramParameter(program, gl.ACTIVE_ATTRIBUTES);
      for (let i = 0; i < attribCount; i += 1) {
        const info = gl.getActiveAttrib(program, i) ?? fail();
        const loc = gl.getAttribLocation(program, info.name);
        // const type = {
        //   [gl.FLOAT_VEC2]: "vec2f",
        //   [gl.FLOAT_VEC4]: "vec4f",
        // }[info.type];
        attributes.push({ count: info.size, loc, type: info.type });
      }
      attributes.sort((a, b) => a.loc - b.loc);
      // TODO Change taca api to work around vertex arrays.
      this.#vertexArrayCreate(attributes);
      // console.log(attributes);
    }
    pipelines.push({ attributes, program });
  }

  #pipelinedEnsure() {
    if (!this.pipeline) {
      this.#pipelineEnsure();
      if (!this.passBegun) this.passBegin();
      if (this.pipelines.length == 1) this.pipelineApply(1);
    }
  }

  pipelines: Pipeline[] = [];

  pointerPos: [x: number, y: number] = [0, 0];

  readBytes(spanPtr: number) {
    // Can cache memory bytes when no app calls are being made.
    const memoryBytes = this.memoryBytes();
    const spanView = new DataView(memoryBytes.buffer, spanPtr, 2 * 4);
    // Wasm is explicitly little-endian.
    const contentPtr = getU32(spanView, 0);
    const contentLen = getU32(spanView, 4);
    return memoryBytes.subarray(contentPtr, contentPtr + contentLen);
  }

  readString(spanPtr: number) {
    return textDecoder.decode(this.readBytes(spanPtr));
  }

  resizeCanvas() {
    const canvas = this.config.canvas;
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
    this.gl.viewport(0, 0, canvas.width, canvas.height);
  }

  shaders: Shader[] = [];

  uniformsApply(uniforms: number) {
    // throw new Error("Method not implemented.");
  }

  #vertexArrayCreate(attributes: Attribute[]) {
    const { buffers, gl } = this;
    if (buffers.length == 2) {
      const vao = gl.createVertexArray();
      gl.bindVertexArray(vao);
      // attributes.find((attr) => attr.)
      const vertex =
        buffers.find((buffer) => buffer.kind == "vertex") ?? fail();
      gl.bindBuffer(gl.ARRAY_BUFFER, vertex.buffer);
      let offset = 0;
      for (const attr of attributes) {
        const { loc } = attr;
        gl.enableVertexAttribArray(loc);
        const [size, type] =
          {
            [gl.FLOAT_VEC2]: [2, gl.FLOAT],
            [gl.FLOAT_VEC4]: [4, gl.FLOAT],
          }[attr.type] ?? fail();
        // Pad for alignment.
        offset = Math.ceil(offset / size) * size;
        console.log(size, vertex.itemSize, offset);
        // TODO Item size vs alignment seems very off.
        gl.vertexAttribPointer(loc, size, type, false, vertex.itemSize, offset);
        offset += size;
      }
    }
  }
}

interface AppExports {
  config: (() => void) | undefined;
  listen: ((event: number) => void) | undefined;
  _start: () => void;
}

interface Attribute {
  count: number;
  loc: number;
  type: number;
}

interface Buffer {
  buffer: WebGLBuffer;
  itemSize: number;
  kind: "index" | "vertex";
}

function fail(message?: string | null): never {
  throw Error(message ?? undefined);
}

function getU32(view: DataView, byteOffset: number) {
  return view.getUint32(byteOffset, true);
}

async function loadApp(config: AppConfig) {
  const appData = config.wasm as ArrayBuffer;
  config.wasm = undefined;
  const appBytes = new Uint8Array(appData);
  const actualData =
    appBytes[0] == 4
      ? // Presume lz4 because wasm starts with 0.
        lz4Decompress(appBytes).buffer
      : appData;
  let app = new App(config);
  const env = makeAppEnv(app);
  let { instance } = await WebAssembly.instantiate(actualData, { env });
  app.init(instance);
  // TODO Fold config into start once we get fully off miniquad.
  const exports = app.exports;
  if (exports.config) {
    exports.config();
  }
  exports._start();
  if (exports.listen) {
    const update = () => {
      exports.listen!(0);
      requestAnimationFrame(update);
    };
    requestAnimationFrame(update);
  }
}

async function loadAppData() {
  const url = new URL(window.location.href);
  const params = new URLSearchParams(url.search);
  const app = params.get("app");
  if (app) {
    return await (await fetch(app)).arrayBuffer();
  }
}

function makeAppEnv(app: App) {
  return {
    taca_RenderingContext_applyBindings(context: number, bindings: number) {},
    taca_RenderingContext_applyPipeline(context: number, pipeline: number) {},
    taca_RenderingContext_applyUniforms(context: number, uniforms: number) {
      app.uniformsApply(uniforms);
    },
    taca_RenderingContext_beginPass(context: number) {},
    taca_RenderingContext_commitFrame(context: number) {
      app.frameCommit();
    },
    taca_RenderingContext_draw(
      context: number,
      itemBegin: number,
      itemCount: number,
      instanceCount: number
    ) {
      app.draw(itemBegin, itemCount, instanceCount);
    },
    taca_RenderingContext_endPass(context: number) {},
    taca_RenderingContext_newBuffer(
      context: number,
      type: number,
      usage: number,
      info: number
    ) {
      return app.bufferNew(type, usage, info);
    },
    taca_RenderingContext_newPipeline(context: number, bytes: number) {
      console.log("taca_RenderingContext_newPipeline");
    },
    taca_RenderingContext_newShader(context: number, bytes: number) {
      app.shaders.push(shaderNew(app.readBytes(bytes)));
      return app.shaders.length;
    },
    taca_Window_newRenderingContext() {
      // TODO If we only have one, we don't need it at all, right?
      return 1;
    },
    taca_Window_print(text: number) {
      console.log(app.readString(text));
    },
    taca_Window_setTitle(title: number) {
      // TODO Abstract to provide callbacks for these things?
      document.title = app.readString(title);
    },
    taca_Window_state(result: number) {
      const { clientWidth, clientHeight } = app.canvas;
      const [pointerX, pointerY] = app.pointerPos;
      const view = app.memoryView(result, 4 * 4);
      setU32(view, 0, pointerX);
      setU32(view, 4, pointerY);
      setU32(view, 8, clientWidth);
      setU32(view, 12, clientHeight);
    },
  };
}

interface Pipeline {
  attributes: Attribute[];
  program: WebGLProgram;
}

const textDecoder = new TextDecoder();

function setU32(view: DataView, byteOffset: number, value: number) {
  return view.setUint32(byteOffset, value, true);
}
