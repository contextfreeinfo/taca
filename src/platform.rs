use miniquad::{window, BufferId, Pipeline, RenderingBackend};
#[cfg(not(target_arch = "wasm32"))]
use wasmer::{Memory, ValueType};

use crate::shaders::Shader;

pub struct Platform {
    pub buffer: Vec<u8>,
    pub contexts: Vec<RenderingContext>,
    pub buffer_ids: Vec<BufferId>,
    #[cfg(not(target_arch = "wasm32"))]
    pub memory: Option<Memory>,
    pub pipelines: Vec<Pipeline>,
    pub shaders: Vec<Shader>,
    pub title: Option<String>,
    pub window_state: WindowState,
}

pub struct RenderingContext(pub Box<dyn RenderingBackend>);
unsafe impl Send for RenderingContext {}

impl Platform {
    pub fn new(buffer_len: usize) -> Platform {
        Platform {
            buffer: vec![0; buffer_len],
            buffer_ids: vec![],
            contexts: vec![],
            #[cfg(not(target_arch = "wasm32"))]
            memory: None,
            pipelines: vec![],
            shaders: vec![],
            title: None,
            window_state: Default::default(),
        }
    }

    pub fn init_state(&mut self) {
        let window_size = window::screen_size();
        self.window_state.pointer = [-1.0, -1.0];
        self.window_state.size = [window_size.0, window_size.1];
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(ValueType))]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct WindowState {
    pub pointer: [f32; 2],
    pub size: [f32; 2],
}
