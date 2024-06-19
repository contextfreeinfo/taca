#![allow(non_snake_case)]

use miniquad::EventHandler;
use wasmer::{
    imports, Function, FunctionEnv, FunctionEnvMut, Instance, MemoryView, Module, Store, Value,
    ValueType, WasmPtr,
};

use crate::platform::Platform;

use super::help::{
    apply_bindings, apply_pipeline, begin_pass, commit_frame, draw, end_pass, new_buffer,
    new_pipeline, new_rendering_context, new_shader, Bindings, BufferSlice, ExternBindings,
    ExternPipelineInfo, PipelineInfo, PipelineShaderInfo, Span,
};

pub struct App {
    listen: Function,
    store: Store,
}

impl App {
    pub fn new(store: Store, instance: Instance) -> Self {
        let listen = instance.exports.get_function("listen").unwrap().clone();
        Self { listen, store }
    }
}

impl<'a> EventHandler for App {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.listen.call(&mut self.store, &[Value::I32(0)]).unwrap();
    }
}

pub fn wasmish(wasm: &[u8]) -> App {
    let mut store = Store::default();
    let module = Module::new(&store, wasm).unwrap();
    let env = FunctionEnv::new(&mut store, Platform::new(0));
    let import_object = imports! {
        "env" => {
            "taca_RenderingContext_applyBindings" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_applyBindings),
            "taca_RenderingContext_applyPipeline" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_applyPipeline),
            "taca_RenderingContext_beginPass" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_beginPass),
            "taca_RenderingContext_commitFrame" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_commitFrame),
            "taca_RenderingContext_draw" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_draw),
            "taca_RenderingContext_endPass" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_endPass),
            "taca_RenderingContext_newBuffer" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_newBuffer),
            "taca_RenderingContext_newPipeline" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_newPipeline),
            "taca_RenderingContext_newShader" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_newShader),
            "taca_Window_get" => Function::new_typed_with_env(&mut store, &env, taca_Window_get),
            "taca_Window_newRenderingContext" => Function::new_typed_with_env(&mut store, &env, taca_Window_newRenderingContext),
        }
    };
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();
    let env_mut = env.as_mut(&mut store);
    env_mut.memory = Some(instance.exports.get_memory("memory").unwrap().clone());
    // Start up.
    let start = instance.exports.get_function("_start").unwrap();
    start.call(&mut store, &[]).unwrap();
    App::new(store, instance)
}

pub fn print(text: &str) {
    println!("{text}");
}

fn read_span<T>(view: &MemoryView, span: Span) -> Vec<T>
where
    T: Copy + ValueType,
{
    WasmPtr::<T>::new(span.ptr)
        .slice(&view, span.len)
        .unwrap()
        .read_to_vec()
        .unwrap()
}

fn read_string(view: &MemoryView, span: Span) -> String {
    WasmPtr::<u8>::new(span.ptr)
        .read_utf8_string(&view, span.len)
        .unwrap()
}

fn taca_RenderingContext_applyBindings(
    mut env: FunctionEnvMut<Platform>,
    context: u32,
    bindings: u32,
) {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_applyBindings {context} {bindings}"
    // ));
    let (platform, store) = env.data_and_store_mut();
    let view = platform.memory.as_ref().unwrap().view(&store);
    let bindings = WasmPtr::<ExternBindings>::new(bindings)
        .read(&view)
        .unwrap();
    let bindings = Bindings {
        vertex_buffers: read_span(&view, bindings.vertex_buffers),
        index_buffer: bindings.index_buffer,
    };
    apply_bindings(platform, context, bindings);
}

fn taca_RenderingContext_applyPipeline(
    mut env: FunctionEnvMut<Platform>,
    context: u32,
    pipeline: u32,
) {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_applyPipeline {context} {pipeline}"
    // ));
    let platform = env.data_mut();
    apply_pipeline(platform, context, pipeline)
}

