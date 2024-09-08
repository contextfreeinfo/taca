#![allow(non_snake_case)]

use std::{
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
};

use lz4_flex::frame::FrameDecoder;
use wasmer::{
    imports, Function, FunctionEnv, FunctionEnvMut, Instance, Memory, MemoryView, Module, Store,
    Value, ValueType, WasmPtr, WasmRef,
};
use winit::event_loop::EventLoop;

use crate::{
    display::{Display, EventKind, Graphics, MaybeGraphics, WindowState},
    gpu::{
        bindings_apply, bound_ensure, buffer_update, buffered_ensure, create_buffer,
        create_pipeline, end_pass, frame_commit, pass_ensure, pipeline_apply, pipelined_ensure,
        shader_create, uniforms_apply, Binding, Bindings, Buffer, BufferSlice, ExternBindings,
        ExternPipelineInfo, Pipeline, PipelineInfo, PipelineShaderInfo, RenderFrame, Shader, Span,
    },
    key::KeyEvent,
    text::{to_text_align_x, to_text_align_y, TextEngine},
};

pub struct App {
    pub env: FunctionEnv<System>,
    pub update: Function,
    pub instance: Instance,
    pub store: Store,
}

pub struct AppPtr(pub *mut App);

unsafe impl Send for AppPtr {}

impl App {
    fn init(wasm: &[u8], display: Display) -> App {
        let mut store = Store::default();
        let module = Module::new(&store, wasm).unwrap();
        let env = FunctionEnv::new(&mut store, System::new(display));
        let import_object = imports! {
            "env" => {
                "taca_bindings_apply" => Function::new_typed_with_env(&mut store, &env, taca_bindings_apply),
                "taca_pipeline_apply" => Function::new_typed_with_env(&mut store, &env, taca_pipeline_apply),
                "taca_uniforms_apply" => Function::new_typed_with_env(&mut store, &env, taca_uniforms_apply),
                "taca_RenderingContext_beginPass" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_beginPass),
                "taca_RenderingContext_commitFrame" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_commitFrame),
                "taca_draw" => Function::new_typed_with_env(&mut store, &env, taca_draw),
                "taca_text_draw" => Function::new_typed_with_env(&mut store, &env, taca_text_draw),
                "taca_text_drawure" => Function::new_typed_with_env(&mut store, &env, taca_text_drawure),
                "taca_RenderingContext_endPass" => Function::new_typed_with_env(&mut store, &env, taca_RenderingContext_endPass),
                "taca_binding_apply" => Function::new_typed_with_env(&mut store, &env, taca_binding_apply),
                "taca_binding_new" => Function::new_typed_with_env(&mut store, &env, taca_binding_new),
                "taca_buffer_new" => Function::new_typed_with_env(&mut store, &env, taca_buffer_new),
                "taca_buffer_update" => Function::new_typed_with_env(&mut store, &env, taca_buffer_update),
                "taca_key_event" => Function::new_typed_with_env(&mut store, &env, taca_key_event),
                "taca_pipeline_new" => Function::new_typed_with_env(&mut store, &env, taca_pipeline_new),
                "taca_shader_new" => Function::new_typed_with_env(&mut store, &env, taca_shader_new),
                "taca_Window_get" => Function::new_typed_with_env(&mut store, &env, taca_Window_get),
                "taca_Window_newRenderingContext" => Function::new_typed_with_env(&mut store, &env, taca_Window_newRenderingContext),
                "taca_print" => Function::new_typed_with_env(&mut store, &env, taca_print),
                "taca_title_update" => Function::new_typed_with_env(&mut store, &env, taca_title_update),
                "taca_window_state" => Function::new_typed_with_env(&mut store, &env, taca_window_state),
                "taca_text_align" => Function::new_typed_with_env(&mut store, &env, taca_text_align),
            }
        };
        let instance = Instance::new(&mut store, &module, &import_object).unwrap();
        let update = instance.exports.get_function("update").unwrap().clone();
        let app = env.as_mut(&mut store);
        app.memory = Some(instance.exports.get_memory("memory").unwrap().clone());
        App {
            env,
            instance,
            update,
            store,
        }
    }

    pub fn listen(&mut self) {
        self.update
            .call(&mut self.store, &[Value::I32(EventKind::Tick as i32)])
            .unwrap();
        let system = self.env.as_mut(&mut self.store);
        frame_commit(system);
    }

    pub fn load(path: &str, display: Display) -> App {
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
        App::init(&buf, display)
    }

    pub fn run(&mut self, event_loop: EventLoop<Graphics>, ptr: *mut App) {
        let system = self.env.as_mut(&mut self.store);
        system.display.app = AppPtr(ptr);
        system.display.run(event_loop);
    }

    pub fn start(&mut self, graphics: &Graphics) {
        let system = self.env.as_mut(&mut self.store);
        system.text = Some(Arc::new(Mutex::new(TextEngine::new(graphics))));
        // Some wasi builds make _initialize even without main/_start.
        // If _start exists, it's supposed to do any initialize on its own, I think.
        if let Ok(initialize) = self.instance.exports.get_function("_initialize") {
            initialize.call(&mut self.store, &[]).unwrap();
        }
        // Part of why to move away from main/_start so we know any _initialize
        // is separate from our own start.
        let start = self.instance.exports.get_function("start").unwrap();
        start.call(&mut self.store, &[]).unwrap();
    }
}

