import std/macros

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
  KeyEvent* = object
    pressed: bool
    key: uint32
    text: array[4, uint8]

  Span*[T] = object
    data: ptr T
    len: int

proc tacaPrint(text: Span[char]) {.importc: "taca_print".}

proc tacaTextDraw*(text: Span[char], x, y: float32)
  {.importc: "taca_text_draw".}

proc toSpan*(text: string): Span[char] =
  result.data = text[0].addr
  result.len = text.len

proc keyEvent*(): KeyEvent {.importc: "taca_key_event".}

proc print*(text: string) =
  tacaPrint(text.toSpan)

proc textDraw*(text: string, x, y: float32) =
  tacaTextDraw(text.toSpan, x, y)
