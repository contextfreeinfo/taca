#![allow(non_snake_case)]

use miniquad::{BufferSource, BufferType, BufferUsage, RenderingBackend};
#[cfg(not(target_arch = "wasm32"))]
use wasmer::ValueType;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(not(target_arch = "wasm32"), derive(ValueType))]
#[repr(C)]
pub struct BufferSlice {
    pub ptr: u32,
    pub size: u32,
    pub item_size: u32,
}

pub fn new_buffer(
    context: &mut Box<dyn RenderingBackend>,
    typ: u32,
    usage: u32,
    buffer: &[u8],
    item_size: usize,
) {
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
    context.new_buffer(typ, usage, source);
}
