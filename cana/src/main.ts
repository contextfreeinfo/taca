// import "./style.css";
// import logo from "../../taca-logo.svg";
// document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
//   <div>
//     <a href="https://vitejs.dev" target="_blank">
//       <img src="${logo}" class="logo" alt="Vite logo" />
//     </a>
//   </div>
// `;
import { default as cana, lz4Decompress } from "../pkg/cana";

export interface AppConfig {
  canvas: HTMLCanvasElement;
  data?: ArrayBuffer | Promise<ArrayBuffer>;
}

export async function runApp(config: AppConfig) {
  const [appData] = await Promise.all([config.data ?? loadAppData(), cana()]);
  if (appData) {
    await loadApp({ ...config, data: appData });
  }
}

/** Allows for additional custom properties. */
class App {
  constructor(config: AppConfig) {
    this.config = config;
    // TODO Track for deregistration needs?
    new ResizeObserver(() => this.resizeCanvas()).observe(config.canvas);
  }

  config: AppConfig;

  context: WebGL2RenderingContext | null = null;

  exports: AppExports | null = null;

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

  readString(spanPtr: number) {
    // Can cache memory bytes when no app calls are being made.
    const memoryBytes = this.memoryBytes();
    const spanView = new DataView(memoryBytes.buffer, spanPtr, 2 * 4);
    // Wasm is explicitly little-endian.
    const stringPtr = spanView.getUint32(0, true);
    const stringLen = spanView.getUint32(4, true);
    const chunk = memoryBytes.slice(stringPtr, stringPtr + stringLen);
    return textDecoder.decode(chunk);
  }

  resizeCanvas() {
    const canvas = this.config.canvas;
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
    this.context?.viewport(0, 0, canvas.width, canvas.height);
  }
}

interface AppExports {
  config: (() => void) | undefined;
  listen: ((event: number) => void) | undefined;
  _start: () => void;
}

async function loadApp(config: AppConfig) {
  const appData = config.data as ArrayBuffer;
  config.data = undefined;
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
  if (app.exports!.config) {
    app.exports!.config();
  }
  app.exports!._start();
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
    taca_RenderingContext_applyUniforms(context: number, uniforms: number) {},
    taca_RenderingContext_beginPass(context: number) {},
    taca_RenderingContext_commitFrame(context: number) {},
    taca_RenderingContext_draw(
      context: number,
      item_begin: number,
      item_count: number,
      instance_count: number
    ) {},
    taca_RenderingContext_endPass(context: number) {},
    taca_RenderingContext_newBuffer(
      context: number,
      typ: number,
      usage: number,
      info: number
    ) {
      console.log("taca_RenderingContext_newBuffer");
    },
    taca_RenderingContext_newPipeline(context: number, bytes: number) {
      console.log("taca_RenderingContext_newPipeline");
    },
    taca_RenderingContext_newShader(context: number, bytes: number) {
      console.log("taca_RenderingContext_newShader");
    },
    taca_Window_newRenderingContext() {
      // TODO Rename to *get*RenderingContext.
      if (app.context) {
        return 1;
      }
      app.resizeCanvas();
      const context = app.config.canvas.getContext("webgl2");
      if (!context) {
        return 0;
      }
      app.context = context;
      return 1;
    },
    taca_Window_print(text: number) {
      console.log(app.readString(text));
    },
    taca_Window_setTitle(title: number) {
      // TODO Abstract to provide callbacks for these things?
      document.title = app.readString(title);
    },
    taca_Window_state(result: number) {},
  };
}

const textDecoder = new TextDecoder();
