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
  # Enums

  EventKind* = enum
    frame
    key
    tasksDone
    press
    release

  Step* = enum
    vertex
    instance

  TextAlignX* = enum
    left
    center
    right

  TextAlignY* = enum
    baseline
    top
    middle
    bottom

  # Handles

  Shader* = distinct uint

  # Helpers

  Span*[T] = object
    data: ptr T
    len: int

  # Extern-only objects

  PipelineShaderInfoExtern* = object
    entry: Span[char]
    shader: Shader

  PipelineInfoExtern* = object
    depthTest: bool
    fragment: PipelineShaderInfoExtern
    vertex: PipelineShaderInfoExtern
    vertexAttributes: Span[AttributeInfo]
    vertexBuffers: Span[BufferInfo]

  # Objects

  AttributeInfo* = object
    shaderLocation: uint
    valueOffset: uint

  BufferInfo* = object
    firstAttribute: uint
    step: Step
    stride: uint

  KeyEvent* = object
    pressed: bool
    key: uint32
    text: array[4, uint8]

  PipelineShaderInfo* = object
    entry: string
    shader: Shader

  PipelineInfo* = object
    depthTest: bool
    fragment: PipelineShaderInfo
    vertex: PipelineShaderInfo
    vertexAttributes: seq[AttributeInfo]
    vertexBuffers: seq[BufferInfo]

# Extern-only bindings

proc tacaPrint(text: Span[char]) {.importc: "taca_print".}

proc tacaShaderNew(bytes: Span[char]): Shader {.importc: "taca_shader_new".}

proc tacaTextDraw(text: Span[char], x, y: float32)
  {.importc: "taca_text_draw".}

proc tacaTitleUpdate(text: Span[char]) {.importc: "taca_title_update".}

# Helpers

proc toSpan*(bytes: string): Span[char] =
  result.data = bytes[0].addr
  result.len = bytes.len

proc toSpan*[T](items: seq[T]): Span[T] =
  result.data = items[0].addr
  result.len = items.len

# Main api

proc draw*(itemBegin: uint32, itemCount: uint32, instanceCount: uint32)
  {.importc: "taca_draw".}

proc keyEvent*(): KeyEvent {.importc: "taca_key_event".}

proc print*(text: string) =
  tacaPrint(text.toSpan)

proc shaderNew*(bytes: string): Shader = tacaShaderNew(bytes.toSpan)

proc textAlign*(x: TextAlignX, y: TextAlignY) {.importc: "taca_text_align".}

proc textDraw*(text: string, x, y: float32) =
  tacaTextDraw(text.toSpan, x, y)

proc titleUpdate*(text: string) = tacaTitleUpdate(text.toSpan)
