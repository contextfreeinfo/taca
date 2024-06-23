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

fn vertex_format_to_value(format: VertexFormat) -> u32 {
    match format {
        VertexFormat::Float1 => 0,
        VertexFormat::Float2 => 1,
        VertexFormat::Float3 => 2,
        VertexFormat::Float4 => 3,
        VertexFormat::Byte1 => 4,
        VertexFormat::Byte2 => 5,
        VertexFormat::Byte3 => 6,
        VertexFormat::Byte4 => 7,
        VertexFormat::Short1 => 8,
        VertexFormat::Short2 => 9,
        VertexFormat::Short3 => 10,
        VertexFormat::Short4 => 11,
        VertexFormat::Int1 => 12,
        VertexFormat::Int2 => 13,
        VertexFormat::Int3 => 14,
        VertexFormat::Int4 => 15,
        VertexFormat::Mat4 => 16,
    }
}

pub fn apply_bindings(platform: &mut Platform, context: u32, bindings: Bindings) {
    attempt_pipeline(platform, context);
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
    let context = &mut platform.contexts[context as usize - 1];
    context.backend.apply_bindings(&bindings);
    context.pass.bound = true;
}

pub fn apply_pipeline(platform: &mut Platform, context: u32, pipeline: u32) {
    let context = &mut platform.contexts[context as usize - 1];
    context.apply_pipeline(&platform.pipelines[pipeline as usize - 1]);
}

pub fn apply_uniforms(platform: &mut Platform, context: u32, uniforms: &[u8]) {
    attempt_pipeline(platform, context);
    let context = &mut platform.contexts[context as usize - 1];
    context
        .backend
        .apply_uniforms_from_bytes(uniforms.as_ptr(), uniforms.len());
}

fn attempt_bindings(platform: &mut Platform, context: u32) {
    attempt_pipeline(platform, context);
    let context = &mut platform.contexts[context as usize - 1];
    if !context.pass.bound
        && platform.index_buffer_ids.len() == 1
        && platform.vertex_buffer_ids.len() == 1
    {
        let bindings = miniquad::Bindings {
            vertex_buffers: platform.vertex_buffer_ids.clone(),
            index_buffer: *platform.index_buffer_ids.first().unwrap(),
            images: vec![],
        };
        context.backend.apply_bindings(&bindings);
        context.pass.bound = true;
    }
}

pub fn attempt_pipeline(platform: &mut Platform, context: u32) {
    if platform.pipelines.is_empty() && platform.shaders.len() == 1 {
        let shader = platform.shaders.first().unwrap();
        let entry = shader.vertex_entries.first().unwrap();
        let info = PipelineInfo {
            attributes: entry
                .attributes
                .iter()
                .map(|format| VertexAttribute {
                    // Backtracking to int here because that's what we expect in
                    // this helper.
                    format: vertex_format_to_value(*format),
                    buffer_index: 0,
                })
                .collect(),
            fragment: PipelineShaderInfo {
                entry_point: "fs_main".into(),
                shader: 1,
            },
            vertex: PipelineShaderInfo {
                entry_point: "vs_main".into(),
                shader: 1,
            },
        };
        new_pipeline(platform, context, info);
    }
    let context = &mut platform.contexts[context as usize - 1];
    if !context.pass.pipelined && platform.pipelines.len() == 1 {
        context.apply_pipeline(platform.pipelines.first().unwrap());
    }
}

pub fn begin_pass(platform: &mut Platform, context: u32) {
    let context = &mut platform.contexts[context as usize - 1];
    context.begin_pass();
}

pub fn build_title(platform: &Platform) -> String {
    platform
        .title
        .as_ref()
        .map_or_else(|| "Taca".into(), |it| it.clone())
}

pub fn commit_frame(platform: &mut Platform, context: u32) {
    let context = &mut platform.contexts[context as usize - 1];
    context.commit_frame();
}

pub fn draw(
    platform: &mut Platform,
    context: u32,
    item_begin: i32,
    item_count: i32,
    instance_count: i32,
) {
    attempt_bindings(platform, context);
    let context = &mut platform.contexts[context as usize - 1];
    context.backend.draw(item_begin, item_count, instance_count);
}

pub fn end_pass(platform: &mut Platform, context: u32) {
    let context = &mut platform.contexts[context as usize - 1];
    context.end_pass();
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
    let buffer_id = platform.contexts[context as usize - 1]
        .backend
        .new_buffer(typ, usage, source);
    match typ {
        BufferType::VertexBuffer => &mut platform.vertex_buffer_ids,
        BufferType::IndexBuffer => &mut platform.index_buffer_ids,
    }
    .push(buffer_id);
    platform.buffer_ids.push(buffer_id);
    platform.buffer_ids.len() as u32
}

pub fn new_pipeline(platform: &mut Platform, context: u32, info: PipelineInfo) -> u32 {
    let context = &mut platform.contexts[context as usize - 1];
    // TODO Metal for Mac.
    // TODO Force single shader????
    // TODO Track doubling up across stages???
    let shader = &platform.shaders[info.vertex.shader as usize - 1];
    let vertex = shader.to_glsl(naga::ShaderStage::Vertex, info.vertex.entry_point);
    let fragment = platform.shaders[info.fragment.shader as usize - 1]
        .to_glsl(naga::ShaderStage::Fragment, info.fragment.entry_point);
    let shader = context
        .backend
        .new_shader(
            ShaderSource::Glsl {
                vertex: &vertex,
                fragment: &fragment,
            },
            ShaderMeta {
                images: vec![],
                uniforms: UniformBlockLayout {
                    uniforms: shader.uniforms.clone(),
                },
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
    let pipeline = context.backend.new_pipeline(
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
    platform.contexts.push(RenderingContext {
        backend: window::new_rendering_backend(),
        pass: Default::default(),
    });
    platform.contexts.len() as u32
}

pub fn new_shader(platform: &mut Platform, bytes: &[u8]) -> u32 {
    platform.shaders.push(Shader::new(bytes));
    platform.shaders.len() as u32
}
