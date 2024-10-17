#![allow(non_snake_case)]

use std::{
    fs::File,
    io::Read,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread,
};

use kira::manager::{AudioManager, AudioManagerSettings};
use lz4_flex::frame::FrameDecoder;
use wasmer::{
    imports, Function, FunctionEnv, FunctionEnvMut, Instance, Memory, MemoryView, Module, Store,
    Value, ValueType, WasmPtr, WasmRef,
};
use winit::event_loop::EventLoop;

use crate::{
    display::{Display, EventKind, Graphics, MaybeGraphics, UserEvent, WindowState},
    gpu::{
        bindings_apply, bindings_new, bound_ensure, buffer_update, buffered_ensure, buffers_apply,
        create_buffer, create_pipeline, frame_commit, image_decode, image_to_texture, pass_ensure,
        pipeline_apply, pipelined_ensure, shader_create, sound_decode, uniforms_apply, Bindings,
        BindingsInfo, Buffer, BufferSlice, ExternBindingsInfo, ExternMeshBuffers,
        ExternPipelineInfo, MeshBuffers, Pipeline, PipelineInfo, PipelineShaderInfo, RenderFrame,
        Shader, Sound, SoundPlayInfoExtern, Span, Texture, TextureInfoExtern,
    },
    key::KeyEvent,
    text::{to_text_align_x, to_text_align_y, TextEngine},
    wasi,
};

