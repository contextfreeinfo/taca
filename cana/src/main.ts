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

  buffers: WebGLBuffer[] = [];

  canvas: HTMLCanvasElement;

  config: AppConfig;

  ensurePipeline() {
    if (this.pipelines.length) {
      return;
    }
    const vertex = shaderToGlsl(this.shaders[0], ShaderStage.Vertex, "vs_main");
    const fragment = shaderToGlsl(
      this.shaders[0],
      ShaderStage.Fragment,
      "fs_main"
    );
    console.log(vertex);
    console.log(fragment);
    this.pipelines.push(1);
  }

  exports: AppExports = undefined as any;

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

  newBuffer(type: number, usage: number, info: number) {
    const infoBytes = this.memoryView(info, 3 * 4);
    const ptr = getU32(infoBytes, 0);
    const size = getU32(infoBytes, 4);
    // TODO Store for later?: const itemSize = viewU32(infoBytes, 8);
    const data = this.memoryBytes().subarray(ptr, ptr + size);
    const gl = this.gl;
    const buffer = gl.createBuffer();
    buffer || fail();
    this.buffers.push(buffer!);
    const target = [gl.ARRAY_BUFFER, gl.ELEMENT_ARRAY_BUFFER][type] ?? fail();
    gl.bindBuffer(target, buffer);
    const usageValue =
      [gl.STATIC_DRAW, gl.DYNAMIC_DRAW, gl.STREAM_DRAW][usage] ?? fail();
    gl.bufferData(target, data, usageValue);
    return this.buffers.length;
  }

  pipelines: number[] = [];

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
}

interface AppExports {
  config: (() => void) | undefined;
  listen: ((event: number) => void) | undefined;
  _start: () => void;
}

function fail(): never {
  throw Error();
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
      // TODO
    },
    taca_RenderingContext_beginPass(context: number) {},
    taca_RenderingContext_commitFrame(context: number) {},
    taca_RenderingContext_draw(
      context: number,
      item_begin: number,
      item_count: number,
      instance_count: number
    ) {
      app.ensurePipeline();
    },
    taca_RenderingContext_endPass(context: number) {},
    taca_RenderingContext_newBuffer(
      context: number,
      type: number,
      usage: number,
      info: number
    ) {
      return app.newBuffer(type, usage, info);
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

const textDecoder = new TextDecoder();

function setU32(view: DataView, byteOffset: number, value: number) {
  return view.setUint32(byteOffset, value, true);
}
