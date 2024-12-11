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

  TextBox = object
    alignX: TextAlignX
    alignY: TextAlignY
    offset: array[2, float32]
    text: string

  App = object
    focus: int
    indexBuffer: Buffer
    instanceBuffer: Buffer
    message: string
    textBoxes: seq[TextBox]
    textInput: string
    vertexBuffer: Buffer

var app: App

proc textbox_entry_read*(buffer: Buffer) {.exportWasm.} =
  var text = app.textBoxes[app.focus].text
  print(&"textbox_entry_read({buffer.repr}) -> {text}")

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
  app = App(
    indexBuffer: bufferNew(index, [0'u16, 1, 2, 1, 3, 2]),
    instanceBuffer: bufferNew(vertex, 10 * DrawInstance.sizeof),
    message: "Press any key",
    textBoxes: @[TextBox(alignX: center, alignY: middle, offset: [0, 30])],
    vertexBuffer: bufferNew(vertex, [[-1'f32, -1], [-1, 1], [1, -1], [1, 1]]),
  )
  app.instanceBuffer.bufferUpdate([
    DrawInstance(color: [0, 0, 0.2, 1], scale: [1, 1]),
  ])
  # proc hex(i: int): string = &"{i:02x}"
  # print(&"size: {shader[0..10].map(proc (c: char): string = c.ord.hex)}")

proc removeLastRune(s: var string) =
  if s.len > 0:
    let lastRuneLength = s.lastRune(s.len - 1)[1]
    s.setLen(s.len - lastRuneLength)

proc update*(eventKind: EventKind) {.exportWasm.} =
  case eventKind
  of frame:
    buffersApply(app.indexBuffer, [app.vertexBuffer, app.instanceBuffer])
    draw(0, 6, 1)
    let size = windowState().size
    textAlign(center, middle)
    let messageOffset = [size[0] / 2, size[1] / 2]
    textDraw(app.message, messageOffset[0], messageOffset[1])
    for textBox in app.textBoxes:
      let offsetX = messageOffset[0] + textBox.offset[0]
      let offsetY = messageOffset[1] + textBox.offset[1]
      textAlign(textBox.alignX, textBox.alignY)
      textDraw(&"{textBox.text}|", offsetX, offsetY)
  of key:
    let event = keyEvent()
    # print(event.repr)
    if not event.pressed:
      return
    app.message = &"Key code: {event.key}"
    if event.key == backspace and app.focus >= 0:
      removeLastRune(app.textBoxes[app.focus].text)
  of text:
    let event = textEvent()
    if app.focus < 0:
      return
    let textBox = addr app.textBoxes[app.focus]
    # print(event.repr)
    app.textInput.setLen(event.size)
    bufferRead(event.buffer, app.textInput)
    # print(app.textInput)
    textBox.text &= app.textInput
    app.message = &"Text: \"{app.textInput}\""
    # print(textBox.repr)
  else: discard
