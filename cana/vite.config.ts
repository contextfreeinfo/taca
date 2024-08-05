import { readdir, readFile, unlink, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { defineConfig } from "vite";

let runtimeWasm: String | undefined;

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, "src/main.ts"),
      name: "Taca",
      fileName: "taca",
    },
    target: "es2022",
  },
  plugins: [
    {
      // TODO What makes the cjs file?
      name: "remove-cjs",
      apply: "build",
      async closeBundle() {
        const distDir = resolve(__dirname, "dist");
        for (const file of await readdir(distDir)) {
          if (file.endsWith(".cjs")) {
            await unlink(resolve(distDir, file));
          }
        }
      }
    },
    {
      name: "place-index",
      apply: "build",
      async closeBundle() {
        const inPath = resolve(__dirname, "index.html");
        const outPath = resolve(__dirname, "dist/index.html");
        let content = (await readFile(inPath)).toString();
        content = content.replace("/src/main.ts", "./taca.js");
        if (runtimeWasm) {
          content = content.replace(/(runtimeWasm = )\w+/, `$1${runtimeWasm}`);
        }
        await writeFile(outPath, content);
      }
    },
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
        runtimeWasm = `fetch("taca.wasm")`;
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
