import std/strformat
import taca

proc start*() {.exportWasm.} =
  print("Hi from Nim!")

var message: string

proc update*(event_kind: uint32) {.exportWasm.} =
  case event_kind
  of 0:
    # textDraw(message, 100, 100)
    discard
  of 1:
    let event = keyEvent()
    if event.pressed:
      message = &"hi {event.key}"
      print(message)
  else: discard
