const a = @import("./zalgebra/main.zig");
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
    const vertex_buffer = c.taca_gpuVertexBufferInit();
    const index_buffer = c.taca_gpuIndexBufferInit(vertex_buffer);
    c.taca_gpuBufferWrite(vertex_buffer, @sizeOf(@TypeOf(d.point_data)), &d.point_data);
    c.taca_gpuBufferWrite(index_buffer, @sizeOf(@TypeOf(d.index_data)), &d.index_data);
    const size = c.taca_windowInnerSize();
    global_state = .{
        .index_buffer = index_buffer,
        .position = a.Vec3.zero(),
        .projection = buildPerspective(size),
        .size = size,
        .time = 0,
        .uniform_buffer = c.taca_gpuUniformBufferInit(),
        .vertex_buffer = vertex_buffer,
        .view = a.Mat4.identity()
            .translate(a.Vec3.new(0, 0, -2))
            .rotate(135, a.Vec3.new(1, 0, 0))
            .transpose(),
    };
    c.taca_windowListen(null, &global_state);
}

export fn windowListen(event_type: c.taca_WindowEventType, userdata: ?*anyopaque) void {
    const state = @ptrCast(*State, @alignCast(@alignOf(State), userdata));
    switch (event_type) {
        // c.taca_WindowEventType_Close => windowClose(state),
        // c.taca_WindowEventType_Key => keyPress(state),
        c.taca_WindowEventType_Redraw => windowRedraw(state),
        c.taca_WindowEventType_Resize => windowResize(state),
        // else => unreachable,
        else => {},
    }
}

fn windowRedraw(state: *State) void {
    state.time += 1.0 / 60.0;
    const uniforms = Uniforms{
        .projection = state.projection,
        .view = state.view,
        .time = state.time,
        .position = state.position,
    };
    c.taca_gpuBufferWrite(state.uniform_buffer, @sizeOf(Uniforms), &uniforms);
    c.taca_gpuDraw(state.index_buffer);
    c.taca_gpuPresent();
}

fn windowResize(state: *State) void {
    const size = c.taca_windowInnerSize();
    if (size.x > 0 and size.y > 0) {
        state.projection = buildPerspective(size);
        state.size = size;
    }
}

const State = struct {
    index_buffer: c.taca_GpuBuffer,
    position: a.Vec3,
    projection: a.Mat4,
    size: c.taca_Vec2,
    time: f32,
    uniform_buffer: c.taca_GpuBuffer,
    vertex_buffer: c.taca_GpuBuffer,
    view: a.Mat4,
};

var global_state: State = undefined;

const Uniforms = extern struct {
    projection: a.Mat4,
    view: a.Mat4,
    time: f32,
    position: a.Vec3,
};

fn buildPerspective(size: c.taca_Vec2) a.Mat4 {
    // The tutorial uses different calculations than zalgebra, and I'm not
    // getting what I want from zalgebra, so go with tutorial.
    const aspect = @intToFloat(f32, size.x) / @intToFloat(f32, size.y);
    const focal_length: f32 = 2;
    const near: f32 = 0.01;
    const far: f32 = 100;
    const divider: f32 = 1.0 / (focal_length * (far - near));
    const perspective = (a.Mat4{
        .data = .{
            .{1, 0, 0, 0},
            .{0, aspect, 0, 0},
            .{0, 0, far * divider, -far * near * divider},
            .{0, 0, 1 / focal_length, 0},
        },
    }).transpose();
    // return a.perspective(45.0, aspect, 0.1, 100.0);
    return perspective;
}
