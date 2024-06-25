#![allow(non_snake_case)]

use crate::platform::{Platform, WindowState};
use crate::wasmic::help::{
    apply_bindings, apply_pipeline, apply_uniforms, begin_pass, build_title, commit_frame, draw,
    end_pass, new_buffer, new_pipeline, new_rendering_context, new_shader, Bindings, BufferSlice,
    ExternBindings, ExternPipelineInfo, PipelineInfo, PipelineShaderInfo, Span, VertexAttribute,
};
use lz4_flex::frame::FrameDecoder;
use miniquad::{conf, EventHandler};
use std::{io::Read, mem::size_of};

struct App {
    platform: *mut Platform,
}

impl EventHandler for App {
    fn update(&mut self) {}

    fn draw(&mut self) {
        unsafe {
            browser_send_event(0);
        }
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        let platform = unsafe { &mut *self.platform };
        platform.window_state.pointer = [x, y];
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        let platform = unsafe { &mut *self.platform };
        platform.window_state.size = [width, height];
    }
}

pub fn wasmish() {
    let mut platform = Box::new(Platform::new(1024));
    let buffer_ptr = platform.buffer.as_mut_ptr();
    let buffer_len = platform.buffer.len();
    let platform = Box::into_raw(platform);
    unsafe {
        browser_load_app(platform, buffer_ptr, buffer_len);
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

    #[link_name = "startApp"]
    pub fn browser_start_app();
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

#[repr(C)]
pub struct VecInfo {
    ptr: *mut u8,
    len: usize,
    capacity: usize,
}

impl VecInfo {
    pub fn from_vec(vec: Vec<u8>) -> *mut VecInfo {
        let mut vec = std::mem::ManuallyDrop::new(vec);
        let vec = VecInfo {
            ptr: vec.as_mut_ptr(),
            len: vec.len(),
            capacity: vec.capacity(),
        };
        Box::into_raw(Box::new(vec))
    }
}

#[no_mangle]
pub extern "C" fn taca_alloc(len: usize) -> *mut VecInfo {
    let vec = vec![0u8; len];
    VecInfo::from_vec(vec)
}

#[no_mangle]
pub extern "C" fn taca_crate_version() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn taca_free(ptr: *mut u8, len: usize, capacity: usize) {
    let vec = VecInfo { ptr, len, capacity };
    let _ = unsafe { Vec::from_raw_parts(vec.ptr, vec.len, vec.capacity) };
}

#[no_mangle]
pub extern "C" fn taca_decompress(ptr: *mut u8, len: usize) -> *mut VecInfo {
    let source = unsafe { std::slice::from_raw_parts(ptr, len) };
    let mut dest = vec![0u8; 0];
    FrameDecoder::new(source).read_to_end(&mut dest).unwrap();
    VecInfo::from_vec(dest)
}

#[no_mangle]
fn taca_start(platform: *mut Platform) {
    let mut conf = conf::Conf::default();
    conf.platform.apple_gfx_api = conf::AppleGfxApi::Metal;
    conf.platform.webgl_version = conf::WebGLVersion::WebGL2;
    let platform = unsafe { &mut *platform };
    conf.window_title = build_title(platform);
    miniquad::start(conf, move || {
        platform.init_state();
        unsafe {
            browser_start_app();
        }
        Box::new(App { platform })
    })
}

#[no_mangle]
fn taca_RenderingContext_applyBindings(platform: *mut Platform, context: u32, _bindings: u32) {
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
    let platform = unsafe { &mut *platform };
    begin_pass(platform, context);
}

#[no_mangle]
fn taca_RenderingContext_commitFrame(platform: *mut Platform, context: u32) {
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
    let platform = unsafe { &mut *platform };
    draw(platform, context, item_begin, item_count, instance_count);
}

#[no_mangle]
fn taca_RenderingContext_endPass(platform: *mut Platform, context: u32) {
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
    let platform = unsafe { &mut *platform };
    let slice = unsafe {
        &*((&platform.buffer[0..size_of::<BufferSlice>()]).as_ptr() as *const BufferSlice)
    };
    let mut buffer = vec![0u8; slice.size as usize];
    unsafe {
        browser_read_app_memory(buffer.as_mut_ptr(), slice.ptr, slice.size);
    }
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
    let platform = unsafe { &mut *platform };
    let info = unsafe {
        &*((&platform.buffer[0..size_of::<ExternPipelineInfo>()]).as_ptr()
            as *const ExternPipelineInfo)
    };
    let attributes = unsafe { browser_read_app_vec::<VertexAttribute>(info.attributes) };
    let fragment_entry_point = unsafe { browser_read_app_string(info.fragment.entry_point) };
    let vertex_entry_point = unsafe { browser_read_app_string(info.vertex.entry_point) };
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
    new_pipeline(platform, context, info)
}

#[no_mangle]
fn taca_RenderingContext_newShader(platform: *mut Platform, _context: u32, _bytes: u32) -> u32 {
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
    1
}

#[no_mangle]
fn taca_Window_newRenderingContext(platform: *mut Platform) -> u32 {
    new_rendering_context(unsafe { &mut *platform })
}

#[no_mangle]
fn taca_Window_print(platform: *mut Platform, _text: u32) {
    let platform = unsafe { &mut *platform };
    let text = unsafe { *((&platform.buffer[0..size_of::<Span>()]).as_ptr() as *const Span) };
    let text = unsafe { browser_read_app_string(text) };
    // This is roundabout, but it lets us filter things here in engine code in the future.
    print(&text);
}

#[no_mangle]
fn taca_Window_setTitle(platform: *mut Platform, textPtr: u32, textLen: u32) {
    let platform = unsafe { &mut *platform };
    let text = Span {
        ptr: textPtr,
        len: textLen,
    };
    let text = unsafe { browser_read_app_string(text) };
    platform.title = Some(text);
}

#[no_mangle]
fn taca_Window_state(platform: *mut Platform, _result: u32) {
    let platform = unsafe { &mut *platform };
    let state = &platform.window_state as *const WindowState as *const u8;
    let size = size_of::<WindowState>();
    let state = unsafe { std::slice::from_raw_parts(state, size) };
    platform.buffer[0..size].copy_from_slice(state);
}
