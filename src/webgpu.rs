pub fn wgpu_adapter_drop(_env: FunctionEnvMut<System>, adapter: u32) {
    // Kill it with the system?
    println!("wgpuAdapterDrop({adapter})");
}

pub fn wgpu_adapter_request_device(
    mut env: FunctionEnvMut<System>,
    adapter: u32,
    descriptor: u32,
    callback: u32,
    userdata: u32,
) {
    println!("wgpuAdapterRequestDevice({adapter}, {descriptor}, {callback}, {userdata})");
    let (system, mut store) = env.data_and_store_mut();
    if system.device.0.is_null() {
        let adapter = system.adapter.0;
        unsafe {
            wgpu_native::device::wgpuAdapterRequestDevice(
                adapter,
                None,
                Some(request_device_callback),
                system as *mut System as *mut std::ffi::c_void,
            );
        }
        extern "C" fn request_device_callback(
            status: native::WGPURequestDeviceStatus,
            device: native::WGPUDevice,
            message: *const std::os::raw::c_char,
            system: *mut std::os::raw::c_void,
        ) {
            if status != native::WGPURequestDeviceStatus_Success {
                panic!("WGPURequestAdapterStatus {status}: {message:?}");
            }
            unsafe {
                let mut system = &mut *(system as *mut System);
                system.device = WGPUDevice(device);
            }
        }
        // TODO Report error if none rather than panicking.
        let functions = system.functions.as_ref().unwrap();
        let value = functions.get(&mut store, callback).unwrap();
        let function = value.unwrap_funcref().as_ref().unwrap();
        function
            .call(
                &mut store,
                &[
                    Value::I32(WGPURequestDeviceStatus_Success.try_into().unwrap()),
                    Value::I32(1),
                    Value::I32(0),
                    // TODO How to put u32 into here? How to just let it wrap?
                    Value::I32(userdata.try_into().unwrap()),
                ],
            )
            .unwrap();
    }
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPURenderPassDescriptor {
    next_in_chain: WasmPtr<WasmWGPUChainedStruct>,
    label: WasmPtr<u8>,
    color_attachment_count: u32,
    color_attachments: WasmPtr<WasmWGPURenderPassColorAttachment>,
    depth_stencil_attachment: u32, // WGPURenderPassDepthStencilAttachment const *
    occlusion_query_set: u32,      // WGPUQuerySet
    timestamp_write_count: u32,
    timestamp_writes: u32, // WGPURenderPassTimestampWrite const *
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPURenderPassColorAttachment {
    view: u32,           // WGPUTextureView,
    resolve_target: u32, // WGPUTextureView
    load_op: native::WGPULoadOp,
    store_op: native::WGPUStoreOp,
    clear_value: WasmWGPUColor,
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPUColor {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

pub fn wgpu_command_encoder_begin_render_pass(
    mut env: FunctionEnvMut<System>,
    encoder: u32,
    descriptor: u32,
) -> u32 {
    println!("wgpuCommandEncoderBeginRenderPass({encoder}, {descriptor})");
    let (mut system, store) = env.data_and_store_mut();
    if system.render_pass.0.is_null() {
        let memory = system.memory.as_ref().unwrap().view(&store);
        let descriptor = WasmRef::<WasmWGPURenderPassDescriptor>::new(&memory, descriptor as u64)
            .read()
            .unwrap();
        let color_attachments: Vec<native::WGPURenderPassColorAttachment> = descriptor
            .color_attachments
            .slice(&memory, descriptor.color_attachment_count)
            .unwrap()
            .iter()
            .map(|attachment| {
                let attachment = attachment.read().unwrap();
                native::WGPURenderPassColorAttachment {
                    view: system.texture_view.0,
                    resolveTarget: std::ptr::null_mut(),
                    loadOp: attachment.load_op,
                    storeOp: attachment.store_op,
                    clearValue: native::WGPUColor {
                        r: attachment.clear_value.r,
                        g: attachment.clear_value.g,
                        b: attachment.clear_value.b,
                        a: attachment.clear_value.a,
                    },
                }
            })
            .collect();
        system.render_pass.0 = unsafe {
            wgpu_native::command::wgpuCommandEncoderBeginRenderPass(
                system.encoder.0,
                Some(&native::WGPURenderPassDescriptor {
                    nextInChain: std::ptr::null(),
                    label: null(),
                    colorAttachmentCount: descriptor.color_attachment_count,
                    colorAttachments: color_attachments.as_ptr(),
                    depthStencilAttachment: std::ptr::null(),
                    occlusionQuerySet: std::ptr::null_mut(),
                    timestampWriteCount: 0,
                    timestampWrites: std::ptr::null(),
                }),
            )
        };
    }
    1
}

pub fn wgpu_command_encoder_finish(
    mut env: FunctionEnvMut<System>,
    _encoder: u32,
    _descriptor: u32,
) -> u32 {
    let system = env.data_mut();
    if !system.encoder.0.is_null() {
        system.command_buffer.0 = unsafe {
            wgpu_native::command::wgpuCommandEncoderFinish(
                system.encoder.0,
                Some(&native::WGPUCommandBufferDescriptor {
                    nextInChain: std::ptr::null(),
                    label: null(),
                }),
            )
        };
        system.encoder.0 = null_mut();
    }
    1
}

pub fn wgpu_create_instance(mut env: FunctionEnvMut<System>, descriptor: u32) -> u32 {
    println!("wgpuCreateInstance({descriptor})");
    let system = env.data_mut();
    if system.instance.0.is_null() {
        system.instance.0 = unsafe {
            wgpu_native::wgpuCreateInstance(Some(&native::WGPUInstanceDescriptor {
                nextInChain: null(),
            }))
        };
    }
    1
}

pub fn wgpu_device_create_command_encoder(
    mut env: FunctionEnvMut<System>,
    device: u32,
    descriptor: u32,
) -> u32 {
    println!("wgpuDeviceCreateCommandEncoder({device}, {descriptor})");
    let system = env.data_mut();
    if system.encoder.0.is_null() {
        system.encoder.0 = unsafe {
            wgpu_native::device::wgpuDeviceCreateCommandEncoder(
                system.device.0,
                Some(&native::WGPUCommandEncoderDescriptor {
                    nextInChain: null(),
                    label: null(),
                }),
            )
        };
    }
    1
}

pub fn wgpu_device_create_pipeline_layout(
    mut env: FunctionEnvMut<System>,
    device: u32,
    descriptor: u32,
) -> u32 {
    println!("wgpuDeviceCreatePipelineLayout({device}, {descriptor})");
    let system = env.data_mut();
    let pipeline_layout = unsafe {
        wgpu_native::device::wgpuDeviceCreatePipelineLayout(
            system.device.0,
            Some(&native::WGPUPipelineLayoutDescriptor {
                nextInChain: null(),
                label: null(),
                bindGroupLayoutCount: 0,
                bindGroupLayouts: null(),
            }),
        )
    };
    system
        .pipeline_layouts
        .push(WGPUPipelineLayout(pipeline_layout));
    system.pipeline_layouts.len().try_into().unwrap()
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPURenderPipelineDescriptor {
    next_in_chain: WasmPtr<WasmWGPUChainedStruct>,
    label: WasmPtr<u8>,
    layout: u32, // WGPUPipelineLayout
    vertex: WasmWGPUVertexState,
    primitive: WasmWGPUPrimitiveState,
    depth_stencil: u32, // WGPUDepthStencilState const *
    multisample: WasmWGPUMultisampleState,
    fragment: WasmPtr<WasmWGPUFragmentState>,
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPUVertexState {
    next_in_chain: WasmPtr<WasmWGPUChainedStruct>,
    module: u32, // WGPUShaderModule
    entry_point: WasmPtr<u8>,
    constant_count: u32,
    constants: u32, // WGPUConstantEntry const *
    buffer_count: u32,
    buffers: u32, // WGPUVertexBufferLayout const *
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPUPrimitiveState {
    next_in_chain: WasmPtr<WasmWGPUChainedStruct>,
    topology: native::WGPUPrimitiveTopology,
    strip_index_format: native::WGPUIndexFormat,
    front_face: native::WGPUFrontFace,
    cull_mode: native::WGPUCullMode,
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPUMultisampleState {
    next_in_chain: WasmPtr<WasmWGPUChainedStruct>,
    count: u32,
    mask: u32,
    alpha_to_coverage_enabled: bool,
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPUFragmentState {
    next_in_chain: WasmPtr<WasmWGPUChainedStruct>,
    module: u32, // WGPUShaderModule
    entry_point: WasmPtr<u8>,
    constant_count: u32,
    constants: u32, // WGPUConstantEntry const *
    target_count: u32,
    targets: WasmPtr<WasmWGPUColorTargetState>,
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPUColorTargetState {
    next_in_chain: WasmPtr<WasmWGPUChainedStruct>,
    format: native::WGPUTextureFormat,
    blend: u32, // WGPUBlendState const *
    write_mask: native::WGPUColorWriteMaskFlags,
}

pub fn wgpu_device_create_render_pipeline(
    mut env: FunctionEnvMut<System>,
    device: u32,
    descriptor: u32,
) -> u32 {
    println!("wgpuDeviceCreateRenderPipeline({device}, {descriptor})");
    let (system, store) = env.data_and_store_mut();
    let memory = system.memory.as_ref().unwrap().view(&store);
    let descriptor = WasmPtr::<WasmWGPURenderPipelineDescriptor>::new(descriptor);
    let descriptor = descriptor.read(&memory).unwrap();
    let vertex_entry_point = read_cstring(descriptor.vertex.entry_point, &memory).unwrap();
    let fragment = descriptor.fragment.read(&memory).unwrap();
    let fragment_entry_point = read_cstring(fragment.entry_point, &memory).unwrap();
    let fragment_targets: Vec<native::WGPUColorTargetState> = fragment
        .targets
        .slice(&memory, fragment.target_count)
        .unwrap()
        .iter()
        .map(|target| {
            let target = target.read().unwrap();
            native::WGPUColorTargetState {
                nextInChain: null(),
                format: target.format,
                blend: null(),
                writeMask: target.write_mask,
            }
        })
        .collect();
    let pipeline = unsafe {
        wgpu_native::device::wgpuDeviceCreateRenderPipeline(
            system.device.0,
            Some(&native::WGPURenderPipelineDescriptor {
                nextInChain: null(),
                label: null(),
                layout: system.pipeline_layouts[descriptor.layout as usize - 1].0,
                vertex: native::WGPUVertexState {
                    nextInChain: null(),
                    module: system.shaders[descriptor.vertex.module as usize - 1].0,
                    entryPoint: vertex_entry_point.as_ptr(),
                    constantCount: 0,
                    constants: null(),
                    bufferCount: 0,
                    buffers: null(),
                },
                primitive: native::WGPUPrimitiveState {
                    nextInChain: null(),
                    topology: descriptor.primitive.topology,
                    stripIndexFormat: descriptor.primitive.strip_index_format,
                    frontFace: descriptor.primitive.front_face,
                    cullMode: descriptor.primitive.cull_mode,
                },
                depthStencil: null(),
                multisample: native::WGPUMultisampleState {
                    nextInChain: null(),
                    count: descriptor.multisample.count,
                    mask: descriptor.multisample.mask,
                    alphaToCoverageEnabled: descriptor.multisample.alpha_to_coverage_enabled,
                },
                fragment: &native::WGPUFragmentState {
                    nextInChain: null(),
                    module: system.shaders[fragment.module as usize - 1].0,
                    entryPoint: fragment_entry_point.as_ptr(),
                    constantCount: 0,
                    constants: null(),
                    targetCount: fragment.target_count,
                    targets: fragment_targets.as_ptr(),
                } as *const native::WGPUFragmentState,
            }),
        )
    };
    system.pipelines.push(WGPURenderPipeline(pipeline));
    system.pipelines.len().try_into().unwrap()
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPUChainedStruct {
    next: WasmPtr<WasmWGPUChainedStruct>,
    s_type: native::WGPUSType,
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WGPUShaderModuleWGSLDescriptor {
    chain: WasmWGPUChainedStruct,
    code: WasmPtr<u8>,
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPUShaderModuleDescriptor {
    next_in_chain: WasmPtr<WasmWGPUChainedStruct>,
    label: u32,
    hint_count: u32,
    hints: u32,
}

pub fn wgpu_device_create_shader_module(
    mut env: FunctionEnvMut<System>,
    device: u32,
    descriptor: u32,
) -> u32 {
    println!("wgpuDeviceCreateShaderModule({device}, {descriptor})");
    let (system, store) = env.data_and_store_mut();
    let memory = system.memory.as_ref().unwrap().view(&store);
    let descriptor = WasmPtr::<WasmWGPUShaderModuleDescriptor>::new(descriptor);
    let descriptor = descriptor.read(&memory).unwrap();
    let next = descriptor.next_in_chain.read(&memory).unwrap();
    let s_type = next.s_type;
    let mut wgsl = native::WGPUShaderModuleWGSLDescriptor {
        chain: native::WGPUChainedStruct {
            next: null(),
            sType: native::WGPUSType_ShaderModuleWGSLDescriptor,
        },
        code: null(),
    };
    let code: Option<CString>;
    let native_next = match s_type {
        native::WGPUSType_ShaderModuleWGSLDescriptor => {
            let wgsl_next =
                WasmPtr::<WGPUShaderModuleWGSLDescriptor>::new(descriptor.next_in_chain.offset());
            let wgsl_next = wgsl_next.read(&memory).unwrap();
            let cstring = read_cstring(wgsl_next.code, &memory).unwrap();
            code = Some(cstring);
            wgsl.code = code.as_ref().unwrap().as_ptr();
            &wgsl as *const native::WGPUShaderModuleWGSLDescriptor
                as *const native::WGPUChainedStruct
        }
        _ => panic!(),
    };
    // println!("{:?}", code.as_ref().unwrap());
    let shader = unsafe {
        wgpu_native::device::wgpuDeviceCreateShaderModule(
            system.device.0,
            Some(&native::WGPUShaderModuleDescriptor {
                nextInChain: native_next,
                label: null(),
                hintCount: 0,
                hints: null(),
            }),
        )
    };
    system.shaders.push(WGPUShaderModule(shader));
    system.shaders.len().try_into().unwrap()
}

pub fn read_cstring(
    pointer: WasmPtr<u8>,
    memory: &MemoryView,
) -> Result<CString, FromVecWithNulError> {
    let mut bytes = pointer.read_until(&memory, |c| *c == 0).unwrap();
    bytes.push(0);
    CString::from_vec_with_nul(bytes)
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmWGPUSwapChainDescriptor {
    next_in_chain: WasmPtr<WasmWGPUChainedStruct>,
    label: WasmPtr<u8>,
    usage: native::WGPUTextureUsageFlags,
    format: native::WGPUTextureFormat,
    width: u32,
    height: u32,
    present_mode: native::WGPUPresentMode,
}

pub fn wgpu_device_create_swap_chain(
    mut env: FunctionEnvMut<System>,
    device: u32,
    surface: u32,
    descriptor: u32,
) -> u32 {
    println!("wgpuDeviceCreateSwapChain({device}, {surface}, {descriptor})");
    let (system, mut store) = env.data_and_store_mut();
    if system.swap_chain.0.is_null() {
        let memory = system.memory.as_ref().unwrap().view(&mut store);
        let descriptor = WasmRef::<WasmWGPUSwapChainDescriptor>::new(&memory, descriptor as u64)
            .read()
            .unwrap();
        system.swap_chain.0 = unsafe {
            wgpu_native::device::wgpuDeviceCreateSwapChain(
                system.device.0,
                system.surface.0,
                Some(&native::WGPUSwapChainDescriptor {
                    nextInChain: null(),
                    label: null(),
                    usage: descriptor.usage,
                    format: descriptor.format,
                    width: descriptor.width,
                    height: descriptor.height,
                    presentMode: descriptor.present_mode,
                }),
            )
        };
    }
    1
}

pub fn wgpu_device_drop(_env: FunctionEnvMut<System>, device: u32) {
    // Kill it with the system?
    println!("wgpuDeviceDrop({device})");
}

pub fn wgpu_device_get_queue(mut env: FunctionEnvMut<System>, adapter: u32) -> u32 {
    println!("wgpuDeviceGetQueue({adapter})");
    let system = env.data_mut();
    if system.queue.0.is_null() {
        system.queue.0 = unsafe { wgpu_native::device::wgpuDeviceGetQueue(system.device.0) };
    }
    1
}

pub fn wgpu_instance_create_surface(
    mut env: FunctionEnvMut<System>,
    instance: u32,
    descriptor: u32,
) -> u32 {
    println!("wgpuInstanceCreateSurface({instance}, {descriptor})");
    let system = env.data_mut();
    if system.surface.0.is_null() {
        system.surface.0 = unsafe {
            wgpu_instance_create_surface_any(system.instance.0, system.window.as_ref().unwrap())
        };
    }
    1
}

unsafe fn wgpu_instance_create_surface_any(
    instance: native::WGPUInstance,
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
                next: null(),
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
            label: null(),
        }),
    )
}

pub fn wgpu_instance_drop(_env: FunctionEnvMut<System>, instance: u32) {
    // Kill it with the system?
    // let system = env.data_mut();
    println!("wgpuInstanceDrop({instance})");
    // if instance == 1 && system.instance_count > 0 {
    //     system.instance_count -= 1;
    //     if system.instance_count == 0 {
    //         let instance = std::mem::take(&mut system.instance);
    //         drop(instance.unwrap());
    //     }
    // }
}

#[allow(non_upper_case_globals)]
const WGPURequestAdapterStatus_Success: i32 = 0;
#[allow(non_upper_case_globals)]
const WGPURequestDeviceStatus_Success: i32 = 0;

pub fn wgpu_instance_request_adapter(
    mut env: FunctionEnvMut<System>,
    instance: u32,
    options: u32,
    callback: u32,
    userdata: u32,
) {
    println!("wgpuInstanceRequestAdapter({instance}, {options}, {callback}, {userdata})");
    let (system, mut store) = env.data_and_store_mut();
    if system.adapter.0.is_null() {
        unsafe {
            wgpu_native::device::wgpuInstanceRequestAdapter(
                system.instance.0,
                Some(&native::WGPURequestAdapterOptions {
                    nextInChain: null(),
                    compatibleSurface: system.surface.0,
                    powerPreference: native::WGPUPowerPreference_Undefined,
                    forceFallbackAdapter: false,
                }),
                Some(request_adapter_callback),
                system as *mut System as *mut std::ffi::c_void,
            )
        };
        extern "C" fn request_adapter_callback(
            status: native::WGPURequestAdapterStatus,
            adapter: native::WGPUAdapter,
            message: *const std::os::raw::c_char,
            system: *mut std::os::raw::c_void,
        ) {
            if status != native::WGPURequestDeviceStatus_Success {
                panic!("WGPURequestAdapterStatus {status}: {message:?}");
            }
            unsafe {
                let mut system = &mut *(system as *mut System);
                system.adapter.0 = adapter;
            }
        }
        // TODO Report error if none rather than panicking.
        let functions = system.functions.as_ref().unwrap();
        let value = functions.get(&mut store, callback).unwrap();
        let function = value.unwrap_funcref().as_ref().unwrap();
        function
            .call(
                &mut store,
                &[
                    Value::I32(WGPURequestAdapterStatus_Success.try_into().unwrap()),
                    Value::I32(1),
                    Value::I32(0),
                    // TODO How to put u32 into here? How to just let it wrap?
                    Value::I32(userdata.try_into().unwrap()),
                ],
            )
            .unwrap();
    }
}

pub fn wgpu_queue_submit(
    mut env: FunctionEnvMut<System>,
    _queue: u32,
    _command_count: u32,
    _commands: u32,
) {
    let system = env.data_mut();
    if !system.queue.0.is_null() && !system.command_buffer.0.is_null() {
        unsafe {
            // TODO Any way to know if the count is right???
            wgpu_native::device::wgpuQueueSubmit(system.queue.0, 1, &system.command_buffer.0);
        }
        system.command_buffer.0 = null_mut();
    }
}

pub fn wgpu_render_pass_encoder_draw(
    env: FunctionEnvMut<System>,
    _render_pass: u32,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) {
    let system = env.data();
    unsafe {
        wgpu_native::command::wgpuRenderPassEncoderDraw(
            system.render_pass.0,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        );
    }
}

pub fn wgpu_render_pass_encoder_end(mut env: FunctionEnvMut<System>, _render_pass: u32) {
    // println!("wgpuSurfaceDrop({surface})");
    let system = env.data_mut();
    if !system.render_pass.0.is_null() {
        unsafe {
            wgpu_native::command::wgpuRenderPassEncoderEnd(system.render_pass.0);
        }
        system.render_pass.0 = null_mut();
    }
}

pub fn wgpu_render_pass_encoder_set_pipeline(
    env: FunctionEnvMut<System>,
    _render_pass: u32,
    pipeline: u32,
) {
    let system = env.data();
    unsafe {
        wgpu_native::command::wgpuRenderPassEncoderSetPipeline(
            system.render_pass.0,
            system.pipelines[pipeline as usize - 1].0,
        );
    }
}

pub fn wgpu_surface_drop(_env: FunctionEnvMut<System>, surface: u32) {
    println!("wgpuSurfaceDrop({surface})");
    // TODO
}

pub fn wgpu_surface_get_preferred_format(
    env: FunctionEnvMut<System>,
    surface: u32,
    adapter: u32,
) -> u32 {
    println!("wgpuSurfaceGetPreferredFormat({surface}, {adapter})");
    let system = env.data();
    unsafe { wgpu_native::wgpuSurfaceGetPreferredFormat(system.surface.0, system.adapter.0) }
        .try_into()
        .unwrap()
}

pub fn wgpu_swap_chain_drop(mut env: FunctionEnvMut<System>, swap_chain: u32) {
    println!("wgpuSwapChainDrop({swap_chain})");
    let system = env.data_mut();
    // For good measure, ensure null view also.
    system.texture_view.0 = null_mut();
    if !system.swap_chain.0.is_null() {
        unsafe {
            wgpu_native::device::wgpuSwapChainDrop(system.swap_chain.0);
        }
        system.swap_chain.0 = null_mut();
    }
}

pub fn wgpu_swap_chain_get_current_texture_view(
    mut env: FunctionEnvMut<System>,
    swap_chain: u32,
) -> u32 {
    println!("wgpuSwapChainGetCurrentTextureView({swap_chain})");
    let system = env.data_mut();
    if system.texture_view.0.is_null() {
        system.texture_view.0 =
            unsafe { wgpu_native::device::wgpuSwapChainGetCurrentTextureView(system.swap_chain.0) };
    }
    1
}

pub fn wgpu_swap_chain_present(mut env: FunctionEnvMut<System>, _swap_chain: u32) {
    // println!("wgpuSwapChainPresent({_swap_chain})");
    let system = env.data_mut();
    if !system.swap_chain.0.is_null() {
        unsafe {
            wgpu_native::device::wgpuSwapChainPresent(system.swap_chain.0);
        }
    }
}

pub fn wgpu_texture_view_drop(mut env: FunctionEnvMut<System>, _texture_view: u32) {
    // println!("wgpuTextureViewDrop({_texture_view})");
    let system = env.data_mut();
    if !system.texture_view.0.is_null() {
        unsafe {
            wgpu_native::device::wgpuTextureViewDrop(system.texture_view.0);
        }
        system.texture_view.0 = null_mut();
    }
}

use crate::system::*;
use std::{
    ffi::{CString, FromVecWithNulError},
    ptr::{null, null_mut},
};
use wasmer::{FunctionEnvMut, MemoryView, Value, ValueType, WasmPtr, WasmRef};
use wgpu_native::native;
use winit::window::Window;
