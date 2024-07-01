use std::{fs::File, io::Read};

use lz4_flex::frame::FrameDecoder;
use wasmer::{
    imports, Function, FunctionEnv, FunctionEnvMut, Instance, Memory, MemoryView, Module, Store,
    ValueType, WasmPtr,
};

pub struct App {
    pub memory: Option<Memory>,
}

impl App {
    fn new() -> App {
        App { memory: None }
    }
}

pub struct Wrap {
    env: FunctionEnv<App>,
    instance: Instance,
    store: Store,
}

impl Wrap {
    fn init(wasm: &[u8]) -> Wrap {
        let mut store = Store::default();
        let module = Module::new(&store, wasm).unwrap();
        let env = FunctionEnv::new(&mut store, App::new());
        let import_object = imports! {
            "env" => {
                "taca_RenderingContext_applyBindings" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_applyBindings),
                "taca_RenderingContext_applyPipeline" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_applyPipeline),
                "taca_RenderingContext_applyUniforms" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_applyUniforms),
                "taca_RenderingContext_beginPass" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_beginPass),
                "taca_RenderingContext_commitFrame" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_commitFrame),
                "taca_RenderingContext_draw" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_draw),
                "taca_RenderingContext_endPass" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_endPass),
                "taca_RenderingContext_newBuffer" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_newBuffer),
                "taca_RenderingContext_newPipeline" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_newPipeline),
                "taca_RenderingContext_newShader" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_newShader),
                "taca_Window_get" => Function::new_typed_with_env(&mut store, &env, taca_Window_get),
                "taca_Window_newRenderingContext" => Function::new_typed_with_env(&mut store, &env, taca_Window_newRenderingContext),
                "taca_Window_print" => Function::new_typed_with_env(&mut store, &env, taca_Window_print),
                "taca_Window_setTitle" => Function::new_typed_with_env(&mut store, &env, taca_Window_setTitle),
                "taca_Window_state" => Function::new_typed_with_env(&mut store, &env, taca_Window_state),
            }
        };
        let instance = Instance::new(&mut store, &module, &import_object).unwrap();
        let app = env.as_mut(&mut store);
        app.memory = Some(instance.exports.get_memory("memory").unwrap().clone());
        if let Ok(config) = instance.exports.get_function("config") {
            config.call(&mut store, &[]).unwrap();
        }
        Wrap {
            env,
            instance,
            store,
        }
    }

    pub fn load(path: &str) -> Wrap {
        let mut buf = Vec::<u8>::new();
        File::open(path)
            .expect("Bad open")
            .read_to_end(&mut buf)
            .expect("Bad read");
        if buf[0] == 0x04 {
            // Presume lz4 compressed since wasm starts with 0x00.
            let mut dest = vec![0u8; 0];
            FrameDecoder::new(&buf as &[u8])
                .read_to_end(&mut dest)
                .unwrap();
            buf = dest;
        }
        Wrap::init(&buf)
    }
}

#[derive(Clone, Copy, Debug, ValueType)]
#[repr(C)]
pub struct Span {
    pub ptr: u32,
    pub len: u32,
}

fn read_string(view: &MemoryView, span: Span) -> String {
    WasmPtr::<u8>::new(span.ptr)
        .read_utf8_string(&view, span.len)
        .unwrap()
}

fn taca_RenderingContext_applyBindings(mut env: FunctionEnvMut<App>, context: u32, bindings: u32) {
    // let (platform, store) = env.data_and_store_mut();
    // let view = platform.memory.as_ref().unwrap().view(&store);
    // let bindings = WasmPtr::<ExternBindings>::new(bindings)
    //     .read(&view)
    //     .unwrap();
    // let bindings = Bindings {
    //     vertex_buffers: read_span(&view, bindings.vertex_buffers),
    //     index_buffer: bindings.index_buffer,
    // };
    // apply_bindings(platform, context, bindings);
}

fn taca_RenderingContext_applyPipeline(mut env: FunctionEnvMut<App>, context: u32, pipeline: u32) {
    // let platform = env.data_mut();
    // apply_pipeline(platform, context, pipeline)
}

fn taca_RenderingContext_applyUniforms(mut env: FunctionEnvMut<App>, context: u32, bytes: u32) {
    // let (platform, store) = env.data_and_store_mut();
    // let view = platform.memory.as_ref().unwrap().view(&store);
    // let uniforms = WasmPtr::<Span>::new(bytes).read(&view).unwrap();
    // let uniforms = read_span::<u8>(&view, uniforms);
    // apply_uniforms(platform, context, &uniforms);
}

