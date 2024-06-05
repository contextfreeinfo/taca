const c = @cImport({
    @cInclude("taca.h");
});
const std = @import("std");

pub fn main() !void {
    c.taca_windowSetTitle("Hi, Taca!");
    const pipeline = c.taca_Pipeline{
        .fragment = fragment,
        .vertex = vertex,
    };
    const pipeline_data = c.taca_PipelineData{
        .uniforms = null,
        .attributes = &layout,
        .attributeCount = 2, // TODO Need more/less info for layout???
        .vertexCount = vertices.len,
        .vertexOutSize = @sizeOf(c.taca_Vec4),
    };
    c.taca_draw(&pipeline, &pipeline_data);
}

const Vec2 = struct {
    x: f32,
    y: f32,
};

export fn fragment(
    input: [*c]const c.taca_ShaderInput,
    output: [*c]c.taca_Rgba,
) void {
    _ = input;
    output.* = .{ .r = 0.0, .g = 0.5, .b = 1.0, .a = 1.0 };
}

export fn vertex(
    input: [*c]const c.taca_ShaderInput,
    output: [*c]c.taca_VertexOutput,
) void {
    const in_position: *const Vec2 = @ptrCast(
        @alignCast(input.*.attributes[0]),
    );
    output.*.position = .{
        .x = in_position.x,
        .y = in_position.y,
        .z = 0.0,
        .w = 1.0,
    };
}

const vertices = [_]Vec2{
    .{ .x = 0.0, .y = 0.0 },
    .{ .x = 1.0, .y = 0.0 },
    .{ .x = 0.0, .y = 1.0 },
    .{ .x = 0.0, .y = 1.0 },
    .{ .x = 1.0, .y = 1.0 },
    .{ .x = 1.0, .y = 0.0 },
};

const layout = c.taca_AttributeLayout{
    .start = &vertices,
    .stride = @sizeOf(Vec2),
};
