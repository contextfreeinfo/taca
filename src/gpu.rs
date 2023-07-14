use crate::system::System;
use wasmer::{FunctionEnvMut, ValueType};
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
    index_format: native::WGPUIndexFormat,
    // next_in_chain: WasmPtr<WasmWGPUChainedStruct>,
    // label: WasmPtr<u8>,
    // bind_group_layout_count: u32,
    // bind_group_layouts: WasmPtr<u32>, // WGPUBindGroupLayout const *
}

// taca_EXPORT void taca_gpuInit(const taca_GpuConfig* config);
pub fn taca_gpu_init(mut env: FunctionEnvMut<System>, config: u32) {
    let (mut system, mut store) = env.data_and_store_mut();
}

// taca_EXPORT void taca_gpuPresent(void);
pub fn taca_gpu_present(mut env: FunctionEnvMut<System>) {
    let (mut system, mut store) = env.data_and_store_mut();
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
