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
  instance.exports.run();
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
      runWasm(wasm, length) {
        runWasm(wasm_memory, wasm, length);
      },
    });
  },
});

// load("../target/wasm32-unknown-unknown/release/taca.wasm");
// load("../target/wasm32-unknown-unknown/release-lto/taca.wasm");
load("../target/wasm32-unknown-unknown/release-lto/taca.opt.wasm");
