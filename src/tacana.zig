const c = @import("c.zig");
const std = @import("std");

const assert = std.debug.assert;

pub fn main() !void {
    try runWasm();
    runSdl();
}

fn findExport(
    name: []const u8,
    exports: c.wasm_extern_vec_t,
    types: c.wasm_exporttype_vec_t,
) ?*c.wasm_extern_t {
    var i = @as(usize, 0);
    while (i < exports.size) : (i += 1) {
        const found_name: *const c.wasm_name_t =
            c.wasm_exporttype_name(types.data[i]);
        var found_slice = found_name.data[0..found_name.size];
        if (std.mem.eql(u8, name, found_slice)) {
            return exports.data[i];
        }
    }
    return null;
}

fn helloCallback(
    args: ?*const c.wasm_val_vec_t,
    results: ?*c.wasm_val_vec_t,
) callconv(.C) ?*c.wasm_trap_t {
    std.debug.print("Calling back...\n", .{});
    std.debug.print("> Hello World!\n", .{});
    return null;
}

fn loadFile(alloc: *std.mem.Allocator, name: []const u8) ![]u8 {
    const path = try std.fs.path.resolve(alloc, &.{name});
    defer alloc.free(path);
    const file = try std.fs.openFileAbsolute(path, .{ .read = true });
    defer file.close();
    std.debug.print("hi: {s}\n", .{path});
    return try file.readToEndAlloc(alloc, (try file.stat()).size);
}

fn runSdl() void {
    var init_result = c.SDL_Init(c.SDL_INIT_VIDEO);
    assert(init_result >= 0);
    defer c.SDL_Quit();
    var window = c.SDL_CreateWindow(
        "Hi!",
        c.SDL_WINDOWPOS_UNDEFINED,
        c.SDL_WINDOWPOS_UNDEFINED,
        640,
        480,
        c.SDL_WINDOW_SHOWN,
    ).?;
    defer c.SDL_DestroyWindow(window);
    var surface: *c.SDL_Surface = c.SDL_GetWindowSurface(window);
    _ = c.SDL_FillRect(
        surface,
        null,
        c.SDL_MapRGB(surface.format, 0x10, 0x20, 0x30),
    );
    _ = c.SDL_UpdateWindowSurface(window);
    c.SDL_Delay(500);
}

fn runWasm() !void {
    defer std.log.info("Done.", .{});
    // Args
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const gpa = &general_purpose_allocator.allocator;
    const cli_args = try std.process.argsAlloc(gpa);
    defer std.process.argsFree(gpa, cli_args);
    assert(cli_args.len > 1);
    // Init
    var engine = c.wasm_engine_new().?;
    defer c.wasm_engine_delete(engine);
    var store = c.wasm_store_new(engine).?;
    defer c.wasm_store_delete(store);
    defer std.log.info("Shutting down ...", .{});
    // var context = c.wasmtime_store_context(store);
    // Load wat and wasm
    // const wat = @embedFile("hello.wat");
    var wasm_bytes = try loadFile(gpa, cli_args[1]);
    // var wasm_bytes = @embedFile("../examples/hello.wasm");
    // var wasm_bytes = @embedFile("../examples/as/build/optimized.wasm");
    var wasm = c.wasm_byte_vec_t{
        .size = wasm_bytes.len,
        .data = wasm_bytes.ptr,
    };
    // var err = c.wasmtime_wat2wasm(wat, wat.len, &wasm);
    // assert(err == null);
    // Compile
    std.log.info("Compiling module ...", .{});
    var module = c.wasm_module_new(store, &wasm).?;
    // defer c.wasm_module_delete(module);
    gpa.free(wasm_bytes);
    // Register
    std.log.info("Creating callback ...", .{});
    var hello_type = c.wasm_functype_new_0_0();
    var hello_func = c.wasm_func_new(store, hello_type, helloCallback);
    c.wasm_functype_delete(hello_type);
    // Instantiate
    std.log.info("Instantiating module ...", .{});
    var externs = [_]?*c.wasm_extern_t{c.wasm_func_as_extern(hello_func)};
    var imports = c.wasm_extern_vec_t{ .size = externs.len, .data = &externs };
    var instance = c.wasm_instance_new(store, module, &imports, null).?;
    c.wasm_func_delete(hello_func);
    // Extract
    std.log.info("Extracting export ...", .{});
    var exports: c.wasm_extern_vec_t = undefined;
    c.wasm_instance_exports(instance, &exports);
    defer c.wasm_extern_vec_delete(&exports);
    var export_types: c.wasm_exporttype_vec_t = undefined;
    c.wasm_module_exports(module, &export_types);
    defer c.wasm_exporttype_vec_delete(&export_types);
    var start = findExport("_start", exports, export_types).?;
    var run_func = c.wasm_extern_as_func(start).?;
    c.wasm_instance_delete(instance);
    c.wasm_module_delete(module);
    // Call
    std.log.info("Calling export ...", .{});
    var args = c.wasm_val_vec_t{ .size = 0, .data = null };
    var results = c.wasm_val_vec_t{ .size = 0, .data = null };
    var trap = c.wasm_func_call(run_func, &args, &results);
    assert(trap == null);
}
