const std = @import("std");
const taca = @import("taca.zig");

const Vertex = extern struct {
    pos: [2]f32,
    color: [4]f32,
};

const Stage = struct {
    ctx: *taca.RenderingContext,
    index_buffer: *taca.Buffer,
    pipeline: *taca.Pipeline,
    vertex_buffer: *taca.Buffer,
};

var stage: ?Stage = null;

pub fn main() void {
    const window = taca.Window.get();
    const ctx = window.newRenderingContext();
    const vertices = [_]Vertex{
        .{ .pos = .{ -0.5, -0.5 }, .color = .{ 1.0, 0.0, 0.0, 1.0 } },
        .{ .pos = .{ 0.5, -0.5 }, .color = .{ 0.0, 1.0, 0.0, 1.0 } },
        .{ .pos = .{ 0.0, 0.5 }, .color = .{ 0.0, 0.0, 1.0, 1.0 } },
    };
    const vertex_buffer = ctx.newBuffer(
        .vertex_buffer,
        .immutable,
        taca.BufferSlice.new(&vertices),
    );
    const indices = [_]u16{ 0, 1, 2 };
    const index_buffer = ctx.newBuffer(
        .index_buffer,
        .immutable,
        taca.BufferSlice.new(&indices),
    );
    const shader = ctx.newShader(@embedFile("shader.opt.spv"));
    const pipeline = ctx.newPipeline(.{
        .attributes = &[_]taca.VertexAttribute{
            .{ .format = .float2 },
            .{ .format = .float4 },
        },
        .fragment = .{
            .entry_point = "fs_main",
            .shader = shader,
        },
        .vertex = .{
            .entry_point = "vs_main",
            .shader = shader,
        },
    });
    stage = .{
        .ctx = ctx,
        .index_buffer = index_buffer,
        .pipeline = pipeline,
        .vertex_buffer = vertex_buffer,
    };
    // std.debug.print("All your {s} are belong to us.\n", .{"codebase"});
}

export fn listen(event: taca.Event) void {
    _ = event;
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
