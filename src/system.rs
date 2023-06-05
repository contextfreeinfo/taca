pub type WGPUAdapter = Pointer<native::WGPUAdapterImpl>;
pub type WGPUInstance = Pointer<native::WGPUInstanceImpl>;
pub type WGPURenderPassEncoder = Pointer<native::WGPURenderPassEncoderImpl>;
pub type WGPUSwapChain = Pointer<native::WGPUSwapChainImpl>;
pub struct Pointer<T>(pub *mut T);
unsafe impl<T> Send for Pointer<T> {}
impl<T> Default for Pointer<T> {
    fn default() -> Self {
        Pointer(null_mut())
    }
}

pub struct WGPUBindGroup(pub native::WGPUBindGroup);
unsafe impl Send for WGPUBindGroup {}
impl Default for WGPUBindGroup {
    fn default() -> Self {
        WGPUBindGroup(null_mut())
    }
}

pub struct WGPUBindGroupLayout(pub native::WGPUBindGroupLayout);
unsafe impl Send for WGPUBindGroupLayout {}
impl Default for WGPUBindGroupLayout {
    fn default() -> Self {
        WGPUBindGroupLayout(null_mut())
    }
}

pub struct WGPUBuffer(pub native::WGPUBuffer);
unsafe impl Send for WGPUBuffer {}
impl Default for WGPUBuffer {
    fn default() -> Self {
        WGPUBuffer(null_mut())
    }
}

pub struct WGPUCommandBuffer(pub native::WGPUCommandBuffer);
unsafe impl Send for WGPUCommandBuffer {}
impl Default for WGPUCommandBuffer {
    fn default() -> Self {
        WGPUCommandBuffer(null_mut())
    }
}

pub struct WGPUCommandEncoder(pub native::WGPUCommandEncoder);
unsafe impl Send for WGPUCommandEncoder {}
impl Default for WGPUCommandEncoder {
    fn default() -> Self {
        WGPUCommandEncoder(null_mut())
    }
}

pub struct WGPUDevice(pub native::WGPUDevice);
unsafe impl Send for WGPUDevice {}
impl Default for WGPUDevice {
    fn default() -> Self {
        WGPUDevice(null_mut())
    }
}

pub struct WGPUQueue(pub native::WGPUQueue);
unsafe impl Send for WGPUQueue {}
impl Default for WGPUQueue {
    fn default() -> Self {
        WGPUQueue(null_mut())
    }
}

pub struct WGPUPipelineLayout(pub native::WGPUPipelineLayout);
unsafe impl Send for WGPUPipelineLayout {}
impl Default for WGPUPipelineLayout {
    fn default() -> Self {
        WGPUPipelineLayout(null_mut())
    }
}

pub struct WGPURenderPipeline(pub native::WGPURenderPipeline);
unsafe impl Send for WGPURenderPipeline {}
impl Default for WGPURenderPipeline {
    fn default() -> Self {
        WGPURenderPipeline(null_mut())
    }
}

pub struct WGPUShaderModule(pub native::WGPUShaderModule);
unsafe impl Send for WGPUShaderModule {}
impl Default for WGPUShaderModule {
    fn default() -> Self {
        WGPUShaderModule(null_mut())
    }
}

pub struct WGPUSurface(pub native::WGPUSurface);
unsafe impl Send for WGPUSurface {}
impl Default for WGPUSurface {
    fn default() -> Self {
        WGPUSurface(null_mut())
    }
}

pub struct WGPUTextureView(pub native::WGPUTextureView);
unsafe impl Send for WGPUTextureView {}
impl Default for WGPUTextureView {
    fn default() -> Self {
        WGPUTextureView(null_mut())
    }
}

#[derive(Default)]
pub struct System {
    pub adapter: WGPUAdapter,
    pub bind_groups: Vec<WGPUBindGroup>,
    pub bind_group_layouts: Vec<WGPUBindGroupLayout>,
    pub buffers: Vec<WGPUBuffer>,
    pub command_buffer: WGPUCommandBuffer,
    pub device: WGPUDevice,
    pub device_uncaptured_error_callback: Option<wasmer::Function>,
    pub device_uncaptured_error_callback_userdata: u32,
    pub encoder: WGPUCommandEncoder,
    pub functions: Option<Table>,
    pub instance: WGPUInstance,
    pub memory: Option<Memory>,
    pub queue: WGPUQueue,
    pub pipelines: Vec<WGPURenderPipeline>,
    pub pipeline_layouts: Vec<WGPUPipelineLayout>,
    pub render_pass: WGPURenderPassEncoder,
    pub shaders: Vec<WGPUShaderModule>,
    pub surface: WGPUSurface,
    pub swap_chain: WGPUSwapChain,
    pub texture_view: WGPUTextureView,
    pub window: Option<Window>,
    pub window_listen: Option<wasmer::Function>,
    pub window_listen_userdata: u32,
}

impl System {
    pub fn new(window: Window) -> System {
        System {
            window: Some(window),
            ..Default::default()
        }
    }
}

use std::ptr::null_mut;
use wasmer::{Memory, Table};
use wgpu_native::native;
use winit::window::Window;
