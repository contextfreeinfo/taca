use std::{
    ffi::CString,
    ptr::{null, null_mut},
    sync::{Arc, Mutex},
};

use crate::{
    system::{System, WGPUBuffer, WGPURenderPipeline, WGPUTextureView},
    webgpu::{
        read_cstring, wgpu_adapter_ensure_device_simple, wgpu_adapter_get_limits_simple,
        wgpu_device_create_shader_module_simple, wgpu_device_ensure_command_encoder_simple,
        wgpu_device_ensure_queue_simple, wgpu_device_ensure_uncaptured_error_callback_simple,
        wgpu_ensure_command_encoder_finish_simple, wgpu_ensure_instance_simple,
        wgpu_ensure_queue_submit_simple, wgpu_instance_ensure_adapter_simple,
        wgpu_instance_ensure_surface_simple, wgpu_surface_get_preferred_format_simple,
        WasmWGPUVertexBufferLayout,
    },
};
use wasmer::{FunctionEnvMut, WasmPtr};
use wgpu_native::{
    device::{wgpuBufferDrop, wgpuDeviceDrop, wgpuRenderPipelineDrop},
    native,
};

pub struct GpuBuffer {
    buffer: WGPUBuffer,
    data: Vec<u8>,
    detail: GpuBufferDetail,
    size: usize,
}

pub enum GpuBufferDetail {
    Index {
        format: native::WGPUIndexFormat,
        vertex: Arc<Mutex<GpuBuffer>>,
    },
    Uniform,
    Vertex {
        layout: WgpuVertexBufferLayout,
    },
}

pub struct WgpuVertexBufferLayout {
    pub array_stride: u64,
    pub step_mode: native::WGPUVertexStepMode,
    pub attributes: Vec<native::WGPUVertexAttribute>,
}

#[derive(Default)]
pub struct SimpleGpu {
    buffers: Vec<Arc<Mutex<GpuBuffer>>>,
    pipeline: WGPURenderPipeline,
    shaders: Vec<CString>,
    texture_view: WGPUTextureView,
}

// taca_EXPORT void taca_gpuBufferWrite(taca_GpuBuffer buffer, const void* data);
pub fn taca_gpu_buffer_write(mut env: FunctionEnvMut<System>, buffer: u32, data: u32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let mut buffer = system.gpu.buffers[buffer as usize].lock().unwrap();
    let data = WasmPtr::<u8>::new(data)
        .slice(&view, buffer.data.len() as u32)
        .unwrap()
        .read_to_vec()
        .unwrap();
    if !buffer.buffer.0.is_null() {
        unsafe {
            wgpuBufferDrop(buffer.buffer.0);
        }
    }
    buffer.buffer.0 = null_mut();
    buffer.data = data;
}

// From https://stackoverflow.com/a/68027744/2748187
#[macro_export]
macro_rules! extract_enum_value {
    ($value:expr, $pattern:pat => $extracted_value:expr) => {
        match $value {
            $pattern => $extracted_value,
            _ => panic!("Pattern doesn't match!"),
        }
    };
}

