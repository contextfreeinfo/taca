const fs = require("fs");
const shaderkit = require("shaderkit");
function minify(name) {
    const code = fs.readFileSync(`${name}.wgsl`, "utf8");
    const shrunk = shaderkit.minify(code, {
        mangle: true,
        mangleExternals: true,
        mangleMap: new Map(Object.entries({
            fs_main: "fs_main",
            vs_main: "vs_main",
        })),
    });
    fs.writeFileSync(`${name}.opt.wgsl`, shrunk);
}
minify("shader");
