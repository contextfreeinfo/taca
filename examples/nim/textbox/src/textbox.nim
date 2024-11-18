import std/macros
import std/strformat

# From here:
# https://github.com/aduros/wasm4/blob/979be845216ee9d613cb6555fb8b11c01bec39a0/cli/assets/templates/nim/src/cart/wasm4.nim#L102
macro exportWasm*(def: untyped): untyped = 
  result = def
  result[^3] = nnkPragma.newTree( 
    ident("exportc"),
    nnkExprColonExpr.newTree(
      ident("codegenDecl"),
      newStrLitNode("__attribute__((export_name(\"$2\"))) $1 $2$3")
    )
  )

type
  KeyEvent = object
    pressed: bool
    key: uint32
    text: array[4, uint8]

proc key_event(): KeyEvent {.importc: "taca_key_event".}

# Doesn't run:
# let name = "World"
var name: string

proc start*() {.exportWasm.} =
  name = "World"
  echo(&"Hello, {name}!")

proc update*(event: uint32) {.exportWasm.} =
  if event == 1:
    let key = key_event()
    echo(&"hi {key.key}")
