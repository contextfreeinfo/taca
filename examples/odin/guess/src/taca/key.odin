package taca

Key_Event :: struct {
	pressed:   bool,
	key:       Key,
	modifiers: u32,
}

Key :: enum u32 {
	None,
	Arrow_Up,
	Arrow_Down,
	Arrow_Left,
	Arrow_Right,
	Space,
	Escape,
	Enter,
	Backspace,
	Delete,
	Numpad_Enter,
}
