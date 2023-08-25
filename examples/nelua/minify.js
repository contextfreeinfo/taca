const fs = require("fs");
const shaderkit = require("shaderkit");
/** @param {Array<string>} names */
function minify(names) {
    const code = names.map(
        (name) => fs.readFileSync(`${name}.wgsl`, "utf8")
    ).join("\n");
    const shrunk = shaderkit.minify(code, {
        mangle: true,
        mangleExternals: true,
        mangleMap: new Map(Object.entries({
            fs_main: "fs_main",
            vs_main: "vs_main",
        })),
    });
    const repaired = shrunk.replace(/([\d\w])-(\d)/g, "$1 - $2");
    fs.writeFileSync(`${names.at(-1)}.opt.wgsl`, repaired);
}
minify(["noise", "shader"]);
