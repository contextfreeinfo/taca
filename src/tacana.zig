const c = @import("c.zig");
const std = @import("std");

const assert = std.debug.assert;

const SomeError = error {
    Error,
};

pub fn main() !void {
    // Init
    var engine = c.wasm_engine_new() orelse unreachable;
    defer c.wasm_engine_delete(engine);
    var store = c.wasmtime_store_new(engine, null, null) orelse unreachable;
    defer c.wasmtime_store_delete(store);
    var context = c.wasmtime_store_context(store);
    // Load
    const wat = @embedFile("hello.wat");
    var wasm: c.wasm_byte_vec_t = undefined;
    var err = c.wasmtime_wat2wasm(wat, wat.len, &wasm);
    if (err != null) {
        std.log.err("failed to parse wat", .{});
        return SomeError.Error;
    }
    // TODO Coordinate early delete of wasm.
    defer c.wasm_byte_vec_delete(&wasm);
    // Compile
    std.log.info("Compiling module ...", .{});
    var module: ?*c.wasmtime_module_t = null;
    err = c.wasmtime_module_new(engine, wasm.data, wasm.size, &module);
    if (err != null) {
        std.log.err("failed to compile module", .{});
        return SomeError.Error;
    }
    // defer c.wasm_module_delete(@ptrCast(*c.wasm_module_t, module));
}

// fn exit_with_error(const char *message, wasmtime_error_t *error, wasm_trap_t *trap) !void {
//   fprintf(stderr, "error: %s\n", message);
//   wasm_byte_vec_t error_message;
//   if (error != NULL) {
//     wasmtime_error_message(error, &error_message);
//     wasmtime_error_delete(error);
//   } else {
//     wasm_trap_message(trap, &error_message);
//     wasm_trap_delete(trap);
//   }
//   fprintf(stderr, "%.*s\n", (int) error_message.size, error_message.data);
//   wasm_byte_vec_delete(&error_message);
//   exit(1);
// }
