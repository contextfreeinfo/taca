package guess

// import "base:runtime"
import "core:fmt"

foreign import env "env"

@(default_calling_convention = "c")
foreign env {
	taca_print :: proc(text: string) ---
	taca_title_update :: proc(text: string) ---
}

EventKind :: enum {
    Frame,
    Key,
    TasksDone,
    Press, // TODO Single touch event kind to match key?
    Release,
    Text,
}

@(default_calling_convention = "c")
foreign env {
	textbox_entry_read :: proc(buffer: u32) ---
}

@(export)
start :: proc "c" () {
	taca_title_update("Guessing Game (Taca Demo)")
	// fmt.println("Hellope!")
	taca_print("Hi from Odin!")
}

@(export)
update :: proc "c" (kind: EventKind) {
	// context = runtime.default_context()
	// taca_print(fmt.tprintf("%d", kind))
	if kind == .Key {
		textbox_entry_read(0)
	}
}
