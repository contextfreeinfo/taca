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
let appMemoryBytes;

/** @type {Uint8Array} */
let engineMemoryBytes;

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
        engineMemoryBytes.set(
          appMemoryBytes.slice(appSrc, appSrc + count),
          engineDest
        );
      },
      sendEvent(kind) {
        appExports?.listen(kind);
      },
    });
  },
});

load("../target/wasm32-unknown-unknown/release/taca.wasm");
// load("taca.wasm");

/**
 * @param {number} platform
 * @param {any} engine
 * @param {WebAssembly.Memory} memory
 * @param {number} bufferPtr
 * @param {number} bufferLen
 */
async function loadApp(platform, engine, memory, bufferPtr, bufferLen) {
  // console.log(`platform: ${platform} ${bufferPtr} ${bufferLen}`);
  engineMemoryBytes = new Uint8Array(memory.buffer);
  const url = new URL(window.location.href);
  const params = new URLSearchParams(url.search);
  const app = params.get("app");
  if (app == null) {
    return;
  }
  const response = await fetch(app);
  const bufferBytes = new Uint8Array(memory.buffer, bufferPtr, bufferLen);
  const { instance } = await WebAssembly.instantiateStreaming(response, {
    env: {
      taca_RenderingContext_applyBindings(context, bindings) {
        bufferBytes.set(appMemoryBytes.slice(bindings, bindings + 3 * 4));
        engine.taca_RenderingContext_applyBindings(platform, context, 0);
      },
      taca_RenderingContext_applyPipeline(context, pipeline) {
        engine.taca_RenderingContext_applyPipeline(platform, context, pipeline);
      },
      taca_RenderingContext_applyUniforms(context, uniforms) {
        bufferBytes.set(appMemoryBytes.slice(uniforms, uniforms + 2 * 4));
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
        bufferBytes.set(appMemoryBytes.slice(info, info + 5 * 4));
        return engine.taca_RenderingContext_newBuffer(
          platform,
          context,
          typ,
          usage,
          0
        );
      },
      taca_RenderingContext_newPipeline(context, bytes) {
        bufferBytes.set(appMemoryBytes.slice(bytes, bytes + 8 * 4));
        return engine.taca_RenderingContext_newPipeline(platform, context, 0);
      },
      taca_RenderingContext_newShader(context, bytes) {
        bufferBytes.set(appMemoryBytes.slice(bytes, bytes + 2 * 4));
        return engine.taca_RenderingContext_newShader(platform, context, 0);
      },
      taca_Window_get() {
        return engine.taca_Window_get(platform);
      },
      taca_Window_newRenderingContext(window) {
        return engine.taca_Window_newRenderingContext(platform, window);
      },
      taca_Window_print(window, text) {
        bufferBytes.set(appMemoryBytes.slice(text, text + 2 * 4));
        engine.taca_Window_print(platform, window, text);
      },
      taca_Window_state(result, window) {
        engine.taca_Window_state(platform, window, 0);
        appMemoryBytes.set(bufferBytes.slice(0, 4 * 4), result);
      },
    },
  });
  appExports = instance.exports;
  appMemoryBytes = new Uint8Array(instance.exports.memory.buffer);
  instance.exports._start();
}
