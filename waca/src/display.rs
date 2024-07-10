use std::{borrow::Cow, future::Future, ptr::null_mut, sync::Arc};
use wasmer::ValueType;
use wgpu::{Adapter, Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration};
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
}

impl Display {
    pub fn new(event_loop: &EventLoop<Graphics>) -> Self {
        Self {
            app: AppPtr(null_mut()),
            graphics: MaybeGraphics::Builder(GraphicsBuilder::new(event_loop.create_proxy())),
            pointer_pos: None,
        }
    }

    fn draw(&mut self) {
        let MaybeGraphics::Graphics(gfx) = &mut self.graphics else {
            // draw call rejected because graphics doesn't exist yet
            return;
        };
        // TODO Event.
        unsafe { &mut *self.app.0 }.listen();

        if gfx.render_pipeline.is_none() {
            gfx.build_render_pipeline();
        }

        let frame = gfx.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = gfx.device.create_command_encoder(&Default::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            rpass.set_pipeline(gfx.render_pipeline.as_ref().unwrap());
            rpass.draw(0..3, 0..1);
        }

        let command_buffer = encoder.finish();
        gfx.queue.submit([command_buffer]);
        frame.present();
        gfx.window.request_redraw();
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
        unsafe { &mut *self.app.0 }.start();
    }
}

#[allow(dead_code)]
pub struct Graphics {
    pub window: Arc<Window>,
    instance: Instance,
    pub surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    adapter: Adapter,
    pub device: Device,
    queue: Queue,
    render_pipeline: Option<RenderPipeline>,
}

impl Graphics {
    fn build_render_pipeline(&mut self) {
        let Graphics {
            adapter,
            device,
            surface,
            ..
        } = self;
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed("
                    @vertex
                    fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                        let x = f32(i32(in_vertex_index) - 1);
                        let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
                        return vec4<f32>(x, y, 0.0, 1.0);
                    }
    
                    @fragment
                    fn fs_main() -> @location(0) vec4<f32> {
                        return vec4<f32>(1.0, 1.0, 0.0, 1.0);
                    }
            "))
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        });
        self.render_pipeline = Some(render_pipeline);
    }
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
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
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
