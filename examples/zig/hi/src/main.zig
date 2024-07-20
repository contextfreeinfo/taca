const taca = @import("taca.zig");
const window = taca.Window;
const ctx = taca.RenderingContext;

export fn config() void {
    window.setTitle("Hi from Zig!");
}

pub fn main() void {
    // TODO Render text to display.
    window.print("Hi from Zig!");
    const y = @sqrt(3.0) / 4.0;
    _ = ctx.newBuffer(
        .vertex,
        .immutable,
        taca.BufferSlice.new(&[_]Vertex{
            .{ .pos = .{ -0.5, y }, .color = .{ 1, 0, 0, 1 } },
            .{ .pos = .{ 0.5, y }, .color = .{ 0, 1, 0, 1 } },
            .{ .pos = .{ 0.0, -y }, .color = .{ 0, 0, 1, 1 } },
        }),
    );
    _ = ctx.newBuffer(
        .index,
        .immutable,
        taca.BufferSlice.new(&[_]u16{ 0, 1, 2 }),
    );
    // TODO Can any languages run command line tools from their source?
    _ = ctx.newShader(@embedFile("shader.opt.spv"));
    // stage = .{ .ctx = ctx };
}

export fn listen(event: taca.EventKind) void {
    // TODO Branch on event kind.
    _ = event;
    const state = window.state();
    const size = state.size;
    const aspect = size[0] / size[1];
    ctx.applyUniforms(&Uniforms{
        .aspect = if (aspect < 1) .{ 1 / aspect, 1 } else .{ 1, aspect },
        .pointer = .{ state.pointer[0], state.pointer[1] },
    });
    ctx.draw(0, 3, 1);
    ctx.commitFrame();
}

// var stage: ?Stage = null;

// const Stage = struct {
//     ctx: *taca.RenderingContext,
// };

const Uniforms = extern struct {
    aspect: [2]f32,
    pointer: [2]f32,
};

const Vertex = extern struct {
    pos: [2]f32,
    color: [4]f32,
};
