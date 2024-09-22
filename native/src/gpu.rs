use std::io::Cursor;

use bytemuck::PodCastError;
use image::{DynamicImage, ImageError, ImageReader};
use naga::{
    front::spv,
    valid::{Capabilities, ValidationFlags, Validator},
    ScalarKind, VectorSize,
};
use wasmer::ValueType;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferDescriptor, BufferUsages, CommandEncoder, MultisampleState, RenderPipelineDescriptor,
    ShaderModule, ShaderModuleDescriptor, ShaderSource, SurfaceTexture, TextureFormat, TextureView,
    TextureViewDescriptor, VertexFormat,
};

use crate::{
    app::System,
    display::{MaybeGraphics, UserEvent},
};

#[derive(Debug)]
pub struct Binding {
    // Need a different bind group per pipeline.
    pub bind_groups: Vec<wgpu::BindGroup>,
    pub index_buffer: u32,
    // TODO textures
    pub vertex_buffers: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Bindings<'a> {
    pub vertex_buffers: &'a [u32],
    pub index_buffer: u32,
}

#[derive(Clone, Copy, Debug, ValueType)]
#[repr(C)]
pub struct BufferSlice {
    pub ptr: u32,
    pub size: u32,
}

pub struct Buffer {
    pub buffer: wgpu::Buffer,
    pub usage: BufferUsages,
}

#[derive(Clone, Copy, Debug, ValueType)]
#[repr(C)]
pub struct ExternBindings {
    pub vertex_buffers: Span,
    pub index_buffer: u32,
}

#[derive(Clone, Copy, Debug, ValueType)]
#[repr(C)]
pub struct ExternPipelineInfo {
    pub depth_test: bool,
    pub fragment: ExternPipelineShaderInfo,
    pub vertex: ExternPipelineShaderInfo,
    pub vertex_attributes: Span,
    pub vertex_buffers: Span,
}

#[derive(Clone, Copy, Debug, ValueType)]
#[repr(C)]
pub struct ExternPipelineShaderInfo {
    pub entry_point: Span,
    pub shader: u32,
}

#[derive(Debug)]
pub struct Pipeline {
    // pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group_index: usize,
    pub bind_groups: Vec<PipelineBindGroup>,
    pub pipeline: wgpu::RenderPipeline,
}

#[derive(Debug)]
pub struct PipelineBindGroup {
    pub bind_group: wgpu::BindGroup,
    // TODO Textures
    // TODO Do we also need to link from texture to pipeline bind group???
    // TODO Do new need to expose bind groups to Taca?
    // TODO But seems like buffers and textures are likely related?
    // TODO Include buffers, textures, and uniforms all together as bundles?
    pub uniform_buffer: wgpu::Buffer,
}

#[derive(Clone, Debug, Default)]
pub struct PipelineInfo {
    pub depth_test: bool,
    pub fragment: PipelineShaderInfo,
    pub vertex: PipelineShaderInfo,
    pub vertex_attributes: Vec<VertexAttribute>,
    pub vertex_buffers: Vec<VertexBufferInfo>,
}

#[derive(Clone, Debug, Default)]
pub struct PipelineShaderInfo {
    pub entry_point: String,
    pub shader: u32,
}

pub struct RenderFrame {
    pub bound: bool,
    pub buffered: bool,
    pub encoder: CommandEncoder,
    pub frame: SurfaceTexture,
    pub pass: Option<wgpu::RenderPass<'static>>,
    pub pipeline: usize,
    pub view: TextureView,
}

pub struct Shader {
    compiled: ShaderModule,
    // info: naga::valid::ModuleInfo,
    module: naga::Module,
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
    pub shader_location: u32,
    pub value_offset: u32,
}

#[derive(Clone, Copy, Debug, ValueType)]
#[repr(C)]
pub struct VertexBufferInfo {
    first_attribute: u32,
    step: u32,
    stride: u32,
}

/// Like wgpu::VertexBufferLayout except with a Vec of attributes.
#[derive(Clone, Debug)]
pub struct VertexBufferLayout {
    array_stride: wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode,
    attributes: Vec<wgpu::VertexAttribute>,
}

