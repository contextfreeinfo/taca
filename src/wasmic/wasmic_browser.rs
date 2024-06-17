#![allow(non_snake_case)]

use crate::platform::Platform;
use crate::wasmic::help::{new_buffer, BufferSlice};
use miniquad::{conf, EventHandler};
use std::mem::size_of;

struct App {}

impl EventHandler for App {
    fn update(&mut self) {}

    fn draw(&mut self) {
        // TODO
    }
}

pub fn wasmish() {
    unsafe {
        let mut conf = conf::Conf::default();
        conf.platform.apple_gfx_api = conf::AppleGfxApi::Metal;
        conf.platform.webgl_version = conf::WebGLVersion::WebGL2;
        conf.window_title = "Taca".into();
        miniquad::start(conf, move || {
            let mut platform = Box::new(Platform::new(1024));
            let buffer_ptr = platform.buffer.as_mut_ptr();
            let buffer_len = platform.buffer.len();
            let platform = Box::into_raw(platform);
            // crate::wasmic::print(&format!("rust platform: {platform:p}"));
            // TODO Also pass in buffer.
            browser_load_app(platform, buffer_ptr, buffer_len);
            Box::new(App {})
        })
    }
}

pub fn print(text: &str) {
    unsafe {
        browser_print(text.as_ptr(), text.len());
    }
}

extern "C" {
    #[link_name = "print"]
    pub fn browser_print(text: *const u8, len: usize);

    #[allow(improper_ctypes)]
    #[link_name = "loadApp"]
    pub fn browser_load_app(platform: *mut Platform, buffer_ptr: *mut u8, buffer_len: usize);

    #[allow(improper_ctypes)]
    #[link_name = "readAppMemory"]
    pub fn browser_read_app_memory(dest: *mut u8, app_src: u32, count: u32);
}

#[no_mangle]
pub extern "C" fn hi() {
    crate::say_hi();
}

#[no_mangle]
pub extern "C" fn taca_crate_version() -> i32 {
    0
}

#[no_mangle]
fn taca_RenderingContext_applyBindings(_platform: *mut Platform, context: u32, bindings: u32) {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_applyBindings {context} {bindings}"
    ));
}

#[no_mangle]
fn taca_RenderingContext_applyPipeline(_platform: *mut Platform, context: u32, pipeline: u32) {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_applyPipeline {context} {pipeline}"
    ));
}

#[no_mangle]
fn taca_RenderingContext_beginPass(_platform: *mut Platform, context: u32) {
    crate::wasmic::print(&format!("taca_RenderingContext_beginPass {context}"));
}

#[no_mangle]
fn taca_RenderingContext_commitFrame(_platform: *mut Platform, context: u32) {
    crate::wasmic::print(&format!("taca_RenderingContext_commitFrame {context}"));
}

#[no_mangle]
fn taca_RenderingContext_draw(
    _platform: *mut Platform,
    context: u32,
    base_element: u32,
    num_elements: u32,
    num_instances: u32,
) {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_draw {context} {base_element} {num_elements} {num_instances}"
    ));
}

#[no_mangle]
fn taca_RenderingContext_endPass(_platform: *mut Platform, context: u32) {
    crate::wasmic::print(&format!("taca_RenderingContext_endPass {context}"));
}

#[no_mangle]
fn taca_RenderingContext_newBuffer(
    platform: *mut Platform,
    context: u32,
    typ: u32,
    usage: u32,
    info: u32,
) -> u32 {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_newBuffer {context} {typ} {usage} {info}"
    ));
    let platform = unsafe { &mut *platform };
    let slice = unsafe {
        &*((&platform.buffer[0..size_of::<BufferSlice>()]).as_ptr() as *const BufferSlice)
    };
    crate::wasmic::print(&format!("{slice:?}"));
    let mut buffer = vec![0u8; slice.size as usize];
    unsafe {
        browser_read_app_memory(buffer.as_mut_ptr(), slice.ptr, slice.size);
    }
    crate::wasmic::print(&format!("{buffer:?}"));
    new_buffer(
        &mut platform.context.0,
        typ,
        usage,
        &mut buffer,
        slice.item_size as usize,
    );
    0
}

#[no_mangle]
fn taca_RenderingContext_newPipeline(_platform: *mut Platform, context: u32, info: u32) -> u32 {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_newPipeline {context} {info}"
    ));
    0
}

#[no_mangle]
fn taca_RenderingContext_newShader(_platform: *mut Platform, context: u32, bytes: u32) -> u32 {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_newShader {context} {bytes}"
    ));
    0
}

#[no_mangle]
fn taca_Window_get(_platform: *mut Platform) -> u32 {
    crate::wasmic::print("taca_Window_get");
    1
}

#[no_mangle]
fn taca_Window_newRenderingContext(_platform: *mut Platform, window: u32) -> u32 {
    crate::wasmic::print(&format!("taca_Window_newRenderingContext {window}"));
    1
}