fn taca_RenderingContext_beginPass(mut env: FunctionEnvMut<Platform>, context: u32) {
    // crate::wasmic::print(&format!("taca_RenderingContext_beginPass {context}"));
    let platform = env.data_mut();
    begin_pass(platform, context)
}

fn taca_RenderingContext_commitFrame(mut env: FunctionEnvMut<Platform>, context: u32) {
    // crate::wasmic::print(&format!("taca_RenderingContext_commitFrame {context}"));
    let platform = env.data_mut();
    commit_frame(platform, context)
}

fn taca_RenderingContext_draw(
    mut env: FunctionEnvMut<Platform>,
    context: u32,
    item_begin: i32,
    item_count: i32,
    instance_count: i32,
) {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_draw {context} {base_element} {num_elements} {num_instances}"
    // ));
    let platform = env.data_mut();
    draw(platform, context, item_begin, item_count, instance_count);
}

fn taca_RenderingContext_endPass(mut env: FunctionEnvMut<Platform>, context: u32) {
    // crate::wasmic::print(&format!("taca_RenderingContext_endPass {context}"));
    let platform = env.data_mut();
    end_pass(platform, context)
}

fn taca_RenderingContext_newBuffer(
    mut env: FunctionEnvMut<Platform>,
    context: u32,
    typ: u32,
    usage: u32,
    slice: u32,
) -> u32 {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_newBuffer {context} {typ} {usage} {slice}"
    // ));
    let (platform, store) = env.data_and_store_mut();
    let view = platform.memory.as_ref().unwrap().view(&store);
    let slice = WasmPtr::<BufferSlice>::new(slice).read(&view).unwrap();
    // crate::wasmic::print(&format!("{slice:?}"));
    let buffer = view
        .copy_range_to_vec(slice.ptr as u64..(slice.ptr + slice.size) as u64)
        .unwrap();
    new_buffer(
        platform,
        context,
        typ,
        usage,
        &buffer,
        slice.item_size as usize,
    )
}

fn taca_RenderingContext_newPipeline(
    mut env: FunctionEnvMut<Platform>,
    context: u32,
    info: u32,
) -> u32 {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_newPipeline {context} {info}"
    // ));
    let (platform, store) = env.data_and_store_mut();
    let view = platform.memory.as_ref().unwrap().view(&store);
    let info = WasmPtr::<ExternPipelineInfo>::new(info)
        .read(&view)
        .unwrap();
    let attributes = read_span(&view, info.attributes);
    let info = PipelineInfo {
        attributes,
        fragment: PipelineShaderInfo {
            entry_point: read_string(&view, info.fragment.entry_point),
            shader: info.fragment.shader,
        },
        vertex: PipelineShaderInfo {
            entry_point: read_string(&view, info.vertex.entry_point),
            shader: info.vertex.shader,
        },
    };
    new_pipeline(platform, context, info)
}

fn taca_RenderingContext_newShader(
    mut env: FunctionEnvMut<Platform>,
    _context: u32,
    bytes: u32,
) -> u32 {
    // crate::wasmic::print(&format!(
    //     "taca_RenderingContext_newShader {context} {bytes}"
    // ));
    let (platform, store) = env.data_and_store_mut();
    let view = platform.memory.as_ref().unwrap().view(&store);
    let bytes = WasmPtr::<Span>::new(bytes).read(&view).unwrap();
    let bytes = read_span(&view, bytes);
    new_shader(platform, &bytes)
}

fn taca_Window_get(mut _env: FunctionEnvMut<Platform>) -> u32 {
    // crate::wasmic::print("taca_Window_get");
    1
}

fn taca_Window_newRenderingContext(mut env: FunctionEnvMut<Platform>, _window: u32) -> u32 {
    // crate::wasmic::print(&format!("taca_Window_newRenderingContext {window}"));
    new_rendering_context(env.data_mut())
}