pub struct App {
    pub env: FunctionEnv<System>,
    pub update: Option<Function>,
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
                "taca_bindings_new" => Function::new_typed_with_env(&mut store, &env, taca_bindings_new),
                "taca_buffer_new" => Function::new_typed_with_env(&mut store, &env, taca_buffer_new),
                "taca_buffer_update" => Function::new_typed_with_env(&mut store, &env, taca_buffer_update),
                "taca_buffers_apply" => Function::new_typed_with_env(&mut store, &env, taca_buffers_apply),
                "taca_clip" => Function::new_typed_with_env(&mut store, &env, taca_clip),
                "taca_draw" => Function::new_typed_with_env(&mut store, &env, taca_draw),
                "taca_image_decode" => Function::new_typed_with_env(&mut store, &env, taca_image_decode),
                "taca_key_event" => Function::new_typed_with_env(&mut store, &env, taca_key_event),
                "taca_pipeline_apply" => Function::new_typed_with_env(&mut store, &env, taca_pipeline_apply),
                "taca_pipeline_new" => Function::new_typed_with_env(&mut store, &env, taca_pipeline_new),
                "taca_print" => Function::new_typed_with_env(&mut store, &env, taca_print),
                "taca_shader_new" => Function::new_typed_with_env(&mut store, &env, taca_shader_new),
                "taca_sound_decode" => Function::new_typed_with_env(&mut store, &env, taca_sound_decode),
                "taca_sound_play" => Function::new_typed_with_env(&mut store, &env, taca_sound_play),
                "taca_text_align" => Function::new_typed_with_env(&mut store, &env, taca_text_align),
                "taca_text_draw" => Function::new_typed_with_env(&mut store, &env, taca_text_draw),
                "taca_texture_info" => Function::new_typed_with_env(&mut store, &env, taca_texture_info),
                "taca_title_update" => Function::new_typed_with_env(&mut store, &env, taca_title_update),
                "taca_uniforms_apply" => Function::new_typed_with_env(&mut store, &env, taca_uniforms_apply),
                "taca_window_state" => Function::new_typed_with_env(&mut store, &env, taca_window_state),
            },
            "wasi_snapshot_preview1" => {
                "args_get" => Function::new_typed_with_env(&mut store, &env, wasi::args_get),
                "args_sizes_get" => Function::new_typed_with_env(&mut store, &env, wasi::args_sizes_get),
                "fd_close" => Function::new_typed_with_env(&mut store, &env, wasi::fd_close),
                "fd_fdstat_get" => Function::new_typed_with_env(&mut store, &env, wasi::fd_fdstat_get),
                "fd_seek" => Function::new_typed_with_env(&mut store, &env, wasi::fd_seek),
                "fd_write" => Function::new_typed_with_env(&mut store, &env, wasi::fd_write),
                "proc_exit" => Function::new_typed(&mut store, wasi::proc_exit),
                "random_get" => Function::new_typed(&mut store, wasi::random_get),
            },
        };
        let instance = Instance::new(&mut store, &module, &import_object).unwrap();
        let update = instance.exports.get_function("update").ok().cloned();
        let app = env.as_mut(&mut store);
        app.memory = Some(instance.exports.get_memory("memory").unwrap().clone());
        App {
            env,
            instance,
            update,
            store,
        }
    }

    pub fn handle(&mut self, event: UserEvent) {
        match event {
            UserEvent::Graphics(_) => {} // handled in display
            UserEvent::ImageDecoded { handle, image } => {
                match image {
                    Ok(image) => {
                        let system = self.env.as_mut(&mut self.store);
                        image_to_texture(system, handle, image);
                    }
                    Err(err) => {
                        // TODO Report errors to app?
                        dbg!(err);
                    }
                }
                self.task_finish();
            }
            UserEvent::SoundDecoded { handle, sound } => {
                match sound {
                    Ok(sound) => {
                        let system = self.env.as_mut(&mut self.store);
                        // dbg!(sound.duration());
                        system.sounds[handle - 1].data = Some(sound);
                    }
                    Err(err) => {
                        // TODO Report errors to app?
                        dbg!(err);
                    }
                }
                self.task_finish();
            }
        }
    }

    pub fn listen(&mut self) {
        let Some(update) = &self.update else { return };
        update
            .call(&mut self.store, &[Value::I32(EventKind::Frame as i32)])
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

    pub fn run(&mut self, event_loop: EventLoop<UserEvent>, ptr: *mut App) {
        let system = self.env.as_mut(&mut self.store);
        // Set up worker thread, and detach. TODO Do we ever need it?
        let (sender, receiver) = channel();
        system.worker = Some(sender);
        let event_loop_proxy = event_loop.create_proxy();
        // TODO Arc mutex the receiver for multiple worker threads?
        thread::spawn(move || {
            for message in receiver {
                match message {
                    WorkItem::ImageDecode { handle, bytes } => {
                        image_decode(handle, bytes, &event_loop_proxy);
                    }
                    WorkItem::SoundDecode { handle, bytes } => {
                        sound_decode(handle, bytes, &event_loop_proxy);
                    }
                }
            }
        });
        // Run event loop.
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
        } else if let Ok(main) = self.instance.exports.get_function("_start") {
            main.call(&mut self.store, &[]).unwrap();
        }
        // Part of why to move away from main/_start so we know any _initialize
        // is separate from our own start.
        if let Ok(start) = self.instance.exports.get_function("start") {
            start.call(&mut self.store, &[]).unwrap();
        }
    }

    fn task_finish(&mut self) {
        let system = self.env.as_mut(&mut self.store);
        system.tasks_active -= 1;
        if system.tasks_active == 0 {
            let Some(update) = &self.update else { return };
            update
                .call(&mut self.store, &[Value::I32(EventKind::TasksDone as i32)])
                .unwrap();
        }
    }
}

pub struct System {
    pub audio_manager: Option<AudioManager>,
    pub bindings: Vec<Bindings>,
    pub bindings_updated: Vec<usize>, // TODO Track by buffer per queue instead?
    pub buffers: Vec<Buffer>,
    pub display: Display,
    pub frame: Option<RenderFrame>,
    pub key_event: KeyEvent,
    pub memory: Option<Memory>,
    pub pipelines: Vec<Pipeline>,
    pub samplers: Vec<wgpu::Sampler>,
    pub shaders: Vec<Shader>,
    pub sounds: Vec<Sound>,
    pub tasks_active: usize,
    pub text: Option<Arc<Mutex<TextEngine>>>,
    pub textures: Vec<Texture>,
    pub worker: Option<Sender<WorkItem>>,
}

