package guess

import "core:fmt"

foreign import env "env"

@(default_calling_convention = "c")
foreign env {
	taca_print :: proc(text: string) ---
}

@(export)
start :: proc "c" () {
	// fmt.println("Hellope!")
	taca_print("Hi!")
}
