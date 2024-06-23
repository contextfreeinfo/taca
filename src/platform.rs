use miniquad::{window, BufferId, Pipeline, RenderingBackend};
#[cfg(not(target_arch = "wasm32"))]
use wasmer::{Memory, ValueType};

use crate::shaders::Shader;

pub struct Platform {
    pub buffer: Vec<u8>,
    pub contexts: Vec<RenderingContext>,
    pub buffer_ids: Vec<BufferId>,
    pub index_buffer_ids: Vec<BufferId>,
    #[cfg(not(target_arch = "wasm32"))]
    pub memory: Option<Memory>,
    pub pipelines: Vec<Pipeline>,
    pub shaders: Vec<Shader>,
    pub title: Option<String>,
    pub vertex_buffer_ids: Vec<BufferId>,
    pub window_state: WindowState,
}

impl Platform {
    pub fn new(buffer_len: usize) -> Platform {
        Platform {
            buffer: vec![0; buffer_len],
            buffer_ids: vec![],
            contexts: vec![],
            index_buffer_ids: vec![],
            #[cfg(not(target_arch = "wasm32"))]
            memory: None,
            pipelines: vec![],
            shaders: vec![],
            title: None,
            window_state: Default::default(),
            vertex_buffer_ids: vec![],
        }
    }

    pub fn init_state(&mut self) {
        let window_size = window::screen_size();
        self.window_state.pointer = [-1.0, -1.0];
        self.window_state.size = [window_size.0, window_size.1];
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Pass {
    pub begun: bool,
    pub bound: bool,
    pub ended: bool,
    pub pipelined: bool,
}

pub struct RenderingContext {
    pub backend: Box<dyn RenderingBackend>,
    pub pass: Pass,
}

impl RenderingContext {
    pub fn apply_pipeline(&mut self, pipeline: &Pipeline) {
        if !self.pass.begun {
            self.begin_pass();
        }
        self.backend.apply_pipeline(pipeline);
        self.pass.pipelined = true;
    }

    pub fn begin_pass(&mut self) {
        self.backend.begin_default_pass(Default::default());
        self.pass.begun = true;
    }

    pub fn commit_frame(&mut self) {
        if !self.pass.ended {
            self.end_pass();
        }
        self.backend.commit_frame();
        self.pass = Default::default();
    }

    pub fn end_pass(&mut self) {
        self.backend.end_render_pass();
        self.pass.ended = true;
    }
}

unsafe impl Send for RenderingContext {}

#[cfg_attr(not(target_arch = "wasm32"), derive(ValueType))]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct WindowState {
    pub pointer: [f32; 2],
    pub size: [f32; 2],
}