#[derive(Debug)]
pub struct TextureData {
    #[allow(unused)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

pub fn bindings_apply(system: &mut System, bindings: Bindings) {
    pass_ensure(system);
    let Some(frame) = system.frame.as_mut() else {
        return;
    };
    let Some(pass) = &mut frame.pass else {
        return;
    };
    pass.set_index_buffer(
        system.buffers[bindings.index_buffer as usize - 1]
            .buffer
            .slice(..),
        wgpu::IndexFormat::Uint16,
    );
    for (index, buffer) in bindings.vertex_buffers.iter().enumerate() {
        pass.set_vertex_buffer(
            index as u32,
            system.buffers[*buffer as usize - 1].buffer.slice(..),
        );
    }
    frame.buffered = true;
}

pub fn bound_ensure(system: &mut System) {
    pass_ensure(system);
    let Some(frame) = system.frame.as_mut() else {
        return;
    };
    if frame.bound {
        return;
    }
    // We apparently need something bound, and neither size 0 nor 1 works.
    uniforms_apply(system, &[0; 4]);
}

pub fn buffer_update(system: &mut System, buffer: u32, bytes: &[u8], offset: u32) {
    pipelined_ensure(system);
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        panic!();
    };
    let buffer = &system.buffers[buffer as usize - 1].buffer;
    gfx.queue
        .write_buffer(buffer, offset as wgpu::BufferAddress, bytes);
}

pub fn buffered_ensure(system: &mut System) {
    pass_ensure(system);
    let Some(frame) = system.frame.as_mut() else {
        return;
    };
    if frame.buffered {
        return;
    }
    let index = system
        .buffers
        .iter()
        .position(|it| it.usage.contains(BufferUsages::INDEX))
        .unwrap();
    let vertex = system
        .buffers
        .iter()
        .position(|it| it.usage.contains(BufferUsages::VERTEX))
        .unwrap();
    let bindings = Bindings {
        vertex_buffers: &[vertex as u32 + 1],
        index_buffer: index as u32 + 1,
    };
    bindings_apply(system, bindings);
}

pub fn create_buffer(system: &mut System, contents: Option<&[u8]>, size: u32, typ: u32) {
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        panic!();
    };
    let usage = match contents {
        Some(_) => BufferUsages::empty(),
        None => BufferUsages::COPY_DST,
    } | match typ {
        0 => BufferUsages::VERTEX,
        1 => BufferUsages::INDEX,
        _ => panic!(),
    };
    let buffer = match contents {
        Some(contents) => gfx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents,
            usage,
        }),
        None => gfx.device.create_buffer(&BufferDescriptor {
            label: None,
            size: size as wgpu::BufferAddress,
            usage,
            mapped_at_creation: false,
        }),
    };
    // dbg!(&buffer);
    // dbg!(&contents);
    system.buffers.push(Buffer { buffer, usage });
}

pub fn create_pipeline(system: &mut System, info: PipelineInfo) {
    let (depth_write_enabled, depth_compare) = match info.depth_test {
        true => (true, wgpu::CompareFunction::Less),
        false => (false, wgpu::CompareFunction::Always),
    };
    fn choose_entry<'a>(entry: String, default: &'a str) -> String {
        match entry.as_str() {
            "" => default.to_string(),
            _ => entry,
        }
    }
    fn choose_shader(shader: u32, other: u32) -> u32 {
        match shader {
            0 => match other {
                0 => 1,
                _ => other,
            },
            _ => shader,
        }
    }
    let fragment_entry_point = choose_entry(info.fragment.entry_point, FRAGMENT_ENTRY_DEFAULT);
    let fragment_shader = choose_shader(info.fragment.shader, info.vertex.shader);
    let vertex_entry_point = choose_entry(info.vertex.entry_point, VERTEX_ENTRY_DEFAULT);
    let vertex_shader = choose_shader(info.vertex.shader, info.fragment.shader);
    let info = PipelineInfo {
        fragment: PipelineShaderInfo {
            entry_point: fragment_entry_point.clone(),
            shader: fragment_shader,
        },
        vertex: PipelineShaderInfo {
            entry_point: vertex_entry_point.clone(),
            shader: vertex_shader,
        },
        ..info
    };
    let Some(buffers) = vertex_buffer_layouts_build(system, info) else {
        return;
    };
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        return;
    };
    let device = &gfx.device;
    let fragment_shader = &system.shaders[fragment_shader as usize - 1];
    let vertex_shader = &system.shaders[vertex_shader as usize - 1];
    let min_binding_size = uniforms_binding_size_find(vertex_shader);
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size,
            },
            count: None,
        }],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    // dbg!(&attr_info);
    let vertex_buffer_layout: Vec<_> = buffers
        .iter()
        .map(|buffer| wgpu::VertexBufferLayout {
            array_stride: buffer.array_stride,
            step_mode: buffer.step_mode,
            attributes: &buffer.attributes,
        })
        .collect();
    // dbg!(&vertex_buffer_layout);
    // let surface_formats = gfx.surface.get_capabilities(&gfx.adapter).formats;
    // dbg!(&surface_formats);
    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader.compiled,
            entry_point: &vertex_entry_point,
            compilation_options: Default::default(),
            buffers: &vertex_buffer_layout,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_shader.compiled,
            entry_point: &fragment_entry_point,
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: TextureFormat::Bgra8Unorm,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled,
            depth_compare,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: MultisampleState::default(),
        multiview: None,
        cache: None,
    });
    system.pipelines.push(Pipeline {
        bind_group_index: 0,
        bind_groups: vec![],
        pipeline,
    });
}

