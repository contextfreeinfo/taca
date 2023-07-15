use crate::{system::System, webgpu::WasmWGPUVertexBufferLayout};
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
