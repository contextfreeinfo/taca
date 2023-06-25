const fs = require("fs");
const shaderkit = require("shaderkit");
const code = fs.readFileSync("shader.wgsl", "utf8");
const shrunk = shaderkit.minify(code);
fs.writeFileSync("shader.opt.wgsl", shrunk);