pub struct System {
    pub bindings: Vec<Binding>,
    pub buffers: Vec<Buffer>,
    pub display: Display,
    pub frame: Option<RenderFrame>,
    pub key_event: KeyEvent,
    pub memory: Option<Memory>,
    pub pipelines: Vec<Pipeline>,
    pub shaders: Vec<Shader>,
    pub text: Option<Arc<Mutex<TextEngine>>>,
}

impl System {
    fn new(display: Display) -> System {
        System {
            bindings: vec![],
            buffers: vec![],
            display,
            key_event: Default::default(),
            memory: None,
            frame: None,
            pipelines: vec![],
            shaders: vec![],
            text: None,
        }
    }
}

fn read_span<T>(view: &MemoryView, span: Span) -> Vec<T>
where
    T: Copy + ValueType,
{
    match span.len {
        0 => vec![],
        _ => WasmPtr::<T>::new(span.ptr)
            .slice(&view, span.len)
            .unwrap()
            .read_to_vec()
            .unwrap(),
    }
}

fn read_string(view: &MemoryView, span: Span) -> String {
    match span.len {
        0 => "".into(),
        _ => WasmPtr::<u8>::new(span.ptr)
            .read_utf8_string(&view, span.len)
            .unwrap(),
    }
}

fn taca_binding_apply(mut env: FunctionEnvMut<System>, binding: u32, bytes: u32) {
    // TOOD Consider this more.
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let _ = binding;
    // let binding = &system.bindings[binding as usize];
    // let bindings = Bindings {
    //     vertex_buffers: &binding.vertex_buffers,
    //     index_buffer: binding.index_buffer,
    // };
    // bindings_apply(system, binding as usize);
    let uniforms = WasmPtr::<Span>::new(bytes).read(&view).unwrap();
    let uniforms = read_span::<u8>(&view, uniforms);
    uniforms_apply(system, &uniforms);
}

fn taca_binding_new(mut env: FunctionEnvMut<System>, binding: u32) -> u32 {
    // TOOD Consider this more.
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let binding = WasmPtr::<ExternBindings>::new(binding).read(&view).unwrap();
    // TODO Reusable buffer to read into!
    let vertex_buffers = read_span(&view, binding.vertex_buffers);
    let binding = Binding {
        bind_groups: vec![],
        index_buffer: binding.index_buffer,
        vertex_buffers: vertex_buffers,
    };
    system.bindings.push(binding);
    system.bindings.len() as u32
}

fn taca_bindings_apply(mut env: FunctionEnvMut<System>, bindings: u32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let bindings = WasmPtr::<ExternBindings>::new(bindings)
        .read(&view)
        .unwrap();
    // TODO Reusable buffer to read into!
    let vertex_buffers = read_span(&view, bindings.vertex_buffers);
    let bindings = Bindings {
        vertex_buffers: &vertex_buffers,
        index_buffer: bindings.index_buffer,
    };
    bindings_apply(system, bindings);
}

fn taca_pipeline_apply(mut env: FunctionEnvMut<System>, pipeline: u32) {
    let system = env.data_mut();
    pipeline_apply(system, pipeline);
}

fn taca_uniforms_apply(mut env: FunctionEnvMut<System>, bytes: u32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let uniforms = WasmPtr::<Span>::new(bytes).read(&view).unwrap();
    let uniforms = read_span::<u8>(&view, uniforms);
    uniforms_apply(system, &uniforms);
}

fn taca_RenderingContext_beginPass(mut env: FunctionEnvMut<System>) {
    pass_ensure(env.data_mut());
}

fn taca_RenderingContext_commitFrame(mut env: FunctionEnvMut<System>) {
    let system = env.data_mut();
    frame_commit(system);
}

fn taca_draw(
    mut env: FunctionEnvMut<System>,
    item_begin: u32,
    item_count: u32,
    instance_count: u32,
) {
    let system = env.data_mut();
    pipelined_ensure(system);
    // TODO Actually ensure we got buffers?
    buffered_ensure(system);
    bound_ensure(system);
    let Some(RenderFrame {
        pass: Some(pass), ..
    }) = &mut system.frame
    else {
        return;
    };
    pass.draw_indexed(item_begin..item_begin + item_count, 0, 0..instance_count);
}

