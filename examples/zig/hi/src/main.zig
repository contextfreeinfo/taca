const taca = @import("taca.zig");

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
}

export fn listen(event: taca.EventKind) void {
    // TODO Branch on event type.
    _ = event;
    const ctx = stage.?.ctx;
    ctx.beginPass();
    ctx.applyPipeline(stage.?.pipeline);
    ctx.applyBindings(.{
        .vertex_buffers = &[_]*taca.Buffer{stage.?.vertex_buffer},
        .index_buffer = stage.?.index_buffer,
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
};

const Vertex = extern struct {
    pos: [2]f32,
    color: [4]f32,
};
