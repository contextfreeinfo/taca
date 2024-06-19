use miniquad::{BufferId, Pipeline, RenderingBackend};
#[cfg(not(target_arch = "wasm32"))]
use wasmer::Memory;

use crate::shaders::Shader;

pub struct Platform {
    pub buffer: Vec<u8>,
    pub contexts: Vec<RenderingContext>,
    pub buffer_ids: Vec<BufferId>,
    #[cfg(not(target_arch = "wasm32"))]
    pub memory: Option<Memory>,
    pub pipelines: Vec<Pipeline>,
    pub shaders: Vec<Shader>,
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
        }
    }
}
