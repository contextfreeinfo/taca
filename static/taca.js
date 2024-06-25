// @ts-check

/**
 * @param {WebAssembly.Memory} memory
 * @param {number} text
 * @param {number} length
 */
function print(memory, text, length) {
  const chunk = memory.buffer.slice(text, text + length);
  const decoded = new TextDecoder("utf-8").decode(chunk);
  console.log(decoded);
}

/** @type {any} */
let appExports;

/** @type {Uint8Array} */
let appMemory;

/** @type {any} */
let engineExports;

/** @type {WebAssembly.Memory} */
let engineMemory;

miniquad_add_plugin({
  name: "taca",
  version: 0,
  register_plugin(importObject) {
    Object.assign(importObject.env, {
      print(text, length) {
        print(wasm_memory, text, length);
      },
      loadApp(platform, bufferPtr, bufferLen) {
        loadApp(platform, wasm_exports, wasm_memory, bufferPtr, bufferLen);
      },
      readAppMemory(engineDest, appSrc, count) {
        new Uint8Array(engineMemory.buffer).set(
          appMemoryBytes().slice(appSrc, appSrc + count),
          engineDest
        );
      },
      sendEvent(kind) {
        appExports?.listen(kind);
      },
      startApp() {
        appExports._start();
      },
    });
  },
});

load("taca.wasm");

function appMemoryBytes() {
  // TODO Keep this cached and check for validity.
  return new Uint8Array(appMemory.buffer);
}

function engineMemoryBytes() {
  // TODO Keep this cached and check for validity.
  return new Uint8Array(engineMemory.buffer);
}

/**
 * @param {number} platform
 * @param {any} engine
 * @param {WebAssembly.Memory} memory
 * @param {number} bufferPtr
 * @param {number} bufferLen
 */
async function loadApp(platform, engine, memory, bufferPtr, bufferLen) {
  // console.log(`platform: ${platform} ${bufferPtr} ${bufferLen}`);
  engineExports = wasm_exports;
  engineMemory = memory;
  const url = new URL(window.location.href);
  const params = new URLSearchParams(url.search);
  const app = params.get("app");
  if (app == null) {
    return;
  }
  const bufferBytes = () => new Uint8Array(memory.buffer, bufferPtr, bufferLen);
  // Prepare env.
  const env = {
    taca_RenderingContext_applyBindings(context, bindings) {
      bufferBytes().set(appMemoryBytes().slice(bindings, bindings + 3 * 4));
      engine.taca_RenderingContext_applyBindings(platform, context, 0);
    },
    taca_RenderingContext_applyPipeline(context, pipeline) {
      engine.taca_RenderingContext_applyPipeline(platform, context, pipeline);
    },
    taca_RenderingContext_applyUniforms(context, uniforms) {
      bufferBytes().set(appMemoryBytes().slice(uniforms, uniforms + 2 * 4));
      return engine.taca_RenderingContext_applyUniforms(platform, context, 0);
    },
    taca_RenderingContext_beginPass(context) {
      engine.taca_RenderingContext_beginPass(platform, context);
    },
    taca_RenderingContext_commitFrame(context) {
      engine.taca_RenderingContext_commitFrame(platform, context);
    },
    taca_RenderingContext_draw(
      context,
      item_begin,
      item_count,
      instance_count
    ) {
      engine.taca_RenderingContext_draw(
        platform,
        context,
        item_begin,
        item_count,
        instance_count
      );
    },
    taca_RenderingContext_endPass(context) {
      engine.taca_RenderingContext_endPass(platform, context);
    },
    taca_RenderingContext_newBuffer(context, typ, usage, info) {
      bufferBytes().set(appMemoryBytes().slice(info, info + 5 * 4));
      return engine.taca_RenderingContext_newBuffer(
        platform,
        context,
        typ,
        usage,
        0
      );
    },
    taca_RenderingContext_newPipeline(context, bytes) {
      bufferBytes().set(appMemoryBytes().slice(bytes, bytes + 8 * 4));
      return engine.taca_RenderingContext_newPipeline(platform, context, 0);
    },
    taca_RenderingContext_newShader(context, bytes) {
      bufferBytes().set(appMemoryBytes().slice(bytes, bytes + 2 * 4));
      return engine.taca_RenderingContext_newShader(platform, context, 0);
    },
    taca_Window_newRenderingContext() {
      return engine.taca_Window_newRenderingContext(platform);
    },
    taca_Window_print(text) {
      bufferBytes().set(appMemoryBytes().slice(text, text + 2 * 4));
      engine.taca_Window_print(platform, text);
    },
    taca_Window_setTitle(title) {
      const titleSlice = appMemoryBytes().slice(title, title + 2 * 4);
      // Also set manually here because miniquad doesn't seem to.
      // TODO Factor out this logic? Seems so painful.
      const titleView = new DataView(
        titleSlice.buffer,
        titleSlice.byteOffset,
        titleSlice.byteLength
      );
      const titlePtr = titleView.getUint32(0, true);
      const titleLen = titleView.getUint32(4, true);
      const chunk = appMemoryBytes().slice(titlePtr, titlePtr + titleLen);
      const decoded = new TextDecoder("utf-8").decode(chunk);
      document.title = decoded;
      // Also let the app know.
      engine.taca_Window_setTitle(platform, titlePtr, titleLen);
    },
    taca_Window_state(result) {
      engine.taca_Window_state(platform, 0);
      appMemoryBytes().set(bufferBytes().slice(0, 4 * 4), result);
    },
  };
  // Read wasm data in advance because of custom handling or mime type issues.
  let mustFree = null;
  let appContent = await (await fetch(app)).arrayBuffer();
  if (new DataView(appContent).getUint32(0, false) == 0x04224d18) {
    // LZ4 compressed.
    const seeInfo = (info) => {
      const view = new DataView(engineMemory.buffer, info, 3 * 4);
      return {
        ptr: view.getUint32(0, true),
        len: view.getUint32(4, true),
        capacity: view.getUint32(8, true),
      };
    };
    const engineBytes = seeInfo(
        // TODO platform
        engineExports.taca_alloc(appContent.byteLength)
    );
    try {
      const engineBytesArray = new Uint8Array(
        engineMemory.buffer,
        engineBytes.ptr,
        appContent.byteLength
      );
      engineBytesArray.set(new Uint8Array(appContent));
      mustFree = seeInfo(
        // TODO platform
        engineExports.taca_decompress(engineBytes.ptr, engineBytes.len)
      );
      appContent = new ArrayBuffer(mustFree.len);
      new Uint8Array(appContent).set(
        new Uint8Array(engineMemory.buffer, mustFree.ptr, appContent.byteLength)
      );
    } finally {
      engineExports.taca_free(
        engineBytes.ptr,
        engineBytes.len,
        engineBytes.capacity
      );
    }
  }
  /** @type {WebAssembly.Instance} */
  let instance;
  try {
    instance = (await WebAssembly.instantiate(appContent, { env })).instance;
  } finally {
    if (mustFree) {
      engineExports.taca_free(mustFree.ptr, mustFree.len, mustFree.capacity);
    }
  }
  appExports = instance.exports;
  appMemory = appExports.memory;
  if (appExports.config) {
    appExports.config();
  }
  engine.taca_start(platform);
}