impl System {
    fn new(display: Display) -> System {
        let audio_manager = AudioManager::<kira::manager::backend::DefaultBackend>::new(
            AudioManagerSettings::default(),
        );
        let audio_manager = match audio_manager {
            Ok(audio_manager) => Some(audio_manager),
            Err(err) => {
                dbg!(err);
                None
            }
        };
        System {
            audio_manager: audio_manager,
            bindings: vec![],
            bindings_updated: vec![],
            buffers: vec![],
            display,
            key_event: Default::default(),
            memory: None,
            frame: None,
            pipelines: vec![],
            samplers: vec![],
            shaders: vec![],
            sounds: vec![],
            tasks_active: 0,
            text: None,
            textures: vec![],
            worker: None,
        }
    }
}

#[derive(Debug)]
pub enum WorkItem {
    ImageDecode { handle: usize, bytes: Vec<u8> },
    SoundDecode { handle: usize, bytes: Vec<u8> },
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

fn taca_bindings_apply(mut env: FunctionEnvMut<System>, bindings: u32) {
    let system = env.data_mut();
    bindings_apply(system, bindings);
}

fn taca_bindings_new(mut env: FunctionEnvMut<System>, bindings: u32) -> u32 {
    // TOOD Consider this more.
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let bindings = WasmPtr::<ExternBindingsInfo>::new(bindings)
        .read(&view)
        .unwrap();
    let bindings = BindingsInfo {
        pipeline: bindings.pipeline,
        group_index: bindings.group_index,
        buffers: read_span(&view, bindings.buffers),
        samplers: read_span(&view, bindings.samplers),
        textures: read_span(&view, bindings.textures),
    };
    bindings_new(system, bindings);
    system.bindings.len().try_into().unwrap()
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

fn taca_buffers_apply(mut env: FunctionEnvMut<System>, bindings: u32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let bindings = WasmPtr::<ExternMeshBuffers>::new(bindings)
        .read(&view)
        .unwrap();
    // TODO Reusable buffer to read into!
    let vertex_buffers = read_span(&view, bindings.vertex_buffers);
    let buffers = MeshBuffers {
        vertex_buffers: &vertex_buffers,
        index_buffer: bindings.index_buffer,
    };
    buffers_apply(system, buffers);
}

fn taca_clip(mut env: FunctionEnvMut<System>, x: f32, y: f32, size_x: f32, size_y: f32) {
    let system = env.data_mut();
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        return;
    };
    let wgpu::SurfaceConfiguration { width, height, .. } = gfx.config;
    pass_ensure(system);
    let Some(RenderFrame {
        pass: Some(pass), ..
    }) = &mut system.frame
    else {
        return;
    };
    // gfx.surface.get_current_texture().unwrap().texture.size();
    let x = (x.round() as u32).clamp(0, width);
    let y = (y.round() as u32).clamp(0, height);
    pass.set_scissor_rect(
        x,
        y,
        (size_x.round() as u32).clamp(0, width - x),
        (size_y.round() as u32).clamp(0, height - y),
    );
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

fn taca_image_decode(mut env: FunctionEnvMut<System>, bytes: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let bytes = WasmPtr::<Span>::new(bytes).read(&view).unwrap();
    let bytes = read_span(&view, bytes);
    system.textures.push(Texture { data: None });
    let handle = system.textures.len();
    system.tasks_active += 1;
    system
        .worker
        .as_ref()
        .unwrap()
        .send(WorkItem::ImageDecode { handle, bytes })
        .unwrap();
    handle.try_into().unwrap()
}

fn taca_key_event(mut env: FunctionEnvMut<System>, result: u32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    WasmPtr::<KeyEvent>::new(result)
        .write(&view, system.key_event)
        .unwrap();
}

fn taca_pipeline_apply(mut env: FunctionEnvMut<System>, pipeline: u32) {
    let system = env.data_mut();
    pipeline_apply(system, pipeline);
}

fn taca_pipeline_new(mut env: FunctionEnvMut<System>, info: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let info = WasmPtr::<ExternPipelineInfo>::new(info)
        .read(&view)
        .unwrap();
    // dbg!(info);
    // println!("{info:?}");
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
    // dbg!(&info);
    create_pipeline(system, info);
    system.pipelines.len() as u32
}

fn taca_print(mut env: FunctionEnvMut<System>, text: u32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let text = WasmPtr::<Span>::new(text).read(&view).unwrap();
    let text = read_string(&view, text);
    println!("{text}");
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

fn taca_sound_decode(mut env: FunctionEnvMut<System>, bytes: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let bytes = WasmPtr::<Span>::new(bytes).read(&view).unwrap();
    let bytes = read_span(&view, bytes);
    system.sounds.push(Sound { data: None });
    let handle = system.sounds.len();
    system.tasks_active += 1;
    system
        .worker
        .as_ref()
        .unwrap()
        .send(WorkItem::SoundDecode { handle, bytes })
        .unwrap();
    handle.try_into().unwrap()
}

fn taca_sound_play(mut env: FunctionEnvMut<System>, info: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let info = WasmPtr::<SoundPlayInfoExtern>::new(info)
        .read(&view)
        .unwrap();
    // dbg!(info);
    let Some(audio_manager) = &mut system.audio_manager else {
        // eprintln!("no audio manager");
        return 0;
    };
    let Some(data) = system
        .sounds
        .get(info.sound as usize - 1)
        .and_then(|it| it.data.clone())
    else {
        // eprintln!("no sound");
        return 0;
    };
    let _play = match audio_manager.play(data) {
        Ok(play) => play,
        Err(err) => {
            dbg!(err);
            return 0;
        }
    };
    0
}

fn taca_text_align(mut env: FunctionEnvMut<System>, x: u32, y: u32) {
    let system = env.data_mut();
    let text_engine = system.text.clone().unwrap();
    let mut text_engine = text_engine.lock().unwrap();
    text_engine.align_x = to_text_align_x(x);
    text_engine.align_y = to_text_align_y(y);
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

fn taca_texture_info(mut env: FunctionEnvMut<System>, result: u32, texture: u32) {
    let (system, store) = env.data_and_store_mut();
    let size = system.textures[texture as usize - 1].data.as_ref().map_or(
        wgpu::Extent3d {
            width: 0,
            height: 0,
            depth_or_array_layers: 0,
        },
        |x| x.size,
    );
    let info = TextureInfoExtern {
        size: [size.width as f32, size.height as f32],
    };
    let view = system.memory.as_ref().unwrap().view(&store);
    WasmRef::<TextureInfoExtern>::new(&view, result as u64)
        .write(info)
        .unwrap();
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

fn taca_uniforms_apply(mut env: FunctionEnvMut<System>, bytes: u32) {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let uniforms = WasmPtr::<Span>::new(bytes).read(&view).unwrap();
    let uniforms = read_span::<u8>(&view, uniforms);
    uniforms_apply(system, &uniforms);
}

fn taca_window_state(mut env: FunctionEnvMut<System>, result: u32) {
    let (system, store) = env.data_and_store_mut();
    let MaybeGraphics::Graphics(gfx) = &mut system.display.graphics else {
        return;
    };
    let pointer = system.display.pointer_pos.unwrap_or(Default::default());
    let size = gfx.window.inner_size();
    let state = WindowState {
        pointer: [pointer.x as f32, pointer.y as f32],
        press: system.display.pointer_press,
        size: [size.width as f32, size.height as f32],
    };
    let view = system.memory.as_ref().unwrap().view(&store);
    WasmRef::<WindowState>::new(&view, result as u64)
        .write(state)
        .unwrap();
}
