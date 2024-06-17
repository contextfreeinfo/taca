#![allow(non_snake_case)]

use miniquad::{BufferSource, BufferType, BufferUsage};

#[cfg(not(target_arch = "wasm32"))]
use wasmer::ValueType;

use crate::{platform::Platform, shaders::Shader};

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
pub struct ExternPipelineInfo {
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

pub fn new_buffer(platform: &mut Platform, typ: u32, usage: u32, buffer: &[u8], item_size: usize) {
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
    crate::wasmic::print(&format!("{buffer:?}"));
    platform.context.0.new_buffer(typ, usage, source);
}

pub fn new_pipeline(platform: &mut Platform, info: PipelineInfo) {
    platform.shaders[info.vertex.shader as usize - 1]
        .to_glsl(naga::ShaderStage::Vertex, info.vertex.entry_point);
    platform.shaders[info.fragment.shader as usize - 1]
        .to_glsl(naga::ShaderStage::Fragment, info.fragment.entry_point);
}

pub fn new_shader(platform: &mut Platform, bytes: &[u8]) -> u32 {
    platform.shaders.push(Shader::new(bytes));
    platform.shaders.len() as u32
}
