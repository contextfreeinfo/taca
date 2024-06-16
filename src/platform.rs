use miniquad::{window, RenderingBackend};
#[cfg(not(target_arch = "wasm32"))]
use wasmer::Memory;

pub struct Platform {
    pub buffer: Vec<u8>,
    pub context: RenderingContext,
    #[cfg(not(target_arch = "wasm32"))]
    pub memory: Option<Memory>,
}

pub struct RenderingContext(pub Box<dyn RenderingBackend>);
unsafe impl Send for RenderingContext {}

impl Platform {
    pub fn new(buffer_len: usize) -> Platform {
        Platform {
            buffer: vec![0; buffer_len],
            context: RenderingContext(window::new_rendering_backend()),
            #[cfg(not(target_arch = "wasm32"))]
            memory: None,
        }
    }
}
