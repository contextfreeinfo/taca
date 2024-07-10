use std::mem::transmute;

use bytemuck::PodCastError;
use wasmer::ValueType;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages, CommandEncoder, ShaderModule, ShaderModuleDescriptor, ShaderSource,
    SurfaceTexture, TextureView,
};

use crate::{app::System, display::MaybeGraphics};

#[derive(Clone, Copy, Debug, ValueType)]
#[repr(C)]
pub struct BufferSlice {
    pub ptr: u32,
    pub size: u32,
    pub item_size: u32,
}

pub struct Buffer {
    pub buffer: wgpu::Buffer,
    pub item_size: usize,
    pub usage: BufferUsages,
}

#[derive(Clone, Copy, Debug, ValueType)]
#[repr(C)]
pub struct ExternPipelineInfo {
    pub attributes: Span,
    pub fragment: ExternPipelineShaderInfo,
    pub vertex: ExternPipelineShaderInfo,
}

#[derive(Clone, Copy, Debug, ValueType)]
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

pub struct RenderFrame {
    pub encoder: CommandEncoder,
    pub frame: SurfaceTexture,
    pub pass: Option<wgpu::RenderPass<'static>>,
    pub view: TextureView,
}

#[derive(Clone, Copy, Debug, ValueType)]
#[repr(C)]
pub struct Span {
    pub ptr: u32,
    pub len: u32,
}

#[derive(Clone, Copy, Debug, ValueType)]
#[repr(C)]
pub struct VertexAttribute {
    pub format: u32,
    pub buffer_index: u32,
}

pub fn create_buffer(
    system: &mut System,
    contents: &[u8],
    typ: u32,
    _usage: u32,
    item_size: usize,
) {
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        panic!();
    };
    let usage = match typ {
        0 => BufferUsages::VERTEX,
        1 => BufferUsages::INDEX,
        _ => panic!(),
    };
    let buffer = gfx.device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents,
        usage,
    });
    system.buffers.push(Buffer {
        buffer,
        item_size,
        usage,
    });
}

pub fn create_pipeline(system: &mut System, info: PipelineInfo) {
    // let shader = &system.shaders[info.vertex.shader as usize];
}

pub fn end_pass(system: &mut System) {
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        return;
    };
    let Some(frame) = system.frame.as_mut() else {
        return;
    };
    if let Some(pass) = frame.pass.take() {
        drop(pass);
    }
}

pub fn ensure_pass(system: &mut System) {
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        return;
    };
    if system.frame.is_none() {
        let frame = gfx.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());
        let encoder = gfx.device.create_command_encoder(&Default::default());
        system.frame = Some(RenderFrame {
            encoder,
            frame,
            pass: None,
            view,
        });
    }
    let Some(frame) = system.frame.as_mut() else {
        panic!()
    };
    if frame.pass.is_some() {
        return;
    }
    let view = &frame.view;
    let encoder = &mut frame.encoder;
    let pass = unsafe { &mut *(encoder as *mut CommandEncoder) }.begin_render_pass(
        &wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: unsafe { &*(view as *const TextureView) },
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        },
    );
    frame.pass = Some(unsafe { transmute(pass) });
}

pub fn shader_create(system: &mut System, bytes: &[u8]) -> ShaderModule {
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        panic!();
    };
    let mut spirv_buffer = Vec::<u32>::new();
    let spirv: &[u32] = bytemuck::try_cast_slice(bytes).unwrap_or_else(|err| match err {
        PodCastError::AlignmentMismatch => {
            // Copy into an aligned buffer if not already aligned.
            for chunk in bytes.chunks_exact(4) {
                let word = u32::from_le_bytes(chunk.try_into().unwrap());
                spirv_buffer.push(word);
            }
            &spirv_buffer
        }
        _ => panic!(),
    });
    gfx.device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::SpirV(std::borrow::Cow::Borrowed(spirv)),
    })
}