fn taca_RenderingContext_beginPass(mut env: FunctionEnvMut<App>, context: u32) {
    // let platform = env.data_mut();
    // begin_pass(platform, context)
}

fn taca_RenderingContext_commitFrame(mut env: FunctionEnvMut<App>, context: u32) {
    // let platform = env.data_mut();
    // commit_frame(platform, context)
}

fn taca_RenderingContext_draw(
    mut env: FunctionEnvMut<App>,
    context: u32,
    item_begin: i32,
    item_count: i32,
    instance_count: i32,
) {
    // let platform = env.data_mut();
    // draw(platform, context, item_begin, item_count, instance_count);
}

fn taca_RenderingContext_endPass(mut env: FunctionEnvMut<App>, context: u32) {
    // let platform = env.data_mut();
    // end_pass(platform, context)
}

fn taca_RenderingContext_newBuffer(
    mut env: FunctionEnvMut<App>,
    context: u32,
    typ: u32,
    usage: u32,
    slice: u32,
) -> u32 {
    // let (platform, store) = env.data_and_store_mut();
    // let view = platform.memory.as_ref().unwrap().view(&store);
    // let slice = WasmPtr::<BufferSlice>::new(slice).read(&view).unwrap();
    // let buffer = view
    //     .copy_range_to_vec(slice.ptr as u64..(slice.ptr + slice.size) as u64)
    //     .unwrap();
    // new_buffer(
    //     platform,
    //     context,
    //     typ,
    //     usage,
    //     &buffer,
    //     slice.item_size as usize,
    // )
    0
}

fn taca_RenderingContext_newPipeline(mut env: FunctionEnvMut<App>, context: u32, info: u32) -> u32 {
    // let (platform, store) = env.data_and_store_mut();
    // let view = platform.memory.as_ref().unwrap().view(&store);
    // let info = WasmPtr::<ExternPipelineInfo>::new(info)
    //     .read(&view)
    //     .unwrap();
    // let attributes = read_span(&view, info.attributes);
    // let info = PipelineInfo {
    //     attributes,
    //     fragment: PipelineShaderInfo {
    //         entry_point: read_string(&view, info.fragment.entry_point),
    //         shader: info.fragment.shader,
    //     },
    //     vertex: PipelineShaderInfo {
    //         entry_point: read_string(&view, info.vertex.entry_point),
    //         shader: info.vertex.shader,
    //     },
    // };
    // new_pipeline(platform, context, info)
    0
}

fn taca_RenderingContext_newShader(mut env: FunctionEnvMut<App>, _context: u32, bytes: u32) -> u32 {
    // let (platform, store) = env.data_and_store_mut();
    // let view = platform.memory.as_ref().unwrap().view(&store);
    // let bytes = WasmPtr::<Span>::new(bytes).read(&view).unwrap();
    // let bytes = read_span(&view, bytes);
    // new_shader(platform, &bytes)
    0
}

fn taca_Window_get(mut _env: FunctionEnvMut<App>) -> u32 {
    1
}

fn taca_Window_newRenderingContext(mut env: FunctionEnvMut<App>) -> u32 {
    // new_rendering_context(env.data_mut())
    0
}

fn taca_Window_print(mut env: FunctionEnvMut<App>, text: u32) {
    let (platform, store) = env.data_and_store_mut();
    let view = platform.memory.as_ref().unwrap().view(&store);
    let text = WasmPtr::<Span>::new(text).read(&view).unwrap();
    let text = read_string(&view, text);
    println!("{text}");
}

fn taca_Window_setTitle(mut env: FunctionEnvMut<App>, text: u32) {
    // let (platform, store) = env.data_and_store_mut();
    // let view = platform.memory.as_ref().unwrap().view(&store);
    // let text = WasmPtr::<Span>::new(text).read(&view).unwrap();
    // let text = read_string(&view, text);
    // platform.title = Some(text);
}

fn taca_Window_state(mut env: FunctionEnvMut<App>, result: u32) {
    // let (platform, store) = env.data_and_store_mut();
    // let view = platform.memory.as_ref().unwrap().view(&store);
    // WasmRef::<WindowState>::new(&view, result as u64)
    //     .write(platform.window_state)
    //     .unwrap();
}
