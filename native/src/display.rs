use image::{DynamicImage, ImageResult};
use kira::sound::static_sound::StaticSoundData;
use std::{
    future::Future,
    ptr::null_mut,
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};
use wasmer::{Value, ValueType};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration, TextureFormat};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Size},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{self, NamedKey},
    window::{Fullscreen, Window, WindowId},
};

use crate::{
    app::AppPtr,
    gpu::TextureData,
    key::{Key, KeyEvent},
};

pub struct Display {
    pub app: AppPtr,
    pub graphics: MaybeGraphics,
    pub pointer_pos: Option<PhysicalPosition<f64>>,
    pub pointer_press: u32,
    time_end: Instant,
    time_mean: f64,
    time_report: Instant,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum EventKind {
    Frame = 0,
    Key = 1,
    TasksDone = 2,
}

const REPORT_DELAY: Duration = Duration::from_secs(10);

#[derive(Clone, Copy, Debug, Default)]
pub struct DisplayOptions {
    pub size: Option<(f64, f64)>,
}

impl Display {
    pub fn new(event_loop: &EventLoop<UserEvent>, options: DisplayOptions) -> Self {
        Self {
            app: AppPtr(null_mut()),
            graphics: MaybeGraphics::Builder(GraphicsBuilder::new(
                event_loop.create_proxy(),
                options,
            )),
            pointer_pos: None,
            pointer_press: 0,
            time_end: Instant::now(),
            time_mean: 0.0,
            time_report: Instant::now() + REPORT_DELAY,
        }
    }

    fn draw(&mut self) {
        let MaybeGraphics::Graphics(gfx) = &mut self.graphics else {
            // draw call rejected because graphics doesn't exist yet
            return;
        };
        // TODO Event.
        unsafe { &mut *self.app.0 }.listen();
        gfx.window.request_redraw();
        let elapsed = self.time_end.elapsed();
        let target_elapsed = Duration::from_secs_f64(1.0 / 60.0);
        if elapsed < target_elapsed {
            sleep(target_elapsed - elapsed);
        }
        let elapsed = self.time_end.elapsed();
        self.time_end = Instant::now();
        let weight = 0.95;
        self.time_mean = weight * self.time_mean + (1.0 - weight) * elapsed.as_secs_f64();
        if self.time_end > self.time_report {
            self.time_report = self.time_end + REPORT_DELAY;
            // println!("fps: {}", 1.0 / self.time_mean);
        }
    }

    fn resized(&mut self, size: PhysicalSize<u32>) {
        let MaybeGraphics::Graphics(gfx) = &mut self.graphics else {
            return;
        };
        gfx.config.width = size.width;
        gfx.config.height = size.height;
        gfx.surface.configure(&gfx.device, &gfx.config);
        gfx.depth_texture = create_depth_texture(&gfx.device, &gfx.config);
    }

