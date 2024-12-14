package guess

import "base:runtime"
// import "core:encoding/endian"
// import "core:fmt"
import "taca"

foreign import env "env"

@(default_calling_convention = "c")
foreign env {
	textbox_entry_read :: proc(buffer: taca.Buffer) ---
	textbox_label_write :: proc(buffer: taca.Buffer) ---
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
		ctx = context,
	}
	write_string(app.buffer, "Guess a number between 1 and 100:")
	textbox_label_write(app.buffer)
}

@(export)
update :: proc "c" (kind: taca.EventKind) {
	context = app.ctx
	defer free_all(context.temp_allocator)
	// taca.print(fmt.tprintf("%d", kind))
	if kind == .Key {
		textbox_entry_read(app.buffer)
	}
}

write_string :: proc(buffer: taca.Buffer, text: string) {
	buf := make([]u8, 4 + len(text), context.temp_allocator)
	size: u32 = cast(u32)len(text)
	// taca.print(fmt.tprintf("%d", size))
	// endian.put_u32(buf, .Little, cast(u32)len(text))
	runtime.mem_copy_non_overlapping(raw_data(buf), transmute(^u8)&size, size_of(size))
	runtime.mem_copy_non_overlapping(raw_data(buf[size_of(size):]), raw_data(text), len(text))
	taca.buffer_update(buffer, buf[:])
}
