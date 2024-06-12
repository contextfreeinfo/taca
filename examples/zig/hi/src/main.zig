const std = @import("std");
const taca = @import("taca.zig");

const Vertex = extern struct {
    pos: [2]f32,
    color: [4]f32,
};

const Stage = struct {
    pipeline: *taca.Pipeline,
    // TODO bindings,
    ctx: *taca.RenderingContext,
};

pub fn main() !void {
    const window = taca.Window.get();
    const ctx = window.newRenderingContext();
    const vertices = [_]Vertex{
        .{ .pos = .{ -0.5, -0.5 }, .color = .{ 1.0, 0.0, 0.0, 1.0 } },
        .{ .pos = .{ 0.5, -0.5 }, .color = .{ 0.0, 1.0, 0.0, 1.0 } },
        .{ .pos = .{ 0.0, 0.5 }, .color = .{ 0.0, 0.0, 1.0, 1.0 } },
    };
    const vertex_buffer = ctx.newBuffer(
        taca.BufferType.VertexBuffer,
        taca.BufferUsage.Immutable,
        taca.BufferSlice.new(&vertices),
    );
    _ = vertex_buffer;
    const indices = [_]u16{ 0, 1, 2 };
    const index_buffer = ctx.newBuffer(
        taca.BufferType.IndexBuffer,
        taca.BufferUsage.Immutable,
        taca.BufferSlice.new(&indices),
    );
    _ = index_buffer;
    const shader = ctx.newShader(@embedFile("shader.opt.spv"));
    const pipeline = ctx.newPipeline(.{
        .fragment = .{
            .entry_point = "fs_main",
            .shader = shader,
        },
        .vertex = .{
            .entry_point = "vs_main",
            .shader = shader,
        },
    });
    _ = pipeline;
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    try bw.flush(); // don't forget to flush!
}

test "hi" {
    const nums: []const i32 = &[_]i32{ 3, 4, 5 };
    try blah(nums);
}

fn blah(comptime items: anytype) !void {
    const expect = @import("std").testing.expect;
    const info = @typeInfo(@TypeOf(items));
    try expect(info.Pointer.size == .Slice);
    try expect(@sizeOf(info.Pointer.child) == 4);
    try expect(items.len == 3);
}
