# import std/sequtils
import std/strformat
# import std/strutils
import std/unicode
import taca

type
  DrawInstance = object
    color: array[4, float32]
    offset: array[2, float32]
    scale: array[2, float32]

  App = object
    bgcolor: array[4, float32]
    bindings: Bindings
    entry: string
    frames: float32
    fontSize: float32
    indexBuffer: Buffer
    instanceBuffer: Buffer
    label: string
    textInput: string
    uniformsBuffer: Buffer
    vertexBuffer: Buffer

  Uniforms = object
    color: array[4, float32]
    frames: float32
    pad: array[3, float32]

var app: App

proc textbox_bgcolor_update*(
  r: float32, g: float32, b: float32
) {.exportWasm.} =
  app.bgcolor = [r, g, b, 1]

proc textbox_entry_read*(buffer: Buffer): int {.exportWasm.} =
  bufferUpdate(buffer, app.entry)
  app.entry.len

proc textbox_entry_write*(buffer: Buffer, size: uint) {.exportWasm.} =
  app.entry.setLen(size)
  bufferRead(buffer, app.entry)

proc textbox_label_write*(buffer: Buffer, size: uint) {.exportWasm.} =
  app.label.setLen(size)
  bufferRead(buffer, app.label)

proc start*() {.exportWasm.} =
  titleUpdate("Text Box (Taca Demo)")
  print("Hi from Nim!")
  const shaderBytes = staticRead("../out/shader.spv")
  discard shaderBytes.shaderNew
  discard PipelineInfo(
    vertexBuffers: @[
      BufferInfo(),
      BufferInfo(firstAttribute: 1, step: instance),
    ],
  ).pipelineNew
  let uniformsBuffer = taca.bufferNew(uniform, Uniforms.sizeof)
  app = App(
    bgcolor: [0, 0, 0.2, 1],
    bindings: BindingsInfo(buffers: @[uniformsBuffer]).bindingsNew,
    fontSize: 30,
    indexBuffer: bufferNew(index, [0'u16, 1, 2, 1, 3, 2]),
    instanceBuffer: bufferNew(vertex, 10 * DrawInstance.sizeof),
    label: "Enter text:",
    uniformsBuffer: uniformsBuffer,
    vertexBuffer: bufferNew(vertex, [[-1'f32, -1], [-1, 1], [1, -1], [1, 1]]),
  )
  app.instanceBuffer.bufferUpdate([
    DrawInstance(color: [1, 1, 1, 1], scale: [1, 1]),
  ])

proc removeLastRune(s: var string) =
  if s.len > 0:
    let lastRuneLength = s.lastRune(s.len - 1)[1]
    s.setLen(s.len - lastRuneLength)

proc update*(eventKind: EventKind) {.exportWasm.} =
  case eventKind
  of frame:
    let uniforms = Uniforms(color: app.bgcolor, frames: app.frames)
    bufferUpdate(app.uniformsBuffer, [uniforms])
    bindingsApply(app.bindings)
    buffersApply(app.indexBuffer, [app.vertexBuffer, app.instanceBuffer])
    draw(0, 6, 1)
    let size = windowState().size
    textAlign(center, middle)
    let adjustY = 1.5 * app.fontSize / 2
    let screenCenter = [size[0] / 2, size[1] / 2]
    textDraw(app.label, screenCenter[0], screenCenter[1] - adjustY)
    textDraw(&"{app.entry}|", screenCenter[0], screenCenter[1] + adjustY)
    app.frames += 1
  of key:
    let event = keyEvent()
    # print(event.repr)
    if not event.pressed:
      return
    # app.label = &"Key code: {event.key}"
    if event.key == backspace:
      removeLastRune(app.entry)
  of text:
    let event = textEvent()
    # print(event.repr)
    app.textInput.setLen(event.size)
    bufferRead(event.buffer, app.textInput)
    app.entry &= app.textInput
    # app.label = &"Text: \"{app.textInput}\""
  else: discard
