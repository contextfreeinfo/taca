const taca = @import("taca.zig");
const ctx = taca.RenderingContext;
const window = taca.Window;

const message = "Hi from Zig!";

export fn start() void {
    window.setTitle("Triangle (Taca Demo)");
    window.print(message);
    const y = @sqrt(3.0) / 4.0;
    _ = ctx.newBuffer(.{
        .slice = taca.BufferSlice.new(&[_]Vertex{
            .{ .pos = .{ -0.5, y }, .color = .{ 1, 0, 0, 1 } },
            .{ .pos = .{ 0.5, y }, .color = .{ 0, 1, 0, 1 } },
            .{ .pos = .{ 0.0, -y }, .color = .{ 0, 0, 1, 1 } },
        }),
    });
    _ = ctx.newBuffer(.{
        .kind = .index,
        .slice = taca.BufferSlice.new(&[_]u16{ 0, 1, 2 }),
    });
    _ = ctx.newShader(@embedFile("shader.opt.spv"));
    stage = .{};
}

export fn update(event: taca.EventKind) void {
    if (event != taca.EventKind.frame) return;
    const state = window.state();
    const size = state.size;
    const aspect = size[0] / size[1];
    ctx.applyUniforms(&Uniforms{
        .aspect = if (aspect < 1) .{ 1 / aspect, 1 } else .{ 1, aspect },
        .count = @floatFromInt(stage.?.count),
        .pointer = state.pointer,
    });
    // Triangle
    ctx.draw(0, 3, 1);
    // Text
    const end = (stage.?.count / 10) % (message.len + 1);
    ctx.drawText(message[0..end], state.pointer[0], state.pointer[1]);
    // Next
    stage.?.count +%= 1;
}

var stage: ?Stage = null;

const Stage = struct {
    count: u32 = 0,
};

const Uniforms = extern struct {
    aspect: [2]f32,
    pointer: [2]f32,
    count: f32,
    pad: f32 = 0,
};

const Vertex = extern struct {
    pos: [2]f32,
    color: [4]f32,
};