fn check_limits(system: &mut System) -> bool {
    // Find needed limits.
    let max_buffer_size: usize = system
        .gpu
        .buffers
        .iter()
        .map(|it| it.lock().unwrap().data.len())
        .sum();
    let uniform_buffers = system
        .gpu
        .buffers
        .iter()
        .filter(|it| matches!(it.lock().unwrap().detail, GpuBufferDetail::Uniform));
    let uniform_buffer_count = uniform_buffers.clone().count();
    let max_uniform_buffer_binding_size: usize = uniform_buffers
        .clone()
        .map(|it| it.lock().unwrap().data.len())
        .sum();
    let vertex_buffers = system
        .gpu
        .buffers
        .iter()
        .filter(|it| matches!(it.lock().unwrap().detail, GpuBufferDetail::Vertex { .. }));
    let vertex_buffer_count = vertex_buffers.clone().count();
    let max_vertex_buffer_attributes: usize = vertex_buffers
        .clone()
        .map(|it| {
            let buffer = it.lock().unwrap();
            let layout =
                extract_enum_value!(&buffer.detail, GpuBufferDetail::Vertex { layout } => layout);
            layout.attributes.len()
        })
        .max()
        .unwrap_or(0) as usize;
    let max_vertex_buffer_stride: usize = vertex_buffers
        .clone()
        .map(|it| {
            let buffer = it.lock().unwrap();
            let layout =
                extract_enum_value!(&buffer.detail, GpuBufferDetail::Vertex { layout } => layout);
            layout.array_stride
        })
        .max()
        .unwrap_or(0) as usize;
    // Find current limits.
    let mut limits = system
        .limits
        .unwrap_or_else(|| wgpu_adapter_get_limits_simple(system));
    let had_device = !system.device.0.is_null();
    let mut any_change = !had_device;
    // Use a growth ratio on anything that seems likely to grow arbitrarily.
    let ratio = match had_device {
        // Grow with a buffer for anything that needs to grow.
        true => 1.5,
        false => 1.0,
    };
    if max_buffer_size > limits.maxBufferSize as usize {
        limits.maxBufferSize = (ratio * max_buffer_size as f64) as u64;
        any_change = true;
    }
    if max_uniform_buffer_binding_size > limits.maxUniformBufferBindingSize as usize {
        limits.maxUniformBufferBindingSize =
            (ratio * max_uniform_buffer_binding_size as f64) as u64;
        any_change = true;
    }
    // Just set exactly those that seem likely to change less.
    if uniform_buffer_count > limits.maxUniformBuffersPerShaderStage as usize {
        limits.maxUniformBuffersPerShaderStage = uniform_buffer_count as u32;
        any_change = true;
    }
    if vertex_buffer_count > limits.maxVertexBuffers as usize {
        limits.maxVertexBuffers = vertex_buffer_count as u32;
        any_change = true;
    }
    if max_vertex_buffer_attributes > limits.maxVertexAttributes as usize {
        limits.maxVertexAttributes = max_vertex_buffer_attributes as u32;
        any_change = true;
    }
    if max_vertex_buffer_stride > limits.maxVertexBufferArrayStride as usize {
        limits.maxVertexBufferArrayStride = max_vertex_buffer_stride as u32;
        any_change = true;
    }
    // Also just prefill any defaults we like.
    if !had_device {
        limits = native::WGPULimits {
            // Semi-arbitrary interstage. TODO Inspect wgsl code???
            maxInterStageShaderComponents: 6,
            ..limits
        };
        any_change = true;
    }
    system.limits = Some(limits);
    any_change
    // This one seems unlikely to grow arbitrarily, so no ratio.
    // TODO Check limits vs needs. If too low, and if already have device, drop it and get new.
    // const required_limits = c.WGPURequiredLimits{
    //     .nextInChain = null,
    //     .limits = std.mem.zeroInit(c.WGPULimits, .{
    //         .maxTextureDimension1D = 5000,
    //         .maxTextureDimension2D = 3000,
    //         .maxTextureArrayLayers = 1,
    //         .maxBindGroups = 1,
    //         .maxSampledTexturesPerShaderStage = 1,
    //         .maxBufferSize = @max(@sizeOf(@TypeOf(d.point_data)), @sizeOf(Uniforms)),
    //         .maxUniformBufferBindingSize = @sizeOf(Uniforms),
    //         .maxUniformBuffersPerShaderStage = 1,
    //         .maxVertexAttributes = 3,
    //         .maxVertexBuffers = 1,
    //         .maxVertexBufferArrayStride = d.vertex_stride,
    //         .minStorageBufferOffsetAlignment = supported_limits.limits.minStorageBufferOffsetAlignment,
    //         .minUniformBufferOffsetAlignment = supported_limits.limits.minUniformBufferOffsetAlignment,
    //         .maxInterStageShaderComponents = 6,
    //     }),
    // };
}

