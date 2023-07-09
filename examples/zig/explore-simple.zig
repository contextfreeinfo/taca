const c = @cImport({@cInclude("taca.h");});
const d = @import("./data.zig");

pub fn main() void {
    c.taca_windowSetTitle("Taca-Simplified WebGPU");
    const vertex_attributes = [_]c.WGPUVertexAttribute{
        .{
            .format = c.WGPUVertexFormat_Float32x3,
            .offset = 0,
            .shaderLocation = 0,
        },
        .{
            .format = c.WGPUVertexFormat_Float32x3,
            .offset = d.vertex_norm_offset * @sizeOf(f32),
            .shaderLocation = 1,
        },
        .{
            .format = c.WGPUVertexFormat_Float32x3,
            .offset = d.vertex_color_offset * @sizeOf(f32),
            .shaderLocation = 2,
        },
    };
    c.taca_gpuInit(&c.taca_GpuConfig{
        .indexFormat = c.WGPUIndexFormat_Uint16,
        .uniformSize = @sizeOf(Uniforms),
        .vertexBufferLayout = &c.WGPUVertexBufferLayout{
            .arrayStride = d.vertex_stride,
            .stepMode = c.WGPUVertexStepMode_Vertex,
            .attributeCount = vertex_attributes.len,
            .attributes = &vertex_attributes,
        },
        .wgsl = @embedFile("./shader.opt.wgsl"),
    });
}

const Uniforms = extern struct {
    projection: a.Mat4,
    view: a.Mat4,
    time: f32,
    position: a.Vec3,
};
