// @ts-check

/**
 * @param {WebAssembly.Memory} memory
 * @param {number} wasm
 * @param {number} length
 */
async function runWasm(memory, wasm, length) {
  const chunk = memory.buffer.slice(wasm, wasm + length);
  const { instance } = await WebAssembly.instantiate(chunk, {
    env: {
      hi() {
        wasm_exports.hi();
      },
    },
  });
  instance.exports._start();
  // Finish initializing taca now that we've initialized the app.
  // wasm_exports.display();
}

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

miniquad_add_plugin({
  name: "taca",
  version: 0,
  register_plugin(importObject) {
    Object.assign(importObject.env, {
      print(text, length) {
        print(wasm_memory, text, length);
      },
      loadApp(platform, bufferPtr, bufferLen) {
        loadApp(platform, wasm_memory, bufferPtr, bufferLen);
      },
    });
  },
});

load("../target/wasm32-unknown-unknown/release/taca.wasm");
// load("../target/wasm32-unknown-unknown/release-lto/taca.wasm");
// load("../target/wasm32-unknown-unknown/release-lto/taca.opt.wasm");

/**
 * @param {number} platform
 * @param {WebAssembly.Memory} memory
 * @param {number} bufferPtr
 * @param {number} bufferLen
 */
async function loadApp(platform, memory, bufferPtr, bufferLen) {
  console.log(`platform: ${platform} ${bufferPtr} ${bufferLen}`);
  const url = new URL(window.location.href);
  const params = new URLSearchParams(url.search);
  const app = params.get("app");
  if (app == null) {
    return;
  }
  const response = await fetch(app);
  const { instance } = await WebAssembly.instantiateStreaming(response, {
    env: {
      taca_RenderingContext_applyBindings(context, bindings) {
        wasm_exports.taca_RenderingContext_applyBindings(
          platform,
          context,
          bindings
        );
      },
      taca_RenderingContext_applyPipeline(context, pipeline) {
        wasm_exports.taca_RenderingContext_applyPipeline(
          platform,
          context,
          pipeline
        );
      },
      taca_RenderingContext_beginPass(context) {
        wasm_exports.taca_RenderingContext_beginPass(platform, context);
      },
      taca_RenderingContext_commitFrame(context) {
        wasm_exports.taca_RenderingContext_commitFrame(platform, context);
      },
      taca_RenderingContext_draw(context) {
        wasm_exports.taca_RenderingContext_draw(platform, context);
      },
      taca_RenderingContext_endPass(context) {
        wasm_exports.taca_RenderingContext_endPass(platform, context);
      },
      taca_RenderingContext_newBuffer(context, typ, usage, info) {
        return wasm_exports.taca_RenderingContext_newBuffer(
          platform,
          context,
          typ,
          usage,
          info
        );
      },
      taca_RenderingContext_newPipeline(context, bytes) {
        return wasm_exports.taca_RenderingContext_newPipeline(
          platform,
          context,
          bytes
        );
      },
      taca_RenderingContext_newShader(context, bytes) {
        return wasm_exports.taca_RenderingContext_newShader(
          platform,
          context,
          bytes
        );
      },
      taca_Window_get() {
        return wasm_exports.taca_Window_get(platform);
      },
      taca_Window_newRenderingContext(window) {
        return wasm_exports.taca_Window_newRenderingContext(platform, window);
      },
    },
  });
  instance.exports._start();
}