fn update_buffers(system: &mut System, need_all: bool) {
    for buffer in &system.gpu.buffers {
        let mut buffer = buffer.lock().unwrap();
        // TODO If previous size non-zero and less than current, reserve extra?
        // TODO Does the full buffer size get seen no matter the latest write?
        let needed = need_all || buffer.size == 0 || buffer.data.len() > buffer.size;
        if !needed {
            continue;
        }
        if !buffer.buffer.0.is_null() {
            unsafe { wgpuBufferDrop(buffer.buffer.0) };
            buffer.buffer.0 = null_mut();
            buffer.size = 0;
        }
        buffer.size = buffer.data.len();
        unsafe {
            buffer.buffer.0 = wgpu_native::device::wgpuDeviceCreateBuffer(
                system.device.0,
                Some(&native::WGPUBufferDescriptor {
                    nextInChain: null(),
                    label: null(),
                    usage: native::WGPUBufferUsage_CopyDst
                        | match buffer.detail {
                            GpuBufferDetail::Index { .. } => native::WGPUBufferUsage_Index,
                            GpuBufferDetail::Uniform => native::WGPUBufferUsage_Uniform,
                            GpuBufferDetail::Vertex { .. } => native::WGPUBufferUsage_Vertex,
                        },
                    size: buffer.size as u64,
                    mappedAtCreation: false,
                }),
            );
            wgpu_native::device::wgpuQueueWriteBuffer(
                system.queue.0,
                buffer.buffer.0,
                0,
                buffer.data.as_ptr(),
                buffer.data.len(),
            );
        }
    }
}

fn ensure_swap_chain(system: &mut System) -> bool {
    if !system.swap_chain.0.is_null() {
        return false;
    }
    let format = wgpu_surface_get_preferred_format_simple(system);
    let size = system.window.as_ref().unwrap().inner_size();
    system.swap_chain.0 = unsafe {
        wgpu_native::device::wgpuDeviceCreateSwapChain(
            system.device.0,
            system.surface.0,
            Some(&native::WGPUSwapChainDescriptor {
                nextInChain: null(),
                label: null(),
                usage: native::WGPUTextureUsage_RenderAttachment,
                format,
                width: size.width,
                height: size.height,
                presentMode: native::WGPUPresentMode_Fifo,
            }),
        )
    };
    true
}

fn taca_gpu_ensure_device(system: &mut System) -> bool {
    // TODO Some clean flag to skip all these checks?
    // Instance, surface, & adapter.
    wgpu_ensure_instance_simple(system);
    wgpu_instance_ensure_surface_simple(system);
    wgpu_instance_ensure_adapter_simple(system);
    // Device & queue.
    let had_device = !system.device.0.is_null();
    let mut any_change = check_limits(system) || !had_device;
    if any_change || !had_device {
        if had_device {
            unsafe {
                // TODO Drop written buffers!
                // TODO Drop any other dependencies!
                wgpuDeviceDrop(system.device.0);
            }
            system.device.0 = null_mut();
        }
        if wgpu_adapter_ensure_device_simple(system) {
            // Always get error messages.
            wgpu_device_ensure_uncaptured_error_callback_simple(system);
        }
        wgpu_device_ensure_queue_simple(system);
    }
    // Buffers.
    update_buffers(system, any_change);
    any_change |= ensure_swap_chain(system);
    any_change
}

