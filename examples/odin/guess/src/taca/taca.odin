package taca

foreign import env "env"

@(default_calling_convention = "c")
foreign env {
	@(link_name = "taca_buffer_new")
	buffer_new :: proc(kind: BufferKind, bytes: []u8 = nil) -> Buffer ---

	@(link_name = "taca_buffer_read")
	buffer_read :: proc(buffer: Buffer, bytes: []u8, buffer_offset: uint = 0) ---

	@(link_name = "taca_buffer_update")
	buffer_update :: proc(buffer: Buffer, bytes: []u8, buffer_offset: uint = 0) ---

	@(link_name = "taca_print")
	print :: proc(text: string) ---

	@(link_name = "taca_title_update")
	title_update :: proc(text: string) ---
}

Buffer :: distinct u32

BufferKind :: enum {
	Vertex,
	Index,
	Uniform,
	Cpu,
}

EventKind :: enum {
	Frame,
	Key,
	TasksDone,
	Press, // TODO Single touch event kind to match key?
	Release,
	Text,
}
