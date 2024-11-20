# import std/sequtils
import std/strformat
import taca

proc start*() {.exportWasm.} =
  titleUpdate("Text Box (Taca Demo)")
  print("Hi from Nim!")
  const shaderBytes = staticRead("../out/shader.spv")
  discard shaderNew(shaderBytes)
  # proc hex(i: int): string = &"{i:02x}"
  # print(&"size: {shader[0..10].map(proc (c: char): string = c.ord.hex)}")

var message: string

proc update*(eventKind: EventKind) {.exportWasm.} =
  case eventKind
  of frame:
    draw(0, 0, 0)
    # textAlign(center, middle)
    # textDraw(message, 0, 0)
    # discard
  of key:
    let event = keyEvent()
    if event.pressed:
      message = &"hi {event.key}"
      print(message)
  else: discard