fn taca_gpu_ensure_pipeline(system: &mut System) {
    let had_pipeline = !system.gpu.pipeline.0.is_null();
    let any_change = taca_gpu_ensure_device(system) || !had_pipeline;
    if !any_change {
        return;
    }
    if had_pipeline {
        // TODO Should this have been dropped already if we redid the device?
        unsafe {
            wgpuRenderPipelineDrop(system.gpu.pipeline.0);
        }
        system.gpu.pipeline.0 = null_mut();
    }
    let mut entries = Vec::<native::WGPUBindGroupLayoutEntry>::new();
    for buffer in &system.gpu.buffers {
        let buffer = buffer.lock().unwrap();
        let entry = match buffer.detail {
            GpuBufferDetail::Uniform => native::WGPUBindGroupLayoutEntry {
                nextInChain: null(),
                binding: 0,
                visibility: native::WGPUShaderStage_Vertex | native::WGPUShaderStage_Fragment,
                buffer: native::WGPUBufferBindingLayout {
                    nextInChain: null(),
                    type_: native::WGPUBufferBindingType_Uniform,
                    hasDynamicOffset: false,
                    minBindingSize: 0,
                },
                sampler: native::WGPUSamplerBindingLayout {
                    nextInChain: null(),
                    type_: native::WGPUSamplerBindingType_Undefined,
                },
                texture: native::WGPUTextureBindingLayout {
                    nextInChain: null(),
                    sampleType: native::WGPUTextureSampleType_Undefined,
                    viewDimension: native::WGPUTextureViewDimension_Undefined,
                    multisampled: false,
                },
                storageTexture: native::WGPUStorageTextureBindingLayout {
                    nextInChain: null(),
                    access: native::WGPUStorageTextureAccess_Undefined,
                    format: native::WGPUTextureFormat_Undefined,
                    viewDimension: native::WGPUTextureViewDimension_Undefined,
                },
            },
            _ => continue,
        };
        entries.push(entry);
    }
    //         std.mem.zeroInit(c.WGPUBindGroupLayoutEntry, .{
    //             .binding = 1,
    //             .visibility = c.WGPUShaderStage_Fragment,
    //             .sampler = std.mem.zeroInit(c.WGPUSamplerBindingLayout, .{
    //                 .type = c.WGPUSamplerBindingType_NonFiltering,
    //             }),
    //             .texture = std.mem.zeroInit(c.WGPUTextureBindingLayout, .{
    //                 .sampleType = c.WGPUTextureSampleType_Float,
    //                 .viewDimension = c.WGPUTextureViewDimension_2D,
    //             }),
    //         }),
    //     },
    // TODO Store this for later.
    let layout = unsafe {
        wgpu_native::device::wgpuDeviceCreateBindGroupLayout(
            system.device.0,
            Some(&native::WGPUBindGroupLayoutDescriptor {
                nextInChain: null(),
                label: null(),
                entryCount: entries.len() as u32,
                entries: entries.as_ptr(),
            }),
        )
    };
    // const bind_group = c.wgpuDeviceCreateBindGroup(device, &c.WGPUBindGroupDescriptor{
    //     .nextInChain = null,
    //     .label = null,
    //     .layout = bind_group_layout,
    //     .entryCount = 2,
    //     .entries = &[_]c.WGPUBindGroupEntry{
    //         std.mem.zeroInit(c.WGPUBindGroupEntry, .{
    //             .binding = 0,
    //             .buffer = uniform_buffer,
    //             .size = @sizeOf(Uniforms),
    //         }),
    //         std.mem.zeroInit(c.WGPUBindGroupEntry, .{
    //             .binding = 1,
    //             .textureView = image_texture_view,
    //         }),
    //     },
    // });
    // TODO Store this for later.
    let pipeline_layout = unsafe {
        wgpu_native::device::wgpuDeviceCreatePipelineLayout(
            system.device.0,
            Some(&native::WGPUPipelineLayoutDescriptor {
                nextInChain: null(),
                label: null(),
                bindGroupLayoutCount: 1,
                bindGroupLayouts: &layout,
            }),
        )
    };
    let shader =
        wgpu_device_create_shader_module_simple(system, system.gpu.shaders[0].clone().as_c_str());
    let shader = system.shaders[shader as usize - 1].0;
    let vertex_layouts: Vec<_> = system
        .gpu
        .buffers
        .clone()
        .iter()
        .filter(|it| matches!(it.lock().unwrap().detail, GpuBufferDetail::Vertex { .. }))
        .map(|it| {
            let buffer = it.lock().unwrap();
            let layout =
                extract_enum_value!(&buffer.detail, GpuBufferDetail::Vertex { layout } => layout);
            native::WGPUVertexBufferLayout {
                arrayStride: layout.array_stride,
                stepMode: layout.step_mode,
                attributeCount: layout.attributes.len() as u32,
                attributes: layout.attributes.as_ptr(),
            }
        })
        .collect();
    let format = wgpu_surface_get_preferred_format_simple(system);
    let target = native::WGPUColorTargetState {
        nextInChain: null(),
        format,
        blend: null(),
        writeMask: native::WGPUColorWriteMask_All,
    };
    let pipeline = unsafe {
        wgpu_native::device::wgpuDeviceCreateRenderPipeline(
            system.device.0,
            Some(&native::WGPURenderPipelineDescriptor {
                nextInChain: null(),
                label: null(),
                layout: pipeline_layout,
                vertex: native::WGPUVertexState {
                    nextInChain: null(),
                    module: shader,
                    entryPoint: "vs_main\0".as_bytes().as_ptr() as *const i8,
                    constantCount: 0,
                    constants: null(),
                    bufferCount: vertex_layouts.len() as u32,
                    buffers: vertex_layouts.as_ptr(),
                },
                primitive: native::WGPUPrimitiveState {
                    nextInChain: null(),
                    topology: native::WGPUPrimitiveTopology_TriangleList,
                    stripIndexFormat: native::WGPUIndexFormat_Undefined,
                    frontFace: native::WGPUFrontFace_CCW,
                    cullMode: native::WGPUCullMode_None,
                },
                depthStencil: null(),
                // depthStencil: &native::WGPUDepthStencilState {
                //     nextInChain: null(),
                //     format: native::WGPUTextureFormat_Depth24Plus,
                //     depthWriteEnabled: true,
                //     depthCompare: native::WGPUCompareFunction_Less,
                //     stencilFront: native::WGPUStencilFaceState {
                //         compare: native::WGPUCompareFunction_Always,
                //         failOp: native::WGPUStencilOperation_Keep,
                //         depthFailOp: native::WGPUStencilOperation_Keep,
                //         passOp: native::WGPUStencilOperation_Keep,
                //     },
                //     stencilBack: native::WGPUStencilFaceState {
                //         compare: native::WGPUCompareFunction_Always,
                //         failOp: native::WGPUStencilOperation_Keep,
                //         depthFailOp: native::WGPUStencilOperation_Keep,
                //         passOp: native::WGPUStencilOperation_Keep,
                //     },
                //     stencilReadMask: 0,
                //     stencilWriteMask: 0,
                //     depthBias: 0,
                //     depthBiasSlopeScale: 0.0,
                //     depthBiasClamp: 0.0,
                // },
                multisample: native::WGPUMultisampleState {
                    nextInChain: null(),
                    count: 1,
                    mask: 0xFFFFFFFF,
                    alphaToCoverageEnabled: false,
                },
                fragment: &native::WGPUFragmentState {
                    nextInChain: null(),
                    module: shader,
                    entryPoint: "fs_main\0".as_bytes().as_ptr() as *const i8,
                    constantCount: 0,
                    constants: null(),
                    targetCount: 1,
                    targets: &target,
                } as *const native::WGPUFragmentState,
            }),
        )
    };
    assert_ne!(null(), pipeline);
    system.gpu.pipeline.0 = pipeline;
}

