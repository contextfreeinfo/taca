const a = @import("./zalgebra/main.zig");
const c = @cImport({
    @cInclude("taca.h");
});
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
    _ = c.taca_gpu_shaderCreate(@embedFile("./shader-no-texture.opt.wgsl"));
    global_state = .{
        .index_buffer = c.taca_gpu_indexBufferCreate(
            @sizeOf(@TypeOf(d.index_data)),
            &d.index_data,
            c.WGPUIndexFormat_Uint16,
            c.taca_gpu_vertexBufferCreate(
                @sizeOf(@TypeOf(d.point_data)),
                &d.point_data,
                &c.WGPUVertexBufferLayout{
                    .arrayStride = d.vertex_stride,
                    .stepMode = c.WGPUVertexStepMode_Vertex,
                    .attributeCount = vertex_attributes.len,
                    .attributes = &vertex_attributes,
                },
            ),
        ),
        .position = a.Vec3.zero(),
        .projection = buildPerspective(c.taca_windowInnerSize()),
        .time = 0,
        .uniform_buffer = c.taca_gpu_uniformBufferCreate(@sizeOf(Uniforms)),
        // TODO Include linear algebra and perspective library? Too slow?
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
    c.taca_gpu_bufferWrite(state.uniform_buffer, &uniforms);
    c.taca_gpu_draw(state.index_buffer);
    c.taca_gpu_present();
}

fn windowResize(state: *State) void {
    const size = c.taca_windowInnerSize();
    if (size.x > 0 and size.y > 0) {
        state.projection = buildPerspective(size);
    }
}

const State = struct {
    index_buffer: c.taca_gpu_Buffer,
    position: a.Vec3,
    projection: a.Mat4,
    time: f32,
    uniform_buffer: c.taca_gpu_Buffer,
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
            .{ 1, 0, 0, 0 },
            .{ 0, aspect, 0, 0 },
            .{ 0, 0, far * divider, -far * near * divider },
            .{ 0, 0, 1 / focal_length, 0 },
        },
    }).transpose();
    // return a.perspective(45.0, aspect, 0.1, 100.0);
    return perspective;
}
