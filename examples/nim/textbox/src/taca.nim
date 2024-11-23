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

  BufferKind* = enum
    vertex
    index

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

  Buffer* = distinct uint
  Pipeline* = distinct uint
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

proc tacaBufferNew(kind: BufferKind, bytes: Span[char]): Buffer
  {.importc: "taca_buffer_new".}

proc tacaPipelineNew(info: PipelineInfoExtern): Pipeline
  {.importc: "taca_pipeline_new".}

proc tacaPrint(text: Span[char]) {.importc: "taca_print".}

proc tacaShaderNew(bytes: Span[char]): Shader {.importc: "taca_shader_new".}

proc tacaTextDraw(text: Span[char], x, y: float32)
  {.importc: "taca_text_draw".}

proc tacaTitleUpdate(text: Span[char]) {.importc: "taca_title_update".}

# Helpers

proc toByteSpan*[T](items: openArray[T]): Span[char] =
  Span[char](data: cast[ptr char](items[0].addr), len: items.len * T.sizeof)

proc toSpan*(bytes: string): Span[char] =
  Span[char](data: bytes[0].addr, len: bytes.len)

proc toSpan*[T](items: seq[T]): Span[T] =
  Span[T](data: items[0].addr, len: items.len)

proc toExtern(info: PipelineShaderInfo): PipelineShaderInfoExtern =
  PipelineShaderInfoExtern(entry: info.entry.toSpan, shader: info.shader)

proc toExtern(info: PipelineInfo): PipelineInfoExtern =
  PipelineInfoExtern(
    depthTest: info.depthTest,
    fragment: info.fragment.toExtern,
    vertex: info.vertex.toExtern,
    vertexAttributes: info.vertexAttributes.toSpan,
    vertexBuffers: info.vertexBuffers.toSpan,
  )

# Main api

proc bufferNew*(kind: BufferKind, len: int): Buffer =
  tacaBufferNew(kind, Span[char](data: nil, len: len))

proc bufferNew*[T](kind: BufferKind, items: openArray[T]): Buffer =
  tacaBufferNew(kind, items.toByteSpan)

proc draw*(itemBegin: uint32, itemCount: uint32, instanceCount: uint32)
  {.importc: "taca_draw".}

proc keyEvent*(): KeyEvent {.importc: "taca_key_event".}

proc pipelineNew*(info: PipelineInfo): Pipeline =
  info.toExtern.tacaPipelineNew

proc print*(text: string) =
  tacaPrint(text.toSpan)

proc shaderNew*(bytes: string): Shader = bytes.toSpan.tacaShaderNew

proc textAlign*(x: TextAlignX, y: TextAlignY) {.importc: "taca_text_align".}

proc textDraw*(text: string, x, y: float32) =
  tacaTextDraw(text.toSpan, x, y)

proc titleUpdate*(text: string) = text.toSpan.tacaTitleUpdate