fn taca_gpu_ensure_render_pass(system: &mut System) {
    taca_gpu_ensure_pipeline(system);
    if system.gpu.texture_view.0.is_null() {
        system.gpu.texture_view.0 =
            unsafe { wgpu_native::device::wgpuSwapChainGetCurrentTextureView(system.swap_chain.0) };
    }
    wgpu_device_ensure_command_encoder_simple(system);
    if system.render_pass.0.is_null() {
        let color_attachment = native::WGPURenderPassColorAttachment {
            view: system.texture_views[0].0,
            resolveTarget: std::ptr::null_mut(),
            loadOp: native::WGPULoadOp_Clear,
            storeOp: native::WGPUStoreOp_Store,
            clearValue: native::WGPUColor {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        };
        // let depth_stencil_attachment = native::WGPURenderPassDepthStencilAttachment {
        //     view: system.texture_views[depth_stencil_attachment.view as usize - 1].0,
        //     depthLoadOp: depth_stencil_attachment.depth_load_op,
        //     depthStoreOp: depth_stencil_attachment.depth_store_op,
        //     depthClearValue: depth_stencil_attachment.depth_clear_value,
        //     depthReadOnly: depth_stencil_attachment.depth_read_only,
        //     stencilLoadOp: depth_stencil_attachment.stencil_load_op,
        //     stencilStoreOp: depth_stencil_attachment.stencil_store_op,
        //     stencilClearValue: depth_stencil_attachment.stencil_clear_value,
        //     stencilReadOnly: depth_stencil_attachment.stencil_read_only,
        // };
        //         .depthStencilAttachment = &c.WGPURenderPassDepthStencilAttachment{
        //             .view = state.depth_texture_out.depth_texture_view,
        //             .depthLoadOp = c.WGPULoadOp_Clear,
        //             .depthStoreOp = c.WGPUStoreOp_Store,
        //             .depthClearValue = 1,
        //             .depthReadOnly = false,
        //             .stencilLoadOp = c.WGPULoadOp_Clear,
        //             .stencilStoreOp = c.WGPUStoreOp_Store,
        //             .stencilClearValue = 0,
        //             .stencilReadOnly = true,
        //         },
        system.render_pass.0 = unsafe {
            wgpu_native::command::wgpuCommandEncoderBeginRenderPass(
                system.encoder.0,
                Some(&native::WGPURenderPassDescriptor {
                    nextInChain: std::ptr::null(),
                    label: null(),
                    colorAttachmentCount: 1,
                    colorAttachments: &color_attachment,
                    depthStencilAttachment: null(),
                    // depthStencilAttachment: &depth_stencil_attachment,
                    occlusionQuerySet: std::ptr::null_mut(),
                    timestampWriteCount: 0,
                    timestampWrites: std::ptr::null(),
                }),
            )
        };
    }
    unsafe {
        wgpu_native::command::wgpuRenderPassEncoderSetPipeline(
            system.render_pass.0,
            system.gpu.pipeline.0,
        );
    }
}

pub fn gpu_draw_set_buffer(system: &System, buffer: &GpuBuffer) {
    match &buffer.detail {
        GpuBufferDetail::Index { format, vertex } => {
            let vertex = vertex.lock().unwrap();
            gpu_draw_set_buffer(system, &vertex);
            unsafe {
                wgpu_native::command::wgpuRenderPassEncoderSetIndexBuffer(
                    system.render_pass.0,
                    buffer.buffer.0,
                    *format,
                    0,
                    buffer.size as u64,
                );
            }
        }
        GpuBufferDetail::Uniform => {}
        GpuBufferDetail::Vertex { .. } => {
            unsafe {
                wgpu_native::command::wgpuRenderPassEncoderSetVertexBuffer(
                    system.render_pass.0,
                    0, // TODO What for slot?
                    buffer.buffer.0,
                    0,
                    buffer.size as u64,
                );
            }
        }
    }
}

// taca_EXPORT void taca_gpuDraw(taca_GpuBuffer buffer);
pub fn taca_gpu_draw(mut env: FunctionEnvMut<System>, buffer: u32) {
    let system = env.data_mut();
    update_buffers(system, false);
    let buffer = system.gpu.buffers[buffer as usize].lock().unwrap();
    gpu_draw_set_buffer(system, &buffer);
    // unsafe {
    //     wgpu_native::command::wgpuRenderPassEncoderSetBindGroup(
    //         system.render_pass.0,
    //         0,
    //         system.gpu.,
    //         0,
    //         null(),
    //     );
    // }
    // TODO Draw
}

pub fn taca_gpu_index_buffer_create(
    mut env: FunctionEnvMut<System>,
    size: u32,
    data: u32,
    format: u32,
    vertex: u32,
) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let vertex = system.gpu.buffers[vertex as usize].clone();
    assert!(matches!(
        vertex.lock().unwrap().detail,
        GpuBufferDetail::Vertex { .. },
    ));
    system.gpu.buffers.push(Arc::new(Mutex::new(GpuBuffer {
        buffer: Default::default(),
        data: WasmPtr::<u8>::new(data)
            .slice(&view, size)
            .unwrap()
            .read_to_vec()
            .unwrap(),
        detail: GpuBufferDetail::Index { format, vertex },
        size: 0,
    })));
    system.buffers.len() as u32
}