pub fn frame_commit(system: &mut System) {
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        return;
    };
    let Some(mut frame) = system.frame.take() else {
        return;
    };
    // First finish any pass.
    frame.pass.take();
    // Then commit frame.
    let command_buffer = frame.encoder.finish();
    // dbg!(&command_buffer);
    gfx.queue.submit([command_buffer]);
    frame.frame.present();
    if let Some(text) = &system.text {
        let mut text = text.lock().unwrap();
        text.renderer_index = 0;
    }
}

pub fn image_decode(
    bytes: Vec<u8>,
    event_loop_proxy: &winit::event_loop::EventLoopProxy<UserEvent>,
) {
    let cursor = Cursor::new(bytes);
    let result = ImageReader::new(cursor)
        .with_guessed_format()
        .map_err(|err| ImageError::IoError(err))
        .and_then(|it| it.decode());
    event_loop_proxy
        .send_event(UserEvent::ImageDecoded(result))
        .unwrap();
}

pub fn image_to_texture(image: DynamicImage) {
    // TODO Also need the texture index!
    dbg!(image.width(), image.height());
}

pub fn pass_ensure(system: &mut System) {
    pass_ensure_load(system, wgpu::LoadOp::Clear(wgpu::Color::BLACK));
}

pub fn pass_ensure_load(system: &mut System, load: wgpu::LoadOp<wgpu::Color>) {
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        return;
    };
    if system.frame.is_none() {
        let frame = gfx.surface.get_current_texture().unwrap();
        let view_descriptor = TextureViewDescriptor {
            format: Some(TextureFormat::Bgra8Unorm),
            ..Default::default()
        };
        let view = frame.texture.create_view(&view_descriptor);
        let encoder = gfx.device.create_command_encoder(&Default::default());
        system.frame = Some(RenderFrame {
            bound: false,
            buffered: false,
            encoder,
            frame,
            pass: None,
            pipeline: 0,
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
    let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load,
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &gfx.depth_texture.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        ..Default::default()
    });
    frame.pass = Some(pass.forget_lifetime());
    for pipeline in &mut system.pipelines {
        pipeline.bind_group_index = 0;
    }
}

pub fn pipeline_apply(system: &mut System, pipeline: u32) {
    pipeline_ensure(system);
    pass_ensure(system);
    let pipeline_ind = pipeline as usize;
    let Some(pipeline) = system.pipelines.get(pipeline_ind - 1) else {
        return;
    };
    let Some(frame) = system.frame.as_mut() else {
        return;
    };
    let Some(pass) = &mut frame.pass else {
        return;
    };
    frame.pipeline = pipeline_ind;
    pass.set_pipeline(&pipeline.pipeline);
}

fn pipeline_ensure(system: &mut System) {
    if !system.pipelines.is_empty() {
        return;
    }
    create_pipeline(system, Default::default());
}

pub fn pipelined_ensure(system: &mut System) {
    let needed = match system.frame.as_ref() {
        Some(frame) => frame.pipeline == 0,
        _ => true,
    };
    if needed {
        pipeline_apply(system, 1);
    }
}

pub fn shader_create(system: &mut System, bytes: &[u8]) -> Shader {
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        panic!();
    };
    let module = spv::parse_u8_slice(bytes, &Default::default()).unwrap();
    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::empty());
    let _info = validator
        .validate(&module)
        .expect("Shader validation failed");
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
    let compiled = gfx.device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::SpirV(std::borrow::Cow::Borrowed(spirv)),
    });
    Shader {
        compiled,
        // info,
        module,
    }
}

fn step_mode_translate(step: u32) -> wgpu::VertexStepMode {
    match step {
        0 => wgpu::VertexStepMode::Vertex,
        _ => wgpu::VertexStepMode::Instance,
    }
}

pub fn uniforms_apply<'a>(system: &'a mut System, bytes: &[u8]) {
    pipelined_ensure(system);
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        panic!();
    };
    let Some(frame) = system.frame.as_mut() else {
        return;
    };
    let device = &gfx.device;
    let pipeline = &mut system.pipelines[frame.pipeline - 1];
    // Make a new bind group if we need one.
    // TODO Once we have textures involved, will we need to group all that together???
    if pipeline.bind_group_index >= pipeline.bind_groups.len() {
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: bytes.len() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let layout = pipeline.pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            // TODO Textures also go in here!
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: None,
        });
        pipeline.bind_groups.push(PipelineBindGroup {
            bind_group,
            uniform_buffer,
        });
        // dbg!(pipeline.bind_groups.len());
    }
    let bind_group = &pipeline.bind_groups[pipeline.bind_group_index];
    pipeline.bind_group_index += 1;
    gfx.queue.write_buffer(&bind_group.uniform_buffer, 0, bytes);
    let Some(pass) = &mut frame.pass else {
        return;
    };
    pass.set_bind_group(0, &bind_group.bind_group, &[]);
    frame.bound = true;
}

