const fs = require("fs");
const shaderkit = require("shaderkit");
const code = fs.readFileSync("shader.wgsl", "utf8");
const shrunk = shaderkit.minify(code, {
    mangle: true,
    mangleExternals: true,
    mangleMap: new Map(Object.entries({
        fs_main: "fs_main",
        vs_main: "vs_main",
    })),
});
fs.writeFileSync("shader.opt.wgsl", shrunk);
