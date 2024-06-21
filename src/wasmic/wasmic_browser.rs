#![allow(non_snake_case)]

use crate::platform::Platform;
use crate::wasmic::help::{
    apply_bindings, apply_pipeline, apply_uniforms, begin_pass, commit_frame, draw, end_pass,
    new_buffer, new_pipeline, new_rendering_context, new_shader, Bindings, BufferSlice,
    ExternBindings, ExternPipelineInfo, PipelineInfo, PipelineShaderInfo, Span, VertexAttribute,
};
use miniquad::{conf, EventHandler};
use std::mem::size_of;

struct App {}

impl EventHandler for App {
    fn update(&mut self) {}

    fn draw(&mut self) {
        unsafe {
            browser_send_event(0);
        }
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

    #[allow(improper_ctypes)] // Ok because opaque.
    #[link_name = "loadApp"]
    pub fn browser_load_app(platform: *mut Platform, buffer_ptr: *mut u8, buffer_len: usize);

    #[link_name = "readAppMemory"]
    pub fn browser_read_app_memory(dest: *mut u8, app_src: u32, count: u32);

    #[link_name = "sendEvent"]
    pub fn browser_send_event(kind: u32);
}

pub unsafe fn browser_read_app_span<T>(dest: *mut T, span: Span) {
    browser_read_app_memory(dest as *mut u8, span.ptr, span.len * size_of::<T>() as u32);
}

pub unsafe fn browser_read_app_string(span: Span) -> String {
    let mut buffer = vec![0u8; span.len as usize];
    browser_read_app_span(buffer.as_mut_ptr(), span);
    String::from_utf8(buffer).unwrap()
}

pub unsafe fn browser_read_app_vec<T>(span: Span) -> Vec<T>
where
    T: Clone + Copy + Default,
{
    let mut buffer = vec![T::default(); span.len as usize];
    browser_read_app_span(buffer.as_mut_ptr(), span);
    buffer
}

#[no_mangle]
pub extern "C" fn taca_crate_version() -> i32 {
    0
}

#[no_mangle]
fn taca_RenderingContext_applyBindings(platform: *mut Platform, context: u32, _bindings: u32) {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_applyBindings {context} {bindings}"
    // ));
    let platform = unsafe { &mut *platform };
    let bindings = unsafe {
        &*((&platform.buffer[0..size_of::<ExternBindings>()]).as_ptr() as *const ExternBindings)
    };
    let vertex_buffers = unsafe { browser_read_app_vec::<u32>(bindings.vertex_buffers) };
    let bindings = Bindings {
        vertex_buffers,
        index_buffer: bindings.index_buffer,
    };
    apply_bindings(platform, context, bindings);
}

#[no_mangle]
fn taca_RenderingContext_applyPipeline(platform: *mut Platform, context: u32, pipeline: u32) {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_applyPipeline {context} {pipeline}"
    // ));
    let platform = unsafe { &mut *platform };
    apply_pipeline(platform, context, pipeline);
}

#[no_mangle]
fn taca_RenderingContext_applyUniforms(platform: *mut Platform, context: u32, _uniforms: u32) {
    let platform = unsafe { &mut *platform };
    let uniforms = unsafe { *((&platform.buffer[0..size_of::<Span>()]).as_ptr() as *const Span) };
    let uniforms = unsafe { browser_read_app_vec::<u8>(uniforms) };
    apply_uniforms(platform, context, &uniforms);
}

#[no_mangle]
fn taca_RenderingContext_beginPass(platform: *mut Platform, context: u32) {
    // crate::wasmic::print(&format!("taca_RenderingContext_beginPass {context}"));
    let platform = unsafe { &mut *platform };
    begin_pass(platform, context);
}

#[no_mangle]
fn taca_RenderingContext_commitFrame(platform: *mut Platform, context: u32) {
    // crate::wasmic::print(&format!("taca_RenderingContext_commitFrame {context}"));
    let platform = unsafe { &mut *platform };
    commit_frame(platform, context);
}

#[no_mangle]
fn taca_RenderingContext_draw(
    platform: *mut Platform,
    context: u32,
    item_begin: i32,
    item_count: i32,
    instance_count: i32,
) {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_draw {context} {base_element} {num_elements} {num_instances}"
    // ));
    let platform = unsafe { &mut *platform };
    draw(platform, context, item_begin, item_count, instance_count);
}

#[no_mangle]
fn taca_RenderingContext_endPass(platform: *mut Platform, context: u32) {
    // crate::wasmic::print(&format!("taca_RenderingContext_endPass {context}"));
    let platform = unsafe { &mut *platform };
    end_pass(platform, context);
}

#[no_mangle]
fn taca_RenderingContext_newBuffer(
    platform: *mut Platform,
    context: u32,
    typ: u32,
    usage: u32,
    _info: u32,
) -> u32 {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_newBuffer {context} {typ} {usage} {info}"
    // ));
    let platform = unsafe { &mut *platform };
    let slice = unsafe {
        &*((&platform.buffer[0..size_of::<BufferSlice>()]).as_ptr() as *const BufferSlice)
    };
    // crate::wasmic::print(&format!("{slice:?}"));
    let mut buffer = vec![0u8; slice.size as usize];
    unsafe {
        browser_read_app_memory(buffer.as_mut_ptr(), slice.ptr, slice.size);
    }
    // crate::wasmic::print(&format!("{buffer:?}"));
    new_buffer(
        platform,
        context,
        typ,
        usage,
        &mut buffer,
        slice.item_size as usize,
    )
}

#[no_mangle]
fn taca_RenderingContext_newPipeline(platform: *mut Platform, context: u32, _info: u32) -> u32 {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_newPipeline {context} {info}"
    // ));
    let platform = unsafe { &mut *platform };
    let info = unsafe {
        &*((&platform.buffer[0..size_of::<ExternPipelineInfo>()]).as_ptr()
            as *const ExternPipelineInfo)
    };
    let attributes = unsafe { browser_read_app_vec::<VertexAttribute>(info.attributes) };
    let fragment_entry_point = unsafe { browser_read_app_string(info.fragment.entry_point) };
    let vertex_entry_point = unsafe { browser_read_app_string(info.vertex.entry_point) };
    // crate::wasmic::print(&format!(
    //     "{info:?} -- {vertex_entry_point} -- {fragment_entry_point}"
    let info = PipelineInfo {
        attributes,
        fragment: PipelineShaderInfo {
            entry_point: fragment_entry_point,
            shader: info.fragment.shader,
        },
        vertex: PipelineShaderInfo {
            entry_point: vertex_entry_point,
            shader: info.vertex.shader,
        },
    };
    // crate::wasmic::print(&format!("{:?}", &info));
    new_pipeline(platform, context, info)
}

#[no_mangle]
fn taca_RenderingContext_newShader(platform: *mut Platform, _context: u32, _bytes: u32) -> u32 {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_newShader {context} {bytes}"
    // ));
    let platform = unsafe { &mut *platform };
    let span = unsafe { *((&platform.buffer[0..size_of::<Span>()]).as_ptr() as *const Span) };
    let mut buffer = vec![0u8; span.len as usize];
    unsafe {
        browser_read_app_span(buffer.as_mut_ptr(), span);
    }
    new_shader(platform, &buffer)
}

#[no_mangle]
fn taca_Window_get(_platform: *mut Platform) -> u32 {
    // crate::wasmic::print("taca_Window_get");
    1
}

#[no_mangle]
fn taca_Window_newRenderingContext(platform: *mut Platform, _window: u32) -> u32 {
    // crate::wasmic::print(&format!("taca_Window_newRenderingContext {window}"));
    new_rendering_context(unsafe { &mut *platform })
}
