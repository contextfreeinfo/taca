import { readFile, unlink, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { defineConfig } from "vite";

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, "src/main.ts"),
      name: "Taca",
      fileName: "taca",
    },
  },
  plugins: [
    {
      name: "remove-wasm",
      apply({ mode }) {
        return mode != "split";
      },
      async buildStart() {
        try {
          await unlink(resolve(__dirname, "public/taca.wasm"));
        } catch {
          // That's fine.
        }
      },
    },
    {
      name: "split-wasm",
      apply(_, { command, mode }) {
        return command == "build" && mode == "split";
      },
      async buildStart() {
        // This works for build but not for preview.
        const jsPath = resolve(__dirname, "pkg/cana.js");
        let jsContent = (await readFile(jsPath)).toString();
        // Prevent inlining.
        jsContent = jsContent.replace("'cana_bg.wasm'", "'taca' + '.wasm'");
        await writeFile(jsPath, jsContent);
        // Copy wasm to public.
        const wasmPathIn = resolve(__dirname, "pkg/cana_bg.wasm");
        const wasmPathOut = resolve(__dirname, "public/taca.wasm");
        await writeFile(wasmPathOut, await readFile(wasmPathIn));
      },
    },
  ],
});
