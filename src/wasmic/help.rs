#![allow(non_snake_case)]

use miniquad::{
    window, BufferId, BufferLayout, BufferSource, BufferType, BufferUsage, PipelineParams,
    ShaderMeta, ShaderSource, UniformBlockLayout, VertexFormat,
};

#[cfg(not(target_arch = "wasm32"))]
use wasmer::ValueType;

use crate::{
    platform::{Platform, RenderingContext},
    shaders::Shader,
};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Bindings {
    pub vertex_buffers: Vec<u32>,
    pub index_buffer: u32,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(ValueType))]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct BufferSlice {
    pub ptr: u32,
    pub size: u32,
    pub item_size: u32,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(ValueType))]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ExternBindings {
    pub vertex_buffers: Span,
    pub index_buffer: u32,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(ValueType))]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ExternPipelineInfo {
    pub attributes: Span,
    pub fragment: ExternPipelineShaderInfo,
    pub vertex: ExternPipelineShaderInfo,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(ValueType))]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ExternPipelineShaderInfo {
    pub entry_point: Span,
    pub shader: u32,
}

#[derive(Clone, Debug)]
pub struct PipelineInfo {
    pub attributes: Vec<VertexAttribute>,
    pub fragment: PipelineShaderInfo,
    pub vertex: PipelineShaderInfo,
}

#[derive(Clone, Debug)]
pub struct PipelineShaderInfo {
    pub entry_point: String,
    pub shader: u32,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(ValueType))]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Span {
    pub ptr: u32,
    pub len: u32,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(ValueType))]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct VertexAttribute {
    pub format: u32,
    pub buffer_index: u32,
}

fn value_to_vertex_format(value: u32) -> VertexFormat {
    match value {
        0 => VertexFormat::Float1,
        1 => VertexFormat::Float2,
        2 => VertexFormat::Float3,
        3 => VertexFormat::Float4,
        4 => VertexFormat::Byte1,
        5 => VertexFormat::Byte2,
        6 => VertexFormat::Byte3,
        7 => VertexFormat::Byte4,
        8 => VertexFormat::Short1,
        9 => VertexFormat::Short2,
        10 => VertexFormat::Short3,
        11 => VertexFormat::Short4,
        12 => VertexFormat::Int1,
        13 => VertexFormat::Int2,
        14 => VertexFormat::Int3,
        15 => VertexFormat::Int4,
        16 => VertexFormat::Mat4,
        _ => panic!(),
    }
}

pub fn apply_bindings(platform: &mut Platform, context: u32, bindings: Bindings) {
    let vertex_buffers: Vec<BufferId> = bindings
        .vertex_buffers
        .iter()
        .map(|buf| platform.buffer_ids[*buf as usize - 1])
        .collect();
    let bindings = miniquad::Bindings {
        vertex_buffers,
        index_buffer: platform.buffer_ids[bindings.index_buffer as usize - 1],
        images: vec![],
    };
    platform.contexts[context as usize - 1]
        .0
        .apply_bindings(&bindings);
}

pub fn apply_pipeline(platform: &mut Platform, context: u32, pipeline: u32) {
    platform.contexts[context as usize - 1]
        .0
        .apply_pipeline(&platform.pipelines[pipeline as usize - 1]);
}

pub fn begin_pass(platform: &mut Platform, context: u32) {
    platform.contexts[context as usize - 1]
        .0
        .begin_default_pass(Default::default());
}

pub fn draw(
    platform: &mut Platform,
    context: u32,
    item_begin: i32,
    item_count: i32,
    instance_count: i32,
) {
    platform.contexts[context as usize - 1]
        .0
        .draw(item_begin, item_count, instance_count);
}

pub fn commit_frame(platform: &mut Platform, context: u32) {
    platform.contexts[context as usize - 1].0.commit_frame();
}

pub fn end_pass(platform: &mut Platform, context: u32) {
    platform.contexts[context as usize - 1].0.end_render_pass();
}

pub fn new_buffer(
    platform: &mut Platform,
    context: u32,
    typ: u32,
    usage: u32,
    buffer: &[u8],
    item_size: usize,
) -> u32 {
    let typ = match typ {
        0 => BufferType::VertexBuffer,
        1 => BufferType::IndexBuffer,
        _ => panic!(),
    };
    let usage = match usage {
        0 => BufferUsage::Immutable,
        1 => BufferUsage::Dynamic,
        2 => BufferUsage::Stream,
        _ => panic!(),
    };
    let source = unsafe { BufferSource::pointer(buffer.as_ptr(), buffer.len(), item_size) };
    // crate::wasmic::print(&format!("{buffer:?}"));
    let buffer_id = platform.contexts[context as usize - 1]
        .0
        .new_buffer(typ, usage, source);
    platform.buffer_ids.push(buffer_id);
    platform.buffer_ids.len() as u32
}

pub fn new_pipeline(platform: &mut Platform, context: u32, info: PipelineInfo) -> u32 {
    let context = &mut platform.contexts[context as usize - 1];
    // TODO Metal for Mac.
    let vertex = platform.shaders[info.vertex.shader as usize - 1]
        .to_glsl(naga::ShaderStage::Vertex, info.vertex.entry_point);
    let fragment = platform.shaders[info.fragment.shader as usize - 1]
        .to_glsl(naga::ShaderStage::Fragment, info.fragment.entry_point);
    let shader = context
        .0
        .new_shader(
            ShaderSource::Glsl {
                vertex: &vertex,
                fragment: &fragment,
            },
            ShaderMeta {
                images: vec![],
                uniforms: UniformBlockLayout { uniforms: vec![] },
            },
        )
        .unwrap();
    let attributes: Vec<miniquad::VertexAttribute> = info
        .attributes
        .iter()
        .enumerate()
        .map(|(index, attr)| miniquad::VertexAttribute {
            name: &ATTRIBUTE_NAMES[index],
            format: value_to_vertex_format(attr.format),
            buffer_index: attr.buffer_index as usize,
        })
        .collect();
    let buffer_count = info
        .attributes
        .iter()
        .map(|attr| attr.buffer_index)
        .max()
        .unwrap_or(0)
        + 1;
    let pipeline = context.0.new_pipeline(
        &vec![BufferLayout::default(); buffer_count as usize],
        &attributes,
        shader,
        PipelineParams::default(),
    );
    platform.pipelines.push(pipeline);
    platform.pipelines.len() as u32
}

// Required to be 'static by miniquad.
const ATTRIBUTE_NAMES: [&str; 10] = [
    "_p2vs_location0",
    "_p2vs_location1",
    "_p2vs_location2",
    "_p2vs_location3",
    "_p2vs_location4",
    "_p2vs_location5",
    "_p2vs_location6",
    "_p2vs_location7",
    "_p2vs_location8",
    "_p2vs_location9",
];

pub fn new_rendering_context(platform: &mut Platform) -> u32 {
    platform
        .contexts
        .push(RenderingContext(window::new_rendering_backend()));
    platform.contexts.len() as u32
}

pub fn new_shader(platform: &mut Platform, bytes: &[u8]) -> u32 {
    platform.shaders.push(Shader::new(bytes));
    platform.shaders.len() as u32
}
