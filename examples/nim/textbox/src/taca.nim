import std/macros
# import std/strformat

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
    uniform
    cpu

  EventKind* = enum
    frame
    key
    tasksDone
    press # TODO Single touch event kind to match key?
    release
    text

  Key* = enum
    none
    arrowUp
    arrowDown
    arrowLeft
    arrowRight
    space
    escape
    enter
    backspace
    delete
    numpadEnter

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

  Bindings* = distinct uint
  Buffer* = distinct uint
  Pipeline* = distinct uint
  Sampler* = distinct uint
  Shader* = distinct uint
  Texture* = distinct uint

  # Helpers

  Span*[T] = object
    data: ptr T
    len: int

  Vec2* = array[2, float32]

  # Extern-only objects

  BindingsInfoExtern* = object
    pipeline: Pipeline
    group_index: uint
    buffers: Span[Buffer]
    samplers: Span[Sampler]
    textures: Span[Texture]

  BuffersExtern* = object
    vertexBuffers: Span[Buffer]
    indexBuffer: Buffer

  KeyEventExtern* = object
    pressed: bool
    key: uint32
    modifiers: uint32

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

  BindingsInfo* = object
    pipeline: Pipeline
    group_index: uint
    buffers: seq[Buffer]
    samplers: seq[Sampler]
    textures: seq[Texture]

  BufferInfo* = object
    firstAttribute: uint
    step: Step
    stride: uint

  KeyEvent* = object
    pressed: bool
    key: Key
    modifiers: uint32

  PipelineShaderInfo* = object
    entry: string
    shader: Shader

  PipelineInfo* = object
    depthTest: bool
    fragment: PipelineShaderInfo
    vertex: PipelineShaderInfo
    vertexAttributes: seq[AttributeInfo]
    vertexBuffers: seq[BufferInfo]

  TextEvent* = object
    buffer: Buffer
    size: int

  WindowState* = object
    pointer: Vec2
    press: uint32
    size: Vec2

# Extern-only bindings

proc tacaBindingsNew(info: BindingsInfoExtern): Bindings
  {.importc: "taca_bindings_new".}

proc tacaBufferNew(kind: BufferKind, bytes: Span[char]): Buffer
  {.importc: "taca_buffer_new".}

proc tacaBufferRead(buffer: Buffer, bytes: Span[char], bufferOffset: uint)
  {.importc: "taca_buffer_read".}

proc tacaBufferUpdate(buffer: Buffer, bytes: Span[char], bufferOffset: uint)
  {.importc: "taca_buffer_update".}

proc tacaBuffersApply(buffers: BuffersExtern) {.importc: "taca_buffers_apply".}

proc tacaKeyEvent*(): KeyEventExtern {.importc: "taca_key_event".}

proc tacaPipelineNew(info: PipelineInfoExtern): Pipeline
  {.importc: "taca_pipeline_new".}

proc tacaPrint(text: Span[char]) {.importc: "taca_print".}

proc tacaShaderNew(bytes: Span[char]): Shader {.importc: "taca_shader_new".}

proc tacaTextDraw(text: Span[char], x, y: float32)
  {.importc: "taca_text_draw".}

proc tacaTitleUpdate(text: Span[char]) {.importc: "taca_title_update".}

# Helpers

proc toByteSpan*[T](items: openArray[T]): Span[char] =
  let data = if items.len > 0: cast[ptr char](items[0].addr) else: nil
  Span[char](data: data, len: items.len * T.sizeof)
  # tacaPrint((&"{result.repr} {cast[ptr char](result.addr).repr}").toSpan)

proc toSpan*(bytes: string): Span[char] =
  # Getting the address of an empty string was crashing the app somehow.
  let data = if bytes.len > 0: bytes[0].addr else: nil
  Span[char](data: data, len: bytes.len)

proc toSpan*[T](items: openArray[T]): Span[T] =
  let data = if items.len > 0: items[0].addr else: nil
  Span[T](data: data, len: items.len)

proc toExtern(info: BindingsInfo): BindingsInfoExtern =
  BindingsInfoExtern(
    pipeline: info.pipeline,
    group_index: info.group_index,
    buffers: info.buffers.toSpan,
    samplers: info.samplers.toSpan,
    textures: info.textures.toSpan,
  )

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

proc bindingsApply*(bindings: Bindings) {.importc: "taca_bindings_apply".}

proc bindingsNew*(info: BindingsInfo): Bindings =
  tacaBindingsNew(info.toExtern)

proc bufferNew*(kind: BufferKind, len: int): Buffer =
  tacaBufferNew(kind, Span[char](data: nil, len: len))

proc bufferNew*[T](kind: BufferKind, items: openArray[T]): Buffer =
  tacaBufferNew(kind, items.toByteSpan)

proc bufferRead*[T](
  buffer: Buffer, items: var openArray[T], itemOffset: int = 0
) = tacaBufferRead(buffer, items.toByteSpan, uint(itemOffset * T.sizeof))

proc bufferUpdate*[T](
  buffer: Buffer, items: openArray[T], itemOffset: int = 0
) = tacaBufferUpdate(buffer, items.toByteSpan, uint(itemOffset * T.sizeof))

proc buffersApply*(indexBuffer: Buffer, vertexBuffers: openArray[Buffer]) =
  let buffers = BuffersExtern(
    vertexBuffers: vertexBuffers.toSpan,
    indexBuffer: indexBuffer,
  )
  buffers.tacaBuffersApply

proc draw*(itemBegin: uint32, itemCount: uint32, instanceCount: uint32)
  {.importc: "taca_draw".}

proc keyEvent*(): KeyEvent =
  let extern = tacaKeyEvent()
  KeyEvent(
    pressed: extern.pressed,
    key: Key(extern.key),
    modifiers: extern.modifiers
  )

proc pipelineNew*(info: PipelineInfo): Pipeline = info.toExtern.tacaPipelineNew

proc print*(text: string) = tacaPrint(text.toSpan)

proc shaderNew*(bytes: string): Shader = bytes.toSpan.tacaShaderNew

proc textAlign*(x: TextAlignX, y: TextAlignY) {.importc: "taca_text_align".}

proc textDraw*(text: string, x, y: float32) = tacaTextDraw(text.toSpan, x, y)

proc textEvent*(): TextEvent {.importc: "taca_text_event".}

proc titleUpdate*(text: string) = text.toSpan.tacaTitleUpdate

proc windowState*(): WindowState {.importc: "taca_window_state".}
