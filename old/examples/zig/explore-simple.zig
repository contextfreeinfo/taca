// Based largely on:
// https://github.com/eliemichel/LearnWebGPU-Code/blob/step033/main.cpp

const std = @import("std");
const a = @import("./zalgebra/main.zig");
const c = @cImport({
    @cInclude("taca.h");
});
const d = @import("./data.zig");

pub fn main() void {
    c.taca_windowSetTitle("Taca-Simplified WebGPU");
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = general_purpose_allocator.allocator();
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
    _ = c.taca_gpu_shaderCreate(@embedFile("./shader.opt.wgsl"));
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
        .uniform_buffer = c.taca_gpu_uniformBufferCreate(@sizeOf(Uniforms), 0),
        .velocity = a.Vec3.zero(),
        // TODO Include linear algebra and perspective library? Too slow?
        .view = a.Mat4.identity()
            .translate(a.Vec3.new(0, 0, -2))
            .rotate(135, a.Vec3.new(1, 0, 0))
            .transpose(),
    };
    createTexture(allocator);
    c.taca_windowListen(null, &global_state);
}

export fn windowListen(event_type: c.taca_WindowEventType, userdata: ?*anyopaque) void {
    const state: *State = @ptrCast(@alignCast(userdata));
    switch (event_type) {
        // c.taca_WindowEventType_Close => windowClose(state),
        c.taca_WindowEventType_Key => keyPress(state),
        c.taca_WindowEventType_Redraw => windowRedraw(state),
        c.taca_WindowEventType_Resize => windowResize(state),
        // else => unreachable,
        else => {},
    }
}

fn createTexture(allocator: std.mem.Allocator) void {
    const info = c.taca_gpu_TextureInfo{
        .format = c.WGPUTextureFormat_RGBA8Unorm,
        .binding = 1,
        .width = 256,
        .height = 256,
    };
    const image_texture_data = allocator.alloc(
        u8,
        4 * info.width * info.height,
    ) catch unreachable;
    for (0..info.width) |j| {
        for (0..info.height) |i| {
            const n = 4 * (i * info.width + j);
            image_texture_data[n + 0] = if ((i / 16) % 2 == (j / 16) % 2) 255 else 0;
            image_texture_data[n + 1] = if (((i - j) / 16) % 2 == 0) 255 else 0;
            image_texture_data[n + 2] = if (((i + j) / 16) % 2 == 0) 255 else 0;
            image_texture_data[n + 3] = 255;
        }
    }
    _ = c.taca_gpu_textureCreate(image_texture_data.ptr, &info);
}

fn keyPress(state: *State) void {
    const event = c.taca_keyEvent();
    const speed: f32 = if (event.pressed) 0.02 else 0;
    switch (event.code) {
        c.taca_KeyCode_Undefined => {},
        c.taca_KeyCode_Left => {
            updateSpeed(&state.velocity, 0, -1, speed);
        },
        c.taca_KeyCode_Up => {
            updateSpeed(&state.velocity, 1, 1, speed);
        },
        c.taca_KeyCode_Right => {
            updateSpeed(&state.velocity, 0, 1, speed);
        },
        c.taca_KeyCode_Down => {
            updateSpeed(&state.velocity, 1, -1, speed);
        },
        else => unreachable,
    }
}

fn updateSpeed(vec: *a.Vec3, index: usize, direction: f32, speed: f32) void {
    if (std.math.sign(vec.data[index]) == direction or speed != 0) {
        vec.data[index] = direction * speed;
    }
}

fn windowRedraw(state: *State) void {
    state.position = state.position.add(state.velocity);
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
    velocity: a.Vec3,
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
    const aspect = @as(f32, @floatFromInt(size.x)) / @as(f32, @floatFromInt(size.y));
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
