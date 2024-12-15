package guess

import "base:runtime"
// import "core:encoding/endian"
import "core:fmt"
import "taca"

foreign import env "env"

@(default_calling_convention = "c")
foreign env {
	textbox_entry_read :: proc(buffer: taca.Buffer) -> uint ---
	textbox_entry_write :: proc(buffer: taca.Buffer, size: uint) -> uint ---
	textbox_label_write :: proc(buffer: taca.Buffer, size: uint) ---
}

App :: struct {
	buffer: taca.Buffer,
	ctx:    runtime.Context,
}

app: App

@(export)
start :: proc "c" () {
	context = runtime.default_context()
	defer free_all(context.temp_allocator)
	taca.title_update("Guessing Game (Taca Demo)")
	// fmt.println("Hellope!")
	taca.print("Hi from Odin!")
	app = {
		buffer = taca.buffer_new(.Cpu),
		ctx    = context,
	}
	label_update(app, "Guess a number between 1 and 100:")
}

@(export)
update :: proc "c" (kind: taca.Event_Kind) {
	context = app.ctx
	defer free_all(context.temp_allocator)
	// taca.print(fmt.tprintf("%d", kind))
	if kind == .Key {
		event := taca.key_event()
		if event.pressed && event.key == .Enter {
			entry := entry_read(app)
			entry_update(app, "")
			taca.print(fmt.tprintf("got %s", entry))
		}
	}
}

entry_read :: proc(app: App) -> string {
	size := textbox_entry_read(app.buffer)
	buf := make([]u8, size, context.temp_allocator)
	taca.buffer_read(app.buffer, buf)
	return transmute(string)buf
}

entry_update :: proc(app: App, text: string) {
	textbox_entry_write(app.buffer, write_string(app.buffer, text))
}

label_update :: proc(app: App, text: string) {
	textbox_label_write(app.buffer, write_string(app.buffer, text))
}

write_string :: proc(buffer: taca.Buffer, text: string) -> uint {
	taca.buffer_update(buffer, transmute([]u8)text)
	return len(text)
}
