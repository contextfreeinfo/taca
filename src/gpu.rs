use std::{
    ptr::null,
    sync::{Arc, Mutex}, ffi::CString,
};

use crate::{
    system::System,
    webgpu::{wgpu_device_create_shader_module_simple, WasmWGPUVertexBufferLayout, read_cstring},
};
use wasmer::{FunctionEnvMut, ValueType, WasmPtr};
use wgpu_native::native;

pub struct GpuBuffer {
    data: Vec<u8>,
    detail: GpuBufferDetail,
    size: usize,
    written: bool,
}

pub enum GpuBufferDetail {
    Index {
        format: native::WGPUIndexFormat,
        vertex: Arc<Mutex<GpuBuffer>>,
    },
    Uniform,
    Vertex {
        layout: WasmWGPUVertexBufferLayout,
    },
}

#[derive(Default)]
pub struct SimpleGpu {
    buffers: Vec<Arc<Mutex<GpuBuffer>>>,
    shaders: Vec<CString>,
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
    buffer.data = data;
    buffer.written = false;
}

fn taca_gpu_ensure_device() {
    // TODO Also queue.
    // TODO Write pending buffers.
    // let buffer = unsafe {
    //     wgpu_native::device::wgpuDeviceCreateBuffer(
    //         system.device.0,
    //         Some(&native::WGPUBufferDescriptor {
    //             nextInChain: null(),
    //             label: null(),
    //             usage: native::WGPUBufferUsage_CopyDst | native::WGPUBufferUsage_Vertex,
    //             size: size as u64,
    //             mappedAtCreation: false,
    //         }),
    //     )
    // };
    // unsafe {
    //     wgpu_native::device::wgpuQueueWriteBuffer(
    //         system.queue.0,
    //         system.buffers[buffer as usize - 1].0,
    //         0,
    //         data.as_ptr(),
    //         size as usize,
    //     );
    // }
}

fn taca_gpu_ensure_pipeline() {
    taca_gpu_ensure_device();
    // TODO Also init anything else needed.
    // let pipeline_layout = unsafe {
    //     wgpu_native::device::wgpuDeviceCreatePipelineLayout(
    //         system.device.0,
    //         Some(&native::WGPUPipelineLayoutDescriptor {
    //             nextInChain: null(),
    //             label: null(),
    //             bindGroupLayoutCount: descriptor.bind_group_layout_count,
    //             bindGroupLayouts: bind_group_layouts.as_ptr(),
    //         }),
    //     )
    // };
    // let pipeline = unsafe {
    //     wgpu_native::device::wgpuDeviceCreateRenderPipeline(
    //         system.device.0,
    //         Some(&native::WGPURenderPipelineDescriptor {
    //             nextInChain: null(),
    //             label: null(),
    //             layout: system.pipeline_layouts[descriptor.layout as usize - 1].0,
    //             vertex: native::WGPUVertexState {
    //                 nextInChain: null(),
    //                 module: system.shaders[descriptor.vertex.module as usize - 1].0,
    //                 entryPoint: vertex_entry_point.as_ptr(),
    //                 constantCount: 0,
    //                 constants: null(),
    //                 bufferCount: descriptor.vertex.buffer_count,
    //                 buffers: vertex_layouts.as_ptr(),
    //             },
    //             primitive: native::WGPUPrimitiveState {
    //                 nextInChain: null(),
    //                 topology: descriptor.primitive.topology,
    //                 stripIndexFormat: descriptor.primitive.strip_index_format,
    //                 frontFace: descriptor.primitive.front_face,
    //                 cullMode: descriptor.primitive.cull_mode,
    //             },
    //             depthStencil: &native::WGPUDepthStencilState {
    //                 nextInChain: null(),
    //                 format: depth_stencil.format,
    //                 depthWriteEnabled: depth_stencil.depth_write_enabled,
    //                 depthCompare: depth_stencil.depth_compare,
    //                 stencilFront: native::WGPUStencilFaceState {
    //                     compare: depth_stencil.stencil_front.compare,
    //                     failOp: depth_stencil.stencil_front.fail_op,
    //                     depthFailOp: depth_stencil.stencil_front.depth_fail_op,
    //                     passOp: depth_stencil.stencil_front.pass_op,
    //                 },
    //                 stencilBack: native::WGPUStencilFaceState {
    //                     compare: depth_stencil.stencil_back.compare,
    //                     failOp: depth_stencil.stencil_back.fail_op,
    //                     depthFailOp: depth_stencil.stencil_back.depth_fail_op,
    //                     passOp: depth_stencil.stencil_back.pass_op,
    //                 },
    //                 stencilReadMask: depth_stencil.stencil_read_mask,
    //                 stencilWriteMask: depth_stencil.stencil_write_mask,
    //                 depthBias: depth_stencil.depth_bias,
    //                 depthBiasSlopeScale: depth_stencil.depth_bias_slope_scale,
    //                 depthBiasClamp: depth_stencil.depth_bias_clamp,
    //             },
    //             multisample: native::WGPUMultisampleState {
    //                 nextInChain: null(),
    //                 count: descriptor.multisample.count,
    //                 mask: descriptor.multisample.mask,
    //                 alphaToCoverageEnabled: descriptor.multisample.alpha_to_coverage_enabled,
    //             },
    //             fragment: &native::WGPUFragmentState {
    //                 nextInChain: null(),
    //                 module: system.shaders[fragment.module as usize - 1].0,
    //                 entryPoint: fragment_entry_point.as_ptr(),
    //                 constantCount: 0,
    //                 constants: null(),
    //                 targetCount: fragment.target_count,
    //                 targets: fragment_targets.as_ptr(),
    //             } as *const native::WGPUFragmentState,
    //         }),
    //     )
    // };
    // assert_ne!(null(), pipeline);
    // system.pipelines.push(WGPURenderPipeline(pipeline));
    // system.pipelines.len().try_into().unwrap()
}

