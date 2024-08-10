use std::{
    future::Future,
    ptr::null_mut,
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};
use wasmer::ValueType;
use wgpu::{
    Adapter, Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration, TextureFormat,
};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::Key,
    window::{Fullscreen, Window, WindowId},
};

use crate::app::AppPtr;

pub struct Display {
    pub app: AppPtr,
    pub graphics: MaybeGraphics,
    pub pointer_pos: Option<PhysicalPosition<f64>>,
    time_end: Instant,
    time_mean: f64,
    time_report: Instant,
}

const REPORT_DELAY: Duration = Duration::from_secs(10);

impl Display {
    pub fn new(event_loop: &EventLoop<Graphics>) -> Self {
        Self {
            app: AppPtr(null_mut()),
            graphics: MaybeGraphics::Builder(GraphicsBuilder::new(event_loop.create_proxy())),
            pointer_pos: None,
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
        gfx.surface_config.width = size.width;
        gfx.surface_config.height = size.height;
        gfx.surface.configure(&gfx.device, &gfx.surface_config);
    }

    pub fn run(&mut self, event_loop: EventLoop<Graphics>) {
        event_loop.run_app(self).unwrap();
    }
}

impl<'a> ApplicationHandler<Graphics> for Display {
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
                    KeyEvent {
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
                Key::Named(key) => match key {
                    winit::keyboard::NamedKey::F11 => {
                        if state.is_pressed() && !repeat {
                            let fullscreen = match gfx.window.fullscreen() {
                                Some(_) => None,
                                None => Some(Fullscreen::Borderless(None)),
                            };
                            gfx.window.set_fullscreen(fullscreen);
                        }
                    }
                    _ => {}
                },
                Key::Character(_) => {}
                Key::Unidentified(_) => {}
                Key::Dead(_) => {}
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.pointer_pos = Some(position);
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

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, graphics: Graphics) {
        graphics.window.as_ref().set_title("Taca");
        self.graphics = MaybeGraphics::Graphics(graphics);
        let MaybeGraphics::Graphics(gfx) = &self.graphics else {
            panic!()
        };
        unsafe { &mut *self.app.0 }.start(gfx);
    }
}

#[allow(dead_code)]
pub struct Graphics {
    pub window: Arc<Window>,
    instance: Instance,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    render_pipeline: Option<RenderPipeline>,
}

pub struct GraphicsBuilder {
    event_loop_proxy: Option<EventLoopProxy<Graphics>>,
}

impl GraphicsBuilder {
    fn new(event_loop_proxy: EventLoopProxy<Graphics>) -> Self {
        Self {
            event_loop_proxy: Some(event_loop_proxy),
        }
    }

    fn build_and_send(&mut self, event_loop: &ActiveEventLoop) {
        let Some(event_loop_proxy) = self.event_loop_proxy.take() else {
            // event_loop_proxy is already spent - we already constructed Graphics
            return;
        };
        let gfx = pollster::block_on(create_graphics(event_loop));
        assert!(event_loop_proxy.send_event(gfx).is_ok());
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
    pub size: [f32; 2],
}

fn create_graphics(event_loop: &ActiveEventLoop) -> impl Future<Output = Graphics> + 'static {
    let window_attrs = Window::default_attributes();
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
        let mut surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface_config.view_formats =
            vec![TextureFormat::Bgra8UnormSrgb, TextureFormat::Bgra8Unorm];
        // dbg!(&surface_config);
        surface.configure(&device, &surface_config);

        Graphics {
            window,
            instance,
            surface,
            surface_config,
            adapter,
            device,
            queue,
            render_pipeline: None,
        }
    }
}
