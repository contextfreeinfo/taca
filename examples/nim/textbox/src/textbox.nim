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
    vertexBuffer: bufferNew(vertex, [[-1'f32, -1], [-1, 1], [1, -1], [1, 1]]),
  )
  # proc hex(i: int): string = &"{i:02x}"
  # print(&"size: {shader[0..10].map(proc (c: char): string = c.ord.hex)}")

var message: string

proc update*(eventKind: EventKind) {.exportWasm.} =
  case eventKind
  of frame:
    # draw(0, 0, 0)
    # textAlign(center, middle)
    # textDraw(message, 0, 0)
    discard
  of key:
    let event = keyEvent()
    if event.pressed:
      message = &"hi {event.key}"
      print(message)
  else: discard