fn uniforms_binding_size_find(shader: &Shader) -> Option<wgpu::BufferSize> {
    let types = &shader.module.types;
    for var in shader.module.global_variables.iter() {
        let var = var.1;
        if var.space == naga::AddressSpace::Uniform {
            let ty = &types[var.ty];
            let size = ty.inner.size(shader.module.to_ctx()) as u64;
            // println!("{var:?}: {ty:?}");
            return wgpu::BufferSize::new(size);
        }
    }
    None
}

fn vertex_buffer_layouts_build(
    system: &System,
    info: PipelineInfo,
) -> Option<Vec<VertexBufferLayout>> {
    let shader = system.shaders.get(info.vertex.shader as usize - 1)?;
    let entry = shader
        .module
        .entry_points
        .iter()
        .find(|it| it.name == info.vertex.entry_point)
        .unwrap();
    let types = &shader.module.types;
    let mut layouts = vec![];
    let mut layout = VertexBufferLayout {
        array_stride: 0,
        step_mode: match () {
            _ if info.vertex_buffers.is_empty() || info.vertex_buffers[0].first_attribute > 0 => {
                wgpu::VertexStepMode::Vertex
            }
            _ => step_mode_translate(info.vertex_buffers[0].step),
        },
        attributes: vec![],
    };
    // TODO Match on location in case order varies? Require order?
    let mut total_attrs = 0;
    for arg in entry.function.arguments.iter() {
        // dbg!(arg);
        let Some(naga::Binding::Location { location, .. }) = arg.binding else {
            continue;
        };
        let format = vertex_format_from_naga_type(&types[arg.ty].inner);
        // Find which buffer we're at.
        loop {
            let next_buffer_index = layouts.len() + 1;
            if next_buffer_index >= info.vertex_buffers.len() {
                // No buffer descriptions left, so stay here.
                break;
            }
            let next_buffer_info = info.vertex_buffers[next_buffer_index];
            if total_attrs < next_buffer_info.first_attribute {
                // We're not yet to the index starting the next buffer, so stay here.
                break;
            }
            if total_attrs > next_buffer_info.first_attribute {
                // Don't allow going backward.
                return None;
            }
            // We're at the next buffer.
            layouts.push(layout);
            layout = VertexBufferLayout {
                array_stride: 0,
                step_mode: step_mode_translate(next_buffer_info.step),
                attributes: vec![],
            };
            // But keep looping since technically it could be empty or something?
        }
        let attr = wgpu::VertexAttribute {
            format,
            offset: layout.array_stride,
            shader_location: location,
        };
        layout.attributes.push(attr);
        // TODO Align.
        layout.array_stride += format.size();
        total_attrs += 1;
    }
    layouts.push(layout);
    Some(layouts)
}

fn vertex_format_from_naga_type(type_inner: &naga::TypeInner) -> VertexFormat {
    match type_inner {
        naga::TypeInner::Scalar(naga::Scalar { kind, width }) => match (kind, width) {
            (ScalarKind::Sint, 4) => VertexFormat::Sint32,
            (ScalarKind::Uint, 4) => VertexFormat::Uint32,
            (ScalarKind::Float, 4) => VertexFormat::Float32,
            _ => todo!(),
        },
        naga::TypeInner::Vector {
            size,
            scalar: naga::Scalar { kind, width },
        } => match (kind, width, size) {
            (ScalarKind::Float, 4, VectorSize::Bi) => VertexFormat::Float32x2,
            (ScalarKind::Float, 4, VectorSize::Tri) => VertexFormat::Float32x3,
            (ScalarKind::Float, 4, VectorSize::Quad) => VertexFormat::Float32x4,
            _ => todo!(),
        },
        naga::TypeInner::Matrix { .. } => todo!(),
        naga::TypeInner::Atomic(_) => todo!(),
        naga::TypeInner::Pointer { .. } => todo!(),
        naga::TypeInner::ValuePointer { .. } => todo!(),
        naga::TypeInner::Array { .. } => todo!(),
        naga::TypeInner::Struct { .. } => todo!(),
        naga::TypeInner::Image { .. } => todo!(),
        naga::TypeInner::Sampler { .. } => todo!(),
        naga::TypeInner::AccelerationStructure => todo!(),
        naga::TypeInner::RayQuery => todo!(),
        naga::TypeInner::BindingArray { .. } => todo!(),
    }
}

const FRAGMENT_ENTRY_DEFAULT: &str = "fragment_main";
const VERTEX_ENTRY_DEFAULT: &str = "vertex_main";