// taca_EXPORT void taca_gpuPresent(void);
pub fn taca_gpu_present(mut env: FunctionEnvMut<System>) {
    let system = env.data_mut();
    taca_gpu_ensure_render_pass(system);
    unsafe {
        wgpu_native::command::wgpuRenderPassEncoderEnd(system.render_pass.0);
    }
    system.render_pass.0 = null_mut();
    if !system.gpu.texture_view.0.is_null() {
        unsafe {
            wgpu_native::device::wgpuTextureViewDrop(system.gpu.texture_view.0);
            system.gpu.texture_view.0 = null_mut();
        }
    }
    wgpu_ensure_command_encoder_finish_simple(system);
    wgpu_ensure_queue_submit_simple(system);
    unsafe {
        wgpu_native::device::wgpuSwapChainPresent(system.swap_chain.0);
    }
}

pub fn taca_gpu_shader_create(mut env: FunctionEnvMut<System>, wgsl: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let wgsl = read_cstring(WasmPtr::<u8>::new(wgsl), &view).unwrap();
    system.gpu.shaders.push(wgsl);
    // wgpu_device_create_shader_module_simple(system, &store, WasmPtr::<u8>::new(wgsl))
    system.gpu.shaders.len() as u32
}

// taca_EXPORT taca_GpuBuffer taca_gpuUniformBufferCreate(size_t size);
pub fn taca_gpu_uniform_buffer_create(mut env: FunctionEnvMut<System>, size: u32) -> u32 {
    let system = env.data_mut();
    system.gpu.buffers.push(Arc::new(Mutex::new(GpuBuffer {
        buffer: Default::default(),
        data: vec![0; size as usize],
        detail: GpuBufferDetail::Uniform,
        size: 0,
    })));
    system.buffers.len() as u32
}

// taca_EXPORT taca_gpu_Buffer taca_gpu_vertexBufferCreate(size_t size, const void* data, const WGPUVertexBufferLayout* layout);
pub fn taca_gpu_vertex_buffer_create(
    mut env: FunctionEnvMut<System>,
    size: u32,
    data: u32,
    layout: u32,
) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    // TODO How to avoid this extra copy?
    let data = WasmPtr::<u8>::new(data)
        .slice(&view, size)
        .unwrap()
        .read_to_vec()
        .unwrap();
    // TODO Read into self-contained nested data with vecs.
    let layout = WasmPtr::<WasmWGPUVertexBufferLayout>::new(layout)
        .read(&view)
        .unwrap();
    let layout = WgpuVertexBufferLayout {
        array_stride: layout.array_stride,
        step_mode: layout.step_mode,
        attributes: layout.attributes_vec(&view),
    };
    system.gpu.buffers.push(Arc::new(Mutex::new(GpuBuffer {
        buffer: Default::default(),
        data,
        detail: GpuBufferDetail::Vertex { layout },
        size: 0,
    })));
    system.buffers.len() as u32
}
