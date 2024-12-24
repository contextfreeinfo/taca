package guess

import "base:runtime"
import "core:fmt"
import "core:math/rand"
import "core:strconv"
import "taca"

foreign import env "env"

@(default_calling_convention = "c")
foreign env {
	textbox_bgcolor_update :: proc(r, g, b: f32) ---
	textbox_entry_read :: proc(buffer: taca.Buffer) -> uint ---
	textbox_entry_write :: proc(buffer: taca.Buffer, size: uint) ---
	textbox_label_write :: proc(buffer: taca.Buffer, size: uint) ---
}

App :: struct {
	answer: int,
	buffer: taca.Buffer,
	ctx:    runtime.Context,
	frames: int,
	max:    int,
}

app: App

@(export)
start :: proc "c" () {
	context = runtime.default_context()
	defer free_all(context.temp_allocator)
	taca.title_update("Guessing Game (Taca Demo)")
	taca.print("Hi from Odin!")
	app = {
		buffer = taca.buffer_new(.CPU),
		ctx    = context,
		max    = 100,
	}
	label_update(app, fmt.tprintf("Guess a number between 1 and %d:", app.max))
	textbox_bgcolor_update(0.09, 0.24, 0.4)
}

@(export)
update :: proc "c" (kind: taca.Event_Kind) {
	context = app.ctx
	defer free_all(context.temp_allocator)
	app.frames += 1
	if kind == .Key {
		event := taca.key_event()
		key := event.key
		if event.pressed && (key == .Enter || key == .Numpad_Enter) {
			process_entry(&app)
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

pick_answer :: proc(app: ^App) {
	app.answer = rand.int_max(app.max - 1) + 1
	// taca.print(fmt.tprintf("Answer: %d", app.answer))
}

process_entry :: proc(app: ^App) {
	if app.answer == 0 {
		// Seed random only after first entry so it depends on timing.
		rand.reset(u64(app.frames))
		pick_answer(app)
	}
	entry := entry_read(app^)
	entry_update(app^, "")
	guess, ok := strconv.parse_int(entry)
	if !ok {
		label_update(app^, fmt.tprintf(`"%s" is not an integer. Guess again:`, entry))
	} else if guess < app.answer {
		label_update(app^, fmt.tprintf("%d is too low. Guess again:", guess))
	} else if guess > app.answer {
		label_update(app^, fmt.tprintf("%d is too high. Guess again:", guess))
	} else {
		label_update(app^, fmt.tprintf("%d is the answer! Guess a new number:", guess))
		pick_answer(app)
	}
}

write_string :: proc(buffer: taca.Buffer, text: string) -> uint {
	taca.buffer_update(buffer, transmute([]u8)text)
	return len(text)
}
