const assert = require("assert");
const myModule = require("..");
assert.strictEqual(myModule.add(1, 2), 3);
console.log("ok");