fn taca_gpu_ensure_render_pass() {
    taca_gpu_ensure_pipeline();
}

// taca_EXPORT void taca_gpuDraw(taca_GpuBuffer buffer);
pub fn taca_gpu_draw(mut env: FunctionEnvMut<System>, buffer: u32) {
    let (mut system, mut store) = env.data_and_store_mut();
    // TODO Ensure queue.
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
    system.gpu.buffers.push(Arc::new(Mutex::new(GpuBuffer {
        data: WasmPtr::<u8>::new(data)
            .slice(&view, size)
            .unwrap()
            .read_to_vec()
            .unwrap(),
        detail: GpuBufferDetail::Index {
            format,
            vertex: system.gpu.buffers[vertex as usize].clone(),
        },
        size: 0,
        written: false,
    })));
    system.buffers.len() as u32
}

// taca_EXPORT void taca_gpuPresent(void);
pub fn taca_gpu_present(mut env: FunctionEnvMut<System>) {
    let (mut system, mut store) = env.data_and_store_mut();
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
    let (system, _) = env.data_and_store_mut();
    system.gpu.buffers.push(Arc::new(Mutex::new(GpuBuffer {
        data: vec![0; size as usize],
        detail: GpuBufferDetail::Uniform,
        size: 0,
        written: false,
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
    let layout = WasmPtr::<WasmWGPUVertexBufferLayout>::new(layout)
        .read(&view)
        .unwrap();
    // let vertex_attributes = layout.attributes_vec(&view);
    // let layout = native::WGPUVertexBufferLayout {
    //     arrayStride: layout.array_stride,
    //     stepMode: layout.step_mode,
    //     attributeCount: layout.attribute_count,
    //     attributes: vertex_attributes.as_ptr(),
    // };
    system.gpu.buffers.push(Arc::new(Mutex::new(GpuBuffer {
        data,
        detail: GpuBufferDetail::Vertex { layout },
        size: 0,
        written: false,
    })));
    system.buffers.len() as u32
}
