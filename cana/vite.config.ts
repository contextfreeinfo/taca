import { readFile, writeFile } from "node:fs/promises";
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
      name: "separate-wasm",
      apply: "build",
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
