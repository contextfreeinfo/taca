pub const vertex_color_offset = 3;

pub const vertex_count = vertex_data.len / vertex_len;

pub const vertex_data_size = @sizeOf(@TypeOf(vertex_data));

pub const vertex_len = 6;

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

pub const index_data = [_]u16{
    // Base
    0, 1, 2,
    0, 2, 3,
    // Sides
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
    3, 0, 4,
};

pub const point_data = [_]f32{
    // x,    y,    z,   r,   g,   b
    // Base
    -0.5, -0.5, -0.3, 1.0, 0.0, 0.0,
    0.5,  -0.5, -0.3, 0.0, 1.0, 0.0,
    0.5,  0.5,  -0.3, 0.0, 0.0, 1.0,
    -0.5, 0.5,  -0.3, 1.0, 1.0, 0.0,
    // Tip
    0.0,  0.0,  0.5,  0.8, 0.8, 0.8,
};
