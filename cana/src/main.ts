// import "./style.css";
// import logo from "../../taca-logo.svg";
// document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
//   <div>
//     <a href="https://vitejs.dev" target="_blank">
//       <img src="${logo}" class="logo" alt="Vite logo" />
//     </a>
//   </div>
// `;

async function main() {
  await loadApp();
}

/** Allows for additional custom properties. */
class App {
  config: (() => void) | undefined;

  init(instance: WebAssembly.Instance) {
    Object.assign(this, instance.exports as any);
  }

  listen: ((event: number) => void) | undefined;

  memory!: WebAssembly.Memory;

  #memoryBuffer: ArrayBuffer | undefined;
  #memoryBufferBytes: Uint8Array | undefined;

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

  _start!: () => void;
}

async function loadApp() {
  const appData = await loadAppData();
  if (!appData) return;
  let app = new App();
  const env = makeAppEnv(app);
  let { instance } = await WebAssembly.instantiate(appData, { env });
  app.init(instance);
  // TODO Fold config into start once we get fully off miniquad.
  if (app.config) {
    app.config();
  }
  app._start();
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
    ) {},
    taca_RenderingContext_newPipeline(context: number, bytes: number) {},
    taca_RenderingContext_newShader(context: number, bytes: number) {},
    taca_Window_newRenderingContext() {},
    taca_Window_print(text: number) {},
    taca_Window_setTitle(title: number) {
      // TODO Abstract to provide callbacks for these things?
      document.title = app.readString(title);
    },
    taca_Window_state(result: number) {},
  };
}

const textDecoder = new TextDecoder();

main();
