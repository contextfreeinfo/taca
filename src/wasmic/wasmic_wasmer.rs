#![allow(non_snake_case)]

use wasmer::{imports, Function, FunctionEnv, FunctionEnvMut, Instance, Module, Store, WasmPtr};

use crate::platform::Platform;

use super::help::{new_buffer, BufferSlice};

pub fn wasmish(wasm: &[u8]) -> anyhow::Result<()> {
    let mut store = Store::default();
    let module = Module::new(&store, wasm)?;
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
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let env_mut = env.as_mut(&mut store);
    env_mut.memory = Some(instance.exports.get_memory("memory")?.clone());

    let start = instance.exports.get_function("_start")?;
    start.call(&mut store, &[])?;

    let listen = instance.exports.get_function("listen")?;
    let _ = listen;

    Ok(())
}

pub fn print(text: &str) {
    println!("{text}");
}

fn taca_RenderingContext_applyBindings(
    mut _env: FunctionEnvMut<Platform>,
    context: u32,
    bindings: u32,
) {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_applyBindings {context} {bindings}"
    ));
}

fn taca_RenderingContext_applyPipeline(
    mut _env: FunctionEnvMut<Platform>,
    context: u32,
    pipeline: u32,
) {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_applyPipeline {context} {pipeline}"
    ));
}

fn taca_RenderingContext_beginPass(mut _env: FunctionEnvMut<Platform>, context: u32) {
    crate::wasmic::print(&format!("taca_RenderingContext_beginPass {context}"));
}

fn taca_RenderingContext_commitFrame(mut _env: FunctionEnvMut<Platform>, context: u32) {
    crate::wasmic::print(&format!("taca_RenderingContext_commitFrame {context}"));
}

fn taca_RenderingContext_draw(
    mut _env: FunctionEnvMut<Platform>,
    context: u32,
    base_element: u32,
    num_elements: u32,
    num_instances: u32,
) {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_draw {context} {base_element} {num_elements} {num_instances}"
    ));
}

fn taca_RenderingContext_endPass(mut _env: FunctionEnvMut<Platform>, context: u32) {
    crate::wasmic::print(&format!("taca_RenderingContext_endPass {context}"));
}

fn taca_RenderingContext_newBuffer(
    mut env: FunctionEnvMut<Platform>,
    context: u32,
    typ: u32,
    usage: u32,
    slice: u32,
) -> u32 {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_newBuffer {context} {typ} {usage} {slice}"
    ));
    let (platform, store) = env.data_and_store_mut();
    let view = platform.memory.as_ref().unwrap().view(&store);
    // platform.memory.as_slice()[info..info+size_of(BufferSlice)];
    let slice = WasmPtr::<BufferSlice>::new(slice).read(&view).unwrap();
    crate::wasmic::print(&format!("{slice:?}"));
    let buffer = view
        .copy_range_to_vec(slice.ptr as u64..(slice.ptr + slice.size) as u64)
        .unwrap();
    new_buffer(
        &mut platform.context.0,
        typ,
        usage,
        &buffer,
        slice.item_size as usize,
    );
    0
}

fn taca_RenderingContext_newPipeline(
    mut _env: FunctionEnvMut<Platform>,
    context: u32,
    info: u32,
) -> u32 {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_newPipeline {context} {info}"
    ));
    0
}

fn taca_RenderingContext_newShader(
    mut _env: FunctionEnvMut<Platform>,
    context: u32,
    bytes: u32,
) -> u32 {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_newShader {context} {bytes}"
    ));
    0
}

fn taca_Window_get(mut _env: FunctionEnvMut<Platform>) -> u32 {
    crate::wasmic::print("taca_Window_get");
    1
}

fn taca_Window_newRenderingContext(mut _env: FunctionEnvMut<Platform>, window: u32) -> u32 {
    crate::wasmic::print(&format!("taca_Window_newRenderingContext {window}"));
    1
}
