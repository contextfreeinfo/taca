import std/strformat
# Package

version       = "0.1.0"
author        = "Tom"
description   = "A new awesome nimble package"
license       = "MIT"
srcDir        = "src"
bin           = @["textbox.wasm"]
binDir = "out"


# Dependencies

requires "nim >= 2.2.0"

# task build, "Build":
#   exec &"nim c --out:bin/ src/textbox.nim"
