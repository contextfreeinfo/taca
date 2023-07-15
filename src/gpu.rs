use std::ptr::null;

use crate::{
    system::System,
    webgpu::{wgpu_device_create_shader_module_simple, WasmWGPUVertexBufferLayout},
};
use wasmer::{FunctionEnvMut, ValueType, WasmPtr};
use wgpu_native::native;

// taca_EXPORT void taca_gpuBufferWrite(taca_GpuBuffer buffer, const void* data);
pub fn taca_gpu_buffer_write(mut env: FunctionEnvMut<System>, buffer: u32, data: u32) {
    let (mut system, mut store) = env.data_and_store_mut();
}

// taca_EXPORT void taca_gpuDraw(taca_GpuBuffer buffer);
pub fn taca_gpu_draw(mut env: FunctionEnvMut<System>, buffer: u32) {
    let (mut system, mut store) = env.data_and_store_mut();
}

// taca_EXPORT taca_GpuBuffer taca_gpuIndexBufferCreate(taca_GpuBuffer vertex, const void* data);
pub fn taca_gpu_index_buffer_create(
    mut env: FunctionEnvMut<System>,
    vertex: u32,
    data: u32,
) -> u32 {
    let (mut system, mut store) = env.data_and_store_mut();
    0
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmGpuConfig {
    vertex_buffer_layout: WasmPtr<WasmWGPUVertexBufferLayout>,
    wgsl: WasmPtr<u8>,
}

// taca_EXPORT void taca_gpuInit(const taca_GpuConfig* config);
pub fn taca_gpu_init(mut env: FunctionEnvMut<System>, config: u32) {
    let (mut system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let config = WasmPtr::<WasmGpuConfig>::new(config).read(&view).unwrap();
    let vertex_layout = config.vertex_buffer_layout.read(&view).unwrap();
    let vertex_attributes = vertex_layout.attributes_vec(&view);
    let vertex_layout = native::WGPUVertexBufferLayout {
        arrayStride: vertex_layout.array_stride,
        stepMode: vertex_layout.step_mode,
        attributeCount: vertex_layout.attribute_count,
        attributes: vertex_attributes.as_ptr(),
    };
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

// taca_EXPORT void taca_gpuPresent(void);
pub fn taca_gpu_present(mut env: FunctionEnvMut<System>) {
    let (mut system, mut store) = env.data_and_store_mut();
}

pub fn taca_gpu_shader_create(mut env: FunctionEnvMut<System>, wgsl: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    wgpu_device_create_shader_module_simple(system, &store, WasmPtr::<u8>::new(wgsl))
}

// taca_EXPORT taca_GpuBuffer taca_gpuUniformBufferCreate(size_t size);
pub fn taca_gpu_uniform_buffer_create(mut env: FunctionEnvMut<System>, size: u32) -> u32 {
    let (mut system, mut store) = env.data_and_store_mut();
    0
}

// taca_EXPORT taca_GpuBuffer taca_gpuVertexBufferCreate(size_t size, const void* data);
pub fn taca_gpu_vertex_buffer_create(mut env: FunctionEnvMut<System>, size: u32, data: u32) -> u32 {
    let (mut system, mut store) = env.data_and_store_mut();
    0
}
