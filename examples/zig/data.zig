pub const vertex_color_offset = 2;

pub const vertex_count = vertex_data.len / vertex_len;

pub const vertex_data_size = @sizeOf(@TypeOf(vertex_data));

pub const vertex_len = 5;

pub const vertex_stride = vertex_len * @sizeOf(f32);

pub const vertex_data = [_]f32{
    // x,     y,   r,   g,   b
    -0.5,  -0.5, 1.0, 0.0, 0.0,
    0.5,   -0.5, 0.0, 1.0, 0.0,
    0.0,   0.5,  0.0, 0.0, 1.0,
    -0.55, -0.5, 1.0, 1.0, 0.0,
    -0.05, 0.5,  1.0, 0.0, 1.0,
    -0.55, 0.5,  0.0, 1.0, 1.0,
};
