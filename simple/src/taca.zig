const c = @import("c.zig");
const std = @import("std");

const assert = std.debug.assert;

pub fn main() !void {
    try runWasm();
    try runSdl();
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
    _ = args;
    _ = results;
    std.debug.print("Calling back...\n", .{});
    std.debug.print("> Hello World!\n", .{});
    return null;
}

fn loadFile(alloc: std.mem.Allocator, name: []const u8) ![]u8 {
    const path = try std.fs.realpathAlloc(alloc, name);
    defer alloc.free(path);
    const file = try std.fs.openFileAbsolute(path, .{ .mode = .read_only });
    defer file.close();
    const result = try file.readToEndAlloc(alloc, (try file.stat()).size);
    std.log.debug("loadFile: {s} {}", .{path, result.len});
    return result;
}

const SysWMinfo = extern struct {
    version: c.SDL_version,
    subsystem: c.SDL_SYSWM_TYPE,
    info: extern union {
        x11: extern struct {
            display: *anyopaque,
            window: c_ulong,
        },
        dummy: [64]u8,
    },
};

fn runWgpu(window: *c.SDL_Window) void {
    var sys: c.SDL_SysWMinfo = undefined;
    c.SDL_GetVersion(&sys.version);
    if (c.SDL_GetWindowWMInfo(window, &sys) == c.SDL_FALSE) {
        // TODO Replace error handling with error returns.
        unreachable;
    }
    // Use a generic type for now so we don't have to worry about compile-time detection.
    // TODO Figure out compile-time detection?
    const generic = @ptrCast(*SysWMinfo, &sys);
    std.debug.print("subsystem: {}\n", .{sys.subsystem});
    const instance = c.wgpuCreateInstance(&c.WGPUInstanceDescriptor{
        .nextInChain = null,
    }) orelse unreachable;
    const sys_descriptor = switch (sys.subsystem) {
        c.SDL_SYSWM_X11 => x11: {
            std.debug.print("x11 info: {} {}\n", .{@ptrToInt(generic.info.x11.display), generic.info.x11.window});
            break :x11 @ptrCast(
                *const c.WGPUChainedStruct,
                &c.WGPUSurfaceDescriptorFromXlibWindow{
                    .chain = .{
                        .next = null,
                        .sType = c.WGPUSType_SurfaceDescriptorFromXlibWindow,
                    },
                    .display = generic.info.x11.display,
                    .window = @intCast(u32, generic.info.x11.window),
                },
            );
        },
        else => null,
    };
    const surface = c.wgpuInstanceCreateSurface(
        instance,
        &c.WGPUSurfaceDescriptor{
            .nextInChain = sys_descriptor,
            .label = null,
        },
    ) orelse unreachable;
    _ = surface;
}

fn runSdl() !void {
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
    ) orelse return TacaError.Display;
    defer c.SDL_DestroyWindow(window);
    runWgpu(window);
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
    const gpa = general_purpose_allocator.allocator();
    const cli_args = try std.process.argsAlloc(gpa);
    defer std.process.argsFree(gpa, cli_args);
    assert(cli_args.len > 1);
    // Init
    var engine = c.wasm_engine_new() orelse return TacaError.Wasm;
    defer c.wasm_engine_delete(engine);
    var store = c.wasm_store_new(engine) orelse return TacaError.Wasm;
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
    var module = c.wasm_module_new(store, &wasm) orelse return TacaError.Wasm;
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
    var instance = c.wasm_instance_new(store, module, &imports, null) orelse return TacaError.Wasm;
    c.wasm_func_delete(hello_func);
    // Extract
    std.log.info("Extracting module exports ...", .{});
    var export_types: c.wasm_exporttype_vec_t = undefined;
    c.wasm_module_exports(module, &export_types);
    defer c.wasm_exporttype_vec_delete(&export_types);
    std.log.info("Extracting instance exports ...", .{});
    var exports: c.wasm_extern_vec_t = undefined;
    c.wasm_instance_exports(instance, &exports);
    defer c.wasm_extern_vec_delete(&exports);
    std.log.info("Extracting main ...", .{});
    var start = findExport("_start", exports, export_types) orelse return TacaError.Wasm;
    var run_func = c.wasm_extern_as_func(start) orelse return TacaError.Wasm;
    c.wasm_instance_delete(instance);
    c.wasm_module_delete(module);
    // Call
    std.log.info("Calling main ...", .{});
    var args = c.wasm_val_vec_t{ .size = 0, .data = null };
    var results = c.wasm_val_vec_t{ .size = 0, .data = null };
    var trap = c.wasm_func_call(run_func, &args, &results);
    assert(trap == null);
}

const TacaError = error {
    Display,
    Wasm,
};

pub const std_options = struct {
    // Set the log level to info
    pub const log_level = .info;

    // Define logFn to override the std implementation
    pub const logFn = myLogFn;
};

pub fn myLogFn(
    comptime level: std.log.Level,
    comptime scope: @TypeOf(.EnumLiteral),
    comptime format: []const u8,
    args: anytype,
) void {
    // Ignore all non-error logging from sources other than
    // .my_project, .nice_library and the default
    const scope_prefix = "(" ++ switch (scope) {
        .my_project, .nice_library, std.log.default_log_scope => @tagName(scope),
        else => if (true) // (@intFromEnum(level) <= @intFromEnum(std.log.Level.err))
            @tagName(scope)
        else
            return,
    } ++ "): ";

    const prefix = "[" ++ comptime level.asText() ++ "] " ++ scope_prefix;

    // Print the message to stderr, silently ignoring any errors
    std.debug.getStderrMutex().lock();
    defer std.debug.getStderrMutex().unlock();
    const stderr = std.io.getStdErr().writer();
    nosuspend stderr.print(prefix ++ format ++ "\n", args) catch return;
}