    pub fn run(&mut self, event_loop: EventLoop<UserEvent>) {
        event_loop.run_app(self).unwrap();
    }
}

impl<'a> ApplicationHandler<UserEvent> for Display {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let MaybeGraphics::Graphics(gfx) = &mut self.graphics else {
            // draw call rejected because graphics doesn't exist yet
            return;
        };
        match event {
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: _,
                        logical_key,
                        text: _,
                        location: _,
                        state,
                        repeat,
                        ..
                    },
                ..
            } => match logical_key {
                keyboard::Key::Named(key) => match key {
                    NamedKey::F11 => {
                        if state.is_pressed() && !repeat {
                            let fullscreen = match gfx.window.fullscreen() {
                                Some(_) => None,
                                None => Some(Fullscreen::Borderless(None)),
                            };
                            gfx.window.set_fullscreen(fullscreen);
                        }
                    }
                    _ if !repeat => {
                        let app = unsafe { &mut *self.app.0 };
                        let key: Key = key.into();
                        let key = key as i32;
                        let pressed = state.is_pressed();
                        let system = app.env.as_mut(&mut app.store);
                        system.key_event = KeyEvent { key, pressed };
                        if let Some(update) = &app.update {
                            update
                                .call(&mut app.store, &[Value::I32(EventKind::Key as i32)])
                                .unwrap();
                        }
                    }
                    _ => {}
                },
                keyboard::Key::Character(_) => {}
                keyboard::Key::Unidentified(_) => {}
                keyboard::Key::Dead(_) => {}
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.pointer_pos = Some(position);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                // TODO Consider device_id?
                let bit = match button {
                    winit::event::MouseButton::Left => 1,
                    winit::event::MouseButton::Right => 2,
                    winit::event::MouseButton::Middle => 4,
                    _ => {
                        dbg!(button);
                        0
                    }
                };
                match state {
                    winit::event::ElementState::Pressed => self.pointer_press |= bit,
                    winit::event::ElementState::Released => self.pointer_press &= !bit,
                }
            }
            WindowEvent::Resized(size) => self.resized(size),
            WindowEvent::RedrawRequested => self.draw(),
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let MaybeGraphics::Builder(builder) = &mut self.graphics {
            builder.build_and_send(event_loop);
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::Graphics(graphics) => {
                graphics.window.as_ref().set_title("Taca");
                self.graphics = MaybeGraphics::Graphics(graphics);
                let MaybeGraphics::Graphics(gfx) = &self.graphics else {
                    panic!()
                };
                unsafe { &mut *self.app.0 }.start(gfx);
            }
            event => unsafe { &mut *self.app.0 }.handle(event),
        }
    }
}

#[derive(Debug)]
pub enum UserEvent {
    Graphics(Graphics),
    ImageDecoded {
        handle: usize,
        image: ImageResult<DynamicImage>,
    },
    SoundDecoded {
        handle: usize,
        sound: Result<StaticSoundData, kira::sound::FromFileError>,
    },
}

#[derive(Debug)]
pub struct Graphics {
    pub window: Arc<Window>,
    pub config: SurfaceConfiguration,
    pub depth_texture: TextureData,
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

pub struct GraphicsBuilder {
    event_loop_proxy: Option<EventLoopProxy<UserEvent>>,
    options: DisplayOptions,
}

impl GraphicsBuilder {
    fn new(event_loop_proxy: EventLoopProxy<UserEvent>, options: DisplayOptions) -> Self {
        Self {
            event_loop_proxy: Some(event_loop_proxy),
            options,
        }
    }

    fn build_and_send(&mut self, event_loop: &ActiveEventLoop) {
        let Some(event_loop_proxy) = self.event_loop_proxy.take() else {
            // event_loop_proxy is already spent - we already constructed Graphics
            return;
        };
        let gfx = pollster::block_on(create_graphics(event_loop, self.options));
        assert!(event_loop_proxy
            .send_event(UserEvent::Graphics(gfx))
            .is_ok());
    }
}

pub enum MaybeGraphics {
    Builder(GraphicsBuilder),
    Graphics(Graphics),
}

#[derive(Clone, Copy, Debug, Default, ValueType)]
#[repr(C)]
pub struct WindowState {
    pub pointer: [f32; 2],
    pub press: u32,
    pub size: [f32; 2],
}

pub fn create_depth_texture(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) -> TextureData {
    let size = wgpu::Extent3d {
        width: config.width.max(1),
        height: config.height.max(1),
        depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[wgpu::TextureFormat::Depth32Float],
    };
    let texture = device.create_texture(&desc);
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    TextureData {
        size,
        texture,
        view,
    }
}

fn create_graphics(
    event_loop: &ActiveEventLoop,
    options: DisplayOptions,
) -> impl Future<Output = Graphics> + 'static {
    let mut window_attrs = Window::default_attributes();
    if let Some((width, height)) = options.size {
        window_attrs = window_attrs.with_inner_size(Size::Logical(LogicalSize { width, height }));
    }
    let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(window.clone()).unwrap();

    async move {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    memory_hints: wgpu::MemoryHints::Performance,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        config.view_formats = vec![TextureFormat::Bgra8UnormSrgb, TextureFormat::Bgra8Unorm];
        // dbg!(&config);
        surface.configure(&device, &config);
        let depth_texture = create_depth_texture(&device, &config);

        Graphics {
            window,
            config,
            depth_texture,
            instance,
            surface,
            adapter,
            device,
            queue,
        }
    }
}