fn taca_text_draw(mut env: FunctionEnvMut<System>, text: u32, x: f32, y: f32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let text = WasmPtr::<Span>::new(text).read(&view).unwrap();
    let text = read_string(&view, text);
    pass_ensure(system);
    let text_engine = system.text.clone().unwrap();
    text_engine.lock().unwrap().draw(system, &text, x, y);
}

fn taca_text_align(mut env: FunctionEnvMut<System>, x: u32, y: u32) {
    let system = env.data_mut();
    let text_engine = system.text.clone().unwrap();
    let mut text_engine = text_engine.lock().unwrap();
    text_engine.align_x = to_text_align_x(x);
    text_engine.align_y = to_text_align_y(y);
}

fn taca_text_drawure(
    mut _env: FunctionEnvMut<System>,
    _texture: u32,
    _x: f32,
    _y: f32,
) {
    // TODO
}

fn taca_RenderingContext_endPass(mut env: FunctionEnvMut<System>) {
    let system = env.data_mut();
    end_pass(system);
}

fn taca_buffer_new(mut env: FunctionEnvMut<System>, typ: u32, slice: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let slice = WasmPtr::<BufferSlice>::new(slice).read(&view).unwrap();
    let contents = match slice.ptr {
        0 => None,
        _ => Some(
            view.copy_range_to_vec(slice.ptr as u64..(slice.ptr + slice.size) as u64)
                .unwrap(),
        ),
    };
    create_buffer(system, contents.as_deref(), slice.size, typ);
    system.buffers.len() as u32
}

fn taca_buffer_update(mut env: FunctionEnvMut<System>, buffer: u32, bytes: u32, offset: u32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let bytes = WasmPtr::<Span>::new(bytes).read(&view).unwrap();
    let bytes = read_span::<u8>(&view, bytes);
    buffer_update(system, buffer, &bytes, offset);
}

fn taca_key_event(mut env: FunctionEnvMut<System>, result: u32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    WasmPtr::<KeyEvent>::new(result)
        .write(&view, system.key_event)
        .unwrap();
}

fn taca_pipeline_new(mut env: FunctionEnvMut<System>, info: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let info = WasmPtr::<ExternPipelineInfo>::new(info)
        .read(&view)
        .unwrap();
    // dbg!(info);
    let vertex_attributes = read_span(&view, info.vertex_attributes);
    let vertex_buffers = read_span(&view, info.vertex_buffers);
    let info = PipelineInfo {
        depth_test: info.depth_test,
        fragment: PipelineShaderInfo {
            entry_point: read_string(&view, info.fragment.entry_point),
            shader: info.fragment.shader,
        },
        vertex: PipelineShaderInfo {
            entry_point: read_string(&view, info.vertex.entry_point),
            shader: info.vertex.shader,
        },
        vertex_attributes,
        vertex_buffers,
    };
    create_pipeline(system, info);
    system.pipelines.len() as u32
}

fn taca_shader_new(mut env: FunctionEnvMut<System>, bytes: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let bytes = WasmPtr::<Span>::new(bytes).read(&view).unwrap();
    let bytes = read_span(&view, bytes);
    let shader = shader_create(system, &bytes);
    system.shaders.push(shader);
    system.shaders.len() as u32
}

fn taca_Window_get(_env: FunctionEnvMut<System>) -> u32 {
    1
}

fn taca_Window_newRenderingContext(_env: FunctionEnvMut<System>) -> u32 {
    1
}

fn taca_print(mut env: FunctionEnvMut<System>, text: u32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let text = WasmPtr::<Span>::new(text).read(&view).unwrap();
    let text = read_string(&view, text);
    println!("{text}");
}

fn taca_title_update(mut env: FunctionEnvMut<System>, text: u32) {
    let (system, store) = env.data_and_store_mut();
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        return;
    };
    let view = system.memory.as_ref().unwrap().view(&store);
    let title = WasmPtr::<Span>::new(text).read(&view).unwrap();
    let title = read_string(&view, title);
    gfx.window.as_ref().set_title(&title);
}

fn taca_window_state(mut env: FunctionEnvMut<System>, result: u32) {
    let (system, store) = env.data_and_store_mut();
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        return;
    };
    let pointer = system.display.pointer_pos.unwrap_or(Default::default());
    let size = gfx.window.inner_size();
    let view = system.memory.as_ref().unwrap().view(&store);
    let state = WindowState {
        pointer: [pointer.x as f32, pointer.y as f32],
        press: system.display.pointer_press,
        size: [size.width as f32, size.height as f32],
    };
    WasmRef::<WindowState>::new(&view, result as u64)
        .write(state)
        .unwrap();
}
