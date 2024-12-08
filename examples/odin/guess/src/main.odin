package guess

// import "core:fmt"

foreign import env "env"

@(default_calling_convention = "c")
foreign env {
	taca_print :: proc(text: string) ---
	taca_title_update :: proc(text: string) ---
}

@(export)
start :: proc "c" () {
	taca_title_update("Guessing Game (Taca Demo)")
	// fmt.println("Hellope!")
	taca_print("Hi from Odin!")
}
