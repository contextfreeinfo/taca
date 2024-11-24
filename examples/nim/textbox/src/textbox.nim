# import std/sequtils
import std/strformat
import taca

type
  DrawInstance = object
    color: array[4, float32]
    offset: array[2, float32]
    scale: array[2, float32]

  App = object
    indexBuffer: Buffer
    instanceBuffer: Buffer
    message: string
    vertexBuffer: Buffer

var app: App

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
    vertexBuffer: bufferNew(vertex, [[-1'f32, -1], [-1, 1], [1, -1], [1, 1]]),
  )
  app.instanceBuffer.bufferUpdate([
    DrawInstance(color: [0, 0, 0.2, 1], scale: [1, 1]),
  ])
  # proc hex(i: int): string = &"{i:02x}"
  # print(&"size: {shader[0..10].map(proc (c: char): string = c.ord.hex)}")

proc update*(eventKind: EventKind) {.exportWasm.} =
  case eventKind
  of frame:
    buffersApply(app.indexBuffer, [app.vertexBuffer, app.instanceBuffer])
    draw(0, 6, 1)
    let size = windowState().size
    textAlign(center, middle)
    textDraw(app.message, size[0] / 2, size[1] / 2)
  of key:
    let event = keyEvent()
    if event.pressed:
      app.message = &"Key code: {event.key}"
  else: discard
