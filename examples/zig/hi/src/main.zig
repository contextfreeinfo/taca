const taca = @import("taca.zig");

pub fn main() void {
    const window = taca.Window.get();
    window.print("Hi from Zig!");
    const ctx = window.newRenderingContext();
    const y = @sqrt(3.0) / 4.0;
    const vertex_buffer = ctx.newBuffer(
        .vertex,
        .immutable,
        taca.BufferSlice.new(&[_]Vertex{
            .{ .pos = .{ -0.5, -y }, .color = .{ 1, 0, 0, 1 } },
            .{ .pos = .{ 0.5, -y }, .color = .{ 0, 1, 0, 1 } },
            .{ .pos = .{ 0.0, y }, .color = .{ 0, 0, 1, 1 } },
        }),
    );
    const index_buffer = ctx.newBuffer(
        .index,
        .immutable,
        taca.BufferSlice.new(&[_]u16{ 0, 1, 2 }),
    );
    const shader = ctx.newShader(@embedFile("shader.opt.spv"));
    // const fs = ctx.newShader(@embedFile("fs.spv"));
    // const vs = ctx.newShader(@embedFile("vs.spv"));
    const pipeline = ctx.newPipeline(.{
        // TODO Automate attributes from shader?
        .attributes = &[_]taca.VertexAttribute{
            .{ .format = .float2 },
            .{ .format = .float4 },
        },
        .fragment = .{
            .entry_point = "fs_main",
            .shader = shader,
            // .shader = fs,
        },
        .vertex = .{
            .entry_point = "vs_main",
            .shader = shader,
            // .shader = vs,
        },
    });
    stage = .{
        .ctx = ctx,
        .index_buffer = index_buffer,
        .pipeline = pipeline,
        .vertex_buffer = vertex_buffer,
        .window = window,
    };
}

export fn listen(event: taca.EventKind) void {
    // TODO Branch on event kind.
    _ = event;
    const state = stage.?.window.state();
    const ctx = stage.?.ctx;
    ctx.beginPass();
    ctx.applyPipeline(stage.?.pipeline);
    ctx.applyBindings(.{
        .vertex_buffers = &[_]*taca.Buffer{stage.?.vertex_buffer},
        .index_buffer = stage.?.index_buffer,
    });
    const size = state.size;
    const aspect = size[0] / size[1];
    ctx.applyUniforms(&Uniforms{
        .aspect = if (aspect < 1) .{ 1 / aspect, 1 } else .{ 1, aspect },
        .pointer = .{ state.pointer[0], size[1] - state.pointer[1] },
    });
    ctx.draw(0, 3, 1);
    ctx.endPass();
    ctx.commitFrame();
}

var stage: ?Stage = null;

const Stage = struct {
    ctx: *taca.RenderingContext,
    index_buffer: *taca.Buffer,
    pipeline: *taca.Pipeline,
    vertex_buffer: *taca.Buffer,
    window: *taca.Window,
};

const Uniforms = packed struct {
    aspect: @Vector(2, f32),
    pointer: @Vector(2, f32),
};

const Vertex = extern struct {
    pos: [2]f32,
    color: [4]f32,
};
