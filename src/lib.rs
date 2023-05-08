use wgpu_native::native;
use winit::{event::WindowEvent, window::Window};

pub struct State {
    surface: native::WGPUSurface,
    device: native::WGPUDevice,
    queue: native::WGPUQueue,
    // config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Window,
}

impl State {
    pub fn new<Callback>(window: Window, callback: Callback)
    where
        Callback: FnOnce(Self),
    {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        // let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        //     backends: wgpu::Backends::all(),
        //     dx12_shader_compiler: Default::default(),
        // });
        let instance = unsafe {
            wgpu_native::wgpuCreateInstance(Some(&native::WGPUInstanceDescriptor {
                nextInChain: std::ptr::null(),
            }))
        };

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        // let surface = unsafe { instance.create_surface(&window) }.unwrap();
        // TODO Need our own "create surface"???
        let surface = unsafe { wgpu_instance_create_surface(instance, &window) };

        // let adapter = instance
        //     .request_adapter(&wgpu::RequestAdapterOptions {
        //         power_preference: wgpu::PowerPreference::default(),
        //         compatible_surface: Some(&surface),
        //         force_fallback_adapter: false,
        //     })
        //     .await
        //     .unwrap();
        let request_adapter_callback_data = Box::into_raw(Box::new(RequestAdapterCallbackData {
            callback,
            instance,
            surface,
            window,
        })) as *mut std::ffi::c_void;
        unsafe {
            wgpu_native::device::wgpuInstanceRequestAdapter(
                instance,
                Some(&native::WGPURequestAdapterOptions {
                    nextInChain: std::ptr::null(),
                    compatibleSurface: surface,
                    powerPreference: native::WGPUPowerPreference_Undefined,
                    forceFallbackAdapter: false,
                }),
                Some(request_adapter_callback::<Callback>),
                request_adapter_callback_data,
            )
        };
        // Supposedly in practice, the adapter is ready immediately after the
        // call returns, but officially, it's not.
        // https://eliemichel.github.io/LearnWebGPU/getting-started/the-adapter.html

        // let (device, queue) = adapter
        //     .request_device(
        //         &wgpu::DeviceDescriptor {
        //             label: None,
        //             features: wgpu::Features::empty(),
        //             limits: wgpu::Limits::downlevel_webgl2_defaults(),
        //         },
        //         // Some(&std::path::Path::new("trace")), // Trace path
        //         None,
        //     )
        //     .await
        //     .unwrap();
        struct RequestAdapterCallbackData<Callback> {
            callback: Callback,
            instance: native::WGPUInstance,
            surface: native::WGPUSurface,
            window: Window,
        }

        extern "C" fn request_adapter_callback<Callback>(
            status: native::WGPURequestAdapterStatus,
            adapter: native::WGPUAdapter,
            message: *const std::os::raw::c_char,
            request_adapter_callback_data: *mut std::os::raw::c_void,
        ) where
            Callback: FnOnce(State),
        {
            if status != native::WGPURequestDeviceStatus_Success {
                panic!("WGPURequestAdapterStatus {status}: {message:?}");
            }
            let request_adapter_callback_data =
                request_adapter_callback_data as *mut RequestAdapterCallbackData<Callback>;
            unsafe {
                let RequestAdapterCallbackData {
                    callback,
                    instance,
                    surface,
                    window,
                } = *Box::from_raw(request_adapter_callback_data);
                // I think we can drop the instance while still using things from it???
                // TODO Impl drop for State to clean up others later?
                wgpu_native::wgpuInstanceDrop(instance);
                wgpu_native::device::wgpuAdapterRequestDevice(
                    adapter,
                    None,
                    Some(request_device_callback::<Callback>),
                    Box::into_raw(Box::new(RequestDeviceCallbackData {
                        adapter,
                        callback,
                        surface,
                        window,
                    })) as *mut std::ffi::c_void,
                );
            }
        }

        struct RequestDeviceCallbackData<Callback> {
            adapter: native::WGPUAdapter,
            callback: Callback,
            surface: native::WGPUSurface,
            window: Window,
        }

        extern "C" fn request_device_callback<Callback>(
            status: native::WGPURequestDeviceStatus,
            device: native::WGPUDevice,
            message: *const std::os::raw::c_char,
            request_device_callback_data: *mut std::os::raw::c_void,
        ) where
            Callback: FnOnce(State),
        {
            if status != native::WGPURequestDeviceStatus_Success {
                panic!("WGPURequestAdapterStatus {status}: {message:?}");
            }
            let request_device_callback_data =
                request_device_callback_data as *mut RequestDeviceCallbackData<Callback>;
            unsafe {
                let RequestDeviceCallbackData {
                    callback,
                    surface,
                    window,
                    ..
                } = *Box::from_raw(request_device_callback_data);
                let queue = wgpu_native::device::wgpuDeviceGetQueue(device);
                let state = State {
                    surface,
                    device,
                    queue,
                    // config,
                    size: window.inner_size(),
                    window: window,
                };
                callback(state);
            }
        }

        // let surface_caps = surface.get_capabilities(&adapter);
        // // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // // one will result all the colors comming out darker. If you want to support non
        // // Srgb surfaces, you'll need to account for that when drawing to the frame.
        // let surface_format = surface_caps
        //     .formats
        //     .iter()
        //     .copied()
        //     .filter(|f| f.is_srgb())
        //     .next()
        //     .unwrap_or(surface_caps.formats[0]);
        // let config = wgpu::SurfaceConfiguration {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format: surface_format,
        //     width: size.width,
        //     height: size.height,
        //     present_mode: surface_caps.present_modes[0],
        //     alpha_mode: surface_caps.alpha_modes[0],
        //     view_formats: vec![],
        // };
        // surface.configure(&device, &config);
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            // self.config.width = new_size.width;
            // self.config.height = new_size.height;
            // self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    // pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    //     let output = self.surface.get_current_texture()?;
    //     let view = output
    //         .texture
    //         .create_view(&wgpu::TextureViewDescriptor::default());

    //     let mut encoder = self
    //         .device
    //         .create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //             label: Some("Render Encoder"),
    //         });

    //     {
    //         // unsafe {
    //         //     let mut native_encoder = native::WGPUCommandEncoderImpl {
    //         //         context: Arc::clone(&self.device)
    //         //     };
    //         //     wgpuCommandEncoderBeginRenderPass(
    //         //         &mut native_encoder,
    //         //         Some(
    //         //             native::WGPURenderPassDescriptor(
    //         //                 //let descriptor_chain =
    //         //             )
    //         //         ),
    //         //     );
    //         // }
    //         let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //             label: Some("Render Pass"),
    //             color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //                 view: &view,
    //                 resolve_target: None,
    //                 ops: wgpu::Operations {
    //                     load: wgpu::LoadOp::Clear(wgpu::Color {
    //                         r: 0.1,
    //                         g: 0.2,
    //                         b: 0.3,
    //                         a: 1.0,
    //                     }),
    //                     store: true,
    //                 },
    //             })],
    //             depth_stencil_attachment: None,
    //         });
    //     }

    //     self.queue.submit(iter::once(encoder.finish()));
    //     output.present();

    //     Ok(())
    // }
}

unsafe fn wgpu_instance_create_surface(
    instance: *mut native::WGPUInstanceImpl,
    window: &Window,
) -> native::WGPUSurface {
    // First extract raw handles.
    let raw_display = raw_window_handle::HasRawDisplayHandle::raw_display_handle(window);
    let raw_window = raw_window_handle::HasRawWindowHandle::raw_window_handle(window);
    // Then put struct data on stack so it lives.
    let xlib = if let raw_window_handle::RawWindowHandle::Xlib(xlib_window) = raw_window {
        let raw_window_handle::RawDisplayHandle::Xlib(xlib_display) = raw_display else {
            unreachable!()
        };
        Some(native::WGPUSurfaceDescriptorFromXlibWindow {
            chain: native::WGPUChainedStruct {
                next: std::ptr::null(),
                sType: native::WGPUSType_SurfaceDescriptorFromXlibWindow,
            },
            display: xlib_display.display,
            window: u32::try_from(xlib_window.window).unwrap(),
        })
    } else {
        None
    };
    // TODO Other backends above and below.
    // Metal: https://github.com/gfx-rs/wgpu/blob/f173575427b028dde71bdb76dce10d27060b03ba/wgpu-hal/src/metal/mod.rs#L83
    // Then cast as a chain pointer.
    let descriptor_chain = if let Some(xlib) = xlib.as_ref() {
        xlib as *const native::WGPUSurfaceDescriptorFromXlibWindow
            as *const native::WGPUChainedStruct
    } else {
        panic!("unsupported backend")
    };
    wgpu_native::wgpuInstanceCreateSurface(
        instance,
        Some(&native::WGPUSurfaceDescriptor {
            nextInChain: descriptor_chain,
            label: std::ptr::null(),
        }),
    )
}
