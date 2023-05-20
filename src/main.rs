use std::{
    fmt,
    ptr::{null, null_mut},
};

use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};
use wasmer::{
    imports, Function, FunctionEnv, FunctionEnvMut, Instance, Memory, Module, Store, Table, Value,
};
use wgpu_native::native;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() -> Result<()> {
    pollster::block_on(run())?;
    Ok(())
}

#[derive(Parser)]
#[command(about, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run(RunArgs),
}

#[derive(Args)]
struct RunArgs {
    app: String,
}

async fn run() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Run(args) => {
            run_app(args)?;
        }
    }

    // let state = State::new(window).await;
    // TODO https://users.rust-lang.org/t/can-you-turn-a-callback-into-a-future-into-async-await/49378/16
    // TODO Does this guarantee to wait on anything???
    // State::new(window, |state| {
    //     run_loop(event_loop, state);
    // });
    Ok(())
}

type WGPUAdapter = Pointer<native::WGPUAdapterImpl>;
type WGPUInstance = Pointer<native::WGPUInstanceImpl>;
type WGPURenderPassEncoder = Pointer<native::WGPURenderPassEncoderImpl>;
type WGPUSwapChain = Pointer<native::WGPUSwapChainImpl>;
struct Pointer<T>(*mut T);
unsafe impl<T> Send for Pointer<T> {}
impl<T> Default for Pointer<T> {
    fn default() -> Self {
        Pointer(null_mut())
    }
}

struct WGPUCommandBuffer(native::WGPUCommandBuffer);
unsafe impl Send for WGPUCommandBuffer {}
impl Default for WGPUCommandBuffer {
    fn default() -> Self {
        WGPUCommandBuffer(null_mut())
    }
}

struct WGPUCommandEncoder(native::WGPUCommandEncoder);
unsafe impl Send for WGPUCommandEncoder {}
impl Default for WGPUCommandEncoder {
    fn default() -> Self {
        WGPUCommandEncoder(null_mut())
    }
}

struct WGPUDevice(native::WGPUDevice);
unsafe impl Send for WGPUDevice {}
impl Default for WGPUDevice {
    fn default() -> Self {
        WGPUDevice(null_mut())
    }
}

struct WGPUQueue(native::WGPUQueue);
unsafe impl Send for WGPUQueue {}
impl Default for WGPUQueue {
    fn default() -> Self {
        WGPUQueue(null_mut())
    }
}

struct WGPUSurface(native::WGPUSurface);
unsafe impl Send for WGPUSurface {}
impl Default for WGPUSurface {
    fn default() -> Self {
        WGPUSurface(null_mut())
    }
}

struct WGPUTextureView(native::WGPUTextureView);
unsafe impl Send for WGPUTextureView {}
impl Default for WGPUTextureView {
    fn default() -> Self {
        WGPUTextureView(null_mut())
    }
}

#[derive(Default)]
struct System {
    adapter: WGPUAdapter,
    command_buffer: WGPUCommandBuffer,
    device: WGPUDevice,
    encoder: WGPUCommandEncoder,
    functions: Option<Table>,
    instance: WGPUInstance,
    memory: Option<Memory>,
    queue: WGPUQueue,
    render_pass: WGPURenderPassEncoder,
    surface: WGPUSurface,
    swap_chain: WGPUSwapChain,
    texture_view: WGPUTextureView,
    window: Option<Window>,
    window_listen: Option<wasmer::Function>,
    window_listen_userdata: u32,
}

impl System {
    fn new(window: Window) -> System {
        System {
            window: Some(window),
            ..Default::default()
        }
    }
}

fn run_app(args: &RunArgs) -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Tacaná");

    let mut store = Store::default();
    let module = Module::from_file(&store, args.app.as_str())?;
    let env = FunctionEnv::new(&mut store, System::new(window));
    let import_object = imports! {
        "env" => {
            "tac_windowInnerSize" => Function::new_typed_with_env(&mut store, &env, tac_window_inner_size),
            "tac_windowListen" => Function::new_typed_with_env(&mut store, &env, tac_window_listen),
            "wgpuAdapterDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_adapter_drop),
            "wgpuAdapterRequestDevice" => Function::new_typed_with_env(&mut store, &env, wgpu_adapter_request_device),
            "wgpuCommandEncoderBeginRenderPass" => Function::new_typed_with_env(&mut store, &env, wgpu_command_encoder_begin_render_pass),
            "wgpuCommandEncoderFinish" => Function::new_typed_with_env(&mut store, &env, wgpu_command_encoder_finish),
            "wgpuCreateInstance" => Function::new_typed_with_env(&mut store, &env, wgpu_create_instance),
            "wgpuDeviceCreateCommandEncoder" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_command_encoder),
            "wgpuDeviceCreateSwapChain" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_swap_chain),
            "wgpuDeviceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_device_drop),
            "wgpuDeviceGetQueue" => Function::new_typed_with_env(&mut store, &env, wgpu_device_get_queue),
            "wgpuInstanceCreateSurface" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_create_surface),
            "wgpuInstanceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_drop),
            "wgpuInstanceRequestAdapter" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_request_adapter),
            "wgpuQueueSubmit" => Function::new_typed_with_env(&mut store, &env, wgpu_queue_submit),
            "wgpuRenderPassEncoderEnd" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_end),
            "wgpuSurfaceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_surface_drop),
            "wgpuSurfaceGetPreferredFormat" => Function::new_typed_with_env(&mut store, &env, wgpu_surface_get_preferred_format),
            "wgpuSwapChainDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_swap_chain_drop),
            "wgpuSwapChainGetCurrentTextureView" => Function::new_typed_with_env(&mut store, &env, wgpu_swap_chain_get_current_texture_view),
            "wgpuSwapChainPresent" => Function::new_typed_with_env(&mut store, &env, wgpu_swap_chain_present),
            "wgpuTextureViewDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_texture_view_drop),
        },
        // TODO Combine ours with wasmer_wasix::WasiEnv
        "wasi_snapshot_preview1" => {
            "proc_exit" => Function::new_typed(&mut store, proc_exit),
        },
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let env_mut = env.as_mut(&mut store);
    env_mut.memory = Some(instance.exports.get_memory("memory")?.clone());
    // See for memory access: https://github.com/wasmerio/wasmer/blob/ef5dbd498722d1852ef05774f2c50f886c32ba80/examples/wasi_manual_setup.rs
    // Something like this: wasi_env.data_mut(&mut store).set_memory(memory.clone());
    // TODO Check function type to see if cli args are expected?
    let _start = instance.exports.get_function("_start")?;
    env_mut.functions = Some(
        instance
            .exports
            .get_table("__indirect_function_table")?
            .clone(),
    );

    match _start.call(&mut store, &[]) {
        Ok(_) => {
            println!("Non error termination");
        }
        Err(err) => match err.downcast::<ExitCode>() {
            Ok(exit_code) => {
                println!("Exit code: {exit_code}");
            }
            Err(err) => {
                bail!("Unexpected error {err}");
            }
        },
    }
    // let add_one = instance.exports.get_function("add_one")?;
    // let result = add_one.call(&mut store, &[Value::I32(42)])?;
    // println!("After: {}", result[0].unwrap_i32());

    if env.as_ref(&store).window_listen.is_some() {
        run_loop(event_loop, store, env);
    }

    Ok(())
}

fn tac_window_inner_size(mut env: FunctionEnvMut<System>, result: u32) {
    println!("tac_windowInnerSize({result})");
    let (system, mut store) = env.data_and_store_mut();
    let memory = system.memory.as_ref().unwrap().view(&mut store);
    let size = system.window.as_ref().unwrap().inner_size();
    let result = result as u64;
    memory.write(result, &size.width.to_le_bytes()).unwrap();
    memory
        .write(result + 4, &size.height.to_le_bytes())
        .unwrap();
}

fn tac_window_listen(mut env: FunctionEnvMut<System>, callback: u32, userdata: u32) {
    println!("tac_windowListen({callback}, {userdata})");
    let (mut system, mut store) = env.data_and_store_mut();
    let functions = system.functions.as_ref().unwrap();
    let value = functions.get(&mut store, callback).unwrap();
    system.window_listen = Some(value.unwrap_funcref().as_ref().unwrap().clone());
    system.window_listen_userdata = userdata;
}

fn wgpu_adapter_drop(_env: FunctionEnvMut<System>, adapter: u32) {
    // Let it die with the system.
    println!("wgpuAdapterDrop({adapter})");
}

fn wgpu_adapter_request_device(
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

// wgpuCommandEncoderBeginRenderPass
fn wgpu_command_encoder_begin_render_pass(
    mut env: FunctionEnvMut<System>,
    encoder: u32,
    descriptor: u32,
) -> u32 {
    println!("wgpuCommandEncoderBeginRenderPass({encoder}, {descriptor})");
    let (mut system, store) = env.data_and_store_mut();
    let memory = system.memory.as_ref().unwrap().view(&store);
    let mut buffer = [0u8; 4];
    let address = descriptor as u64;
    memory.read(address + 8, &mut buffer).unwrap();
    let color_attachment_count = u32::from_le_bytes(buffer);
    let clear_value = if color_attachment_count > 0 {
        memory.read(address + 12, &mut buffer).unwrap();
        let attachment = u32::from_le_bytes(buffer) as u64;
        let mut color_buffer = [0u8; 8];
        memory
            .read(attachment + 16 + 0 * 8, &mut color_buffer)
            .unwrap();
        let r = f64::from_le_bytes(color_buffer);
        memory
            .read(attachment + 16 + 1 * 8, &mut color_buffer)
            .unwrap();
        let g = f64::from_le_bytes(color_buffer);
        memory
            .read(attachment + 16 + 2 * 8, &mut color_buffer)
            .unwrap();
        let b = f64::from_le_bytes(color_buffer);
        memory
            .read(attachment + 16 + 3 * 8, &mut color_buffer)
            .unwrap();
        let a = f64::from_le_bytes(color_buffer);
        native::WGPUColor { r, g, b, a }
    } else {
        native::WGPUColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    };
    if system.render_pass.0.is_null() {
        system.render_pass.0 = unsafe {
            wgpu_native::command::wgpuCommandEncoderBeginRenderPass(
                system.encoder.0,
                Some(&native::WGPURenderPassDescriptor {
                    nextInChain: std::ptr::null(),
                    label: null(),
                    colorAttachmentCount: 1,
                    colorAttachments: &native::WGPURenderPassColorAttachment {
                        view: system.texture_view.0,
                        resolveTarget: std::ptr::null_mut(),
                        loadOp: native::WGPULoadOp_Clear,
                        storeOp: native::WGPUStoreOp_Store,
                        clearValue: clear_value,
                    },
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

fn wgpu_command_encoder_finish(
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

fn wgpu_create_instance(mut env: FunctionEnvMut<System>, descriptor: u32) -> u32 {
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

fn wgpu_device_create_command_encoder(
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

// #[derive(Default)]
// #[repr(C)]
// struct WGPUSwapChainDescriptor {
//     nextInChain: u32, // WGPUChainedStruct const*
//     label: u32,       // char const* // nullable
//     usage: u32,       // WGPUTextureUsageFlags
//     format: wgpu_native::WGPUTextureFormat,
//     width: u32,
//     height: u32,
//     presentMode: u32, // WGPUPresentMode
// }

fn wgpu_device_create_swap_chain(
    mut env: FunctionEnvMut<System>,
    device: u32,
    surface: u32,
    descriptor: u32,
) -> u32 {
    println!("wgpuDeviceCreateSwapChain({device}, {surface}, {descriptor})");
    let (system, mut store) = env.data_and_store_mut();
    if system.swap_chain.0.is_null() {
        let memory = system.memory.as_ref().unwrap().view(&mut store);
        let address = descriptor as u64;
        // Read
        let mut buffer = [0u8; 4];
        // typedef struct WGPUSwapChainDescriptorExtras {
        //     WGPUChainedStruct chain;
        //     WGPUCompositeAlphaMode alphaMode;
        //     size_t viewFormatCount;
        //     WGPUTextureFormat const * viewFormats;
        // } WGPUSwapChainDescriptorExtras;
        memory.read(address + 8, &mut buffer).unwrap();
        let usage = u32::from_le_bytes(buffer);
        memory.read(address + 12, &mut buffer).unwrap();
        let format = u32::from_le_bytes(buffer);
        memory.read(address + 16, &mut buffer).unwrap();
        let width = u32::from_le_bytes(buffer);
        memory.read(address + 20, &mut buffer).unwrap();
        let height = u32::from_le_bytes(buffer);
        memory.read(address + 24, &mut buffer).unwrap();
        let present_mode = u32::from_le_bytes(buffer);
        // Configure
        system.swap_chain.0 = unsafe {
            wgpu_native::device::wgpuDeviceCreateSwapChain(
                system.device.0,
                system.surface.0,
                Some(&native::WGPUSwapChainDescriptor {
                    nextInChain: null(),
                    label: null(),
                    usage,
                    format,
                    width,
                    height,
                    presentMode: present_mode,
                }),
            )
        };
    }
    1
}

fn wgpu_device_drop(_env: FunctionEnvMut<System>, device: u32) {
    // Let it die with the system.
    println!("wgpuDeviceDrop({device})");
}

fn wgpu_device_get_queue(mut env: FunctionEnvMut<System>, adapter: u32) -> u32 {
    println!("wgpuDeviceGetQueue({adapter})");
    let system = env.data_mut();
    if system.queue.0.is_null() {
        system.queue.0 = unsafe { wgpu_native::device::wgpuDeviceGetQueue(system.device.0) };
    }
    1
}

fn wgpu_instance_create_surface(
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

fn wgpu_instance_drop(_env: FunctionEnvMut<System>, instance: u32) {
    // Let it die with the system.
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

fn wgpu_instance_request_adapter(
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

fn wgpu_queue_submit(
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

fn wgpu_render_pass_encoder_end(mut env: FunctionEnvMut<System>, _render_pass: u32) {
    // println!("wgpuSurfaceDrop({surface})");
    let system = env.data_mut();
    if !system.render_pass.0.is_null() {
        unsafe {
            wgpu_native::command::wgpuRenderPassEncoderEnd(system.render_pass.0);
        }
        system.render_pass.0 = null_mut();
    }
}

fn wgpu_surface_drop(_env: FunctionEnvMut<System>, surface: u32) {
    println!("wgpuSurfaceDrop({surface})");
    // TODO
}

fn wgpu_surface_get_preferred_format(
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

fn wgpu_swap_chain_drop(mut env: FunctionEnvMut<System>, swap_chain: u32) {
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

fn wgpu_swap_chain_get_current_texture_view(
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

fn wgpu_swap_chain_present(mut env: FunctionEnvMut<System>, _swap_chain: u32) {
    // println!("wgpuSwapChainPresent({_swap_chain})");
    let system = env.data_mut();
    if !system.swap_chain.0.is_null() {
        unsafe {
            wgpu_native::device::wgpuSwapChainPresent(system.swap_chain.0);
        }
    }
}

fn wgpu_texture_view_drop(mut env: FunctionEnvMut<System>, _texture_view: u32) {
    // println!("wgpuTextureViewDrop({_texture_view})");
    let system = env.data_mut();
    if !system.texture_view.0.is_null() {
        unsafe {
            wgpu_native::device::wgpuTextureViewDrop(system.texture_view.0);
        }
        system.texture_view.0 = null_mut();
    }
}

#[derive(Debug, Clone, Copy)]
struct ExitCode(u32);

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ExitCode {}

fn proc_exit(code: u32) -> std::result::Result<(), ExitCode> {
    println!("proc_exit({code})");
    Err(ExitCode(code))
}

pub type WindowEventType = u32;
#[allow(non_upper_case_globals)]
const tac_WindowEventType_Close: WindowEventType = 1;
#[allow(non_upper_case_globals)]
const tac_WindowEventType_Redraw: WindowEventType = 2;
#[allow(non_upper_case_globals)]
const tac_WindowEventType_Resize: WindowEventType = 3;

fn run_loop(event_loop: EventLoop<()>, mut store: Store, env: FunctionEnv<System>) {
    // let (system, mut store) = env.data_and_store_mut();
    // let window = system.window.as_ref().unwrap();
    event_loop.run(move |event, _, control_flow| {
        let system = env.as_ref(&store);
        let window = system.window.as_ref().unwrap();
        let window_listen = system.window_listen.as_ref().unwrap().clone();
        let window_listen_userdata = system.window_listen_userdata;
        fn send_event(
            store: &mut Store,
            function: &Function,
            event_type: WindowEventType,
            userdata: u32,
        ) {
            function
                .call(
                    store,
                    &[
                        // TODO How to put u32 into here? How to just let it wrap?
                        Value::I32(event_type.try_into().unwrap()),
                        Value::I32(userdata.try_into().unwrap()),
                    ],
                )
                .unwrap();
        }
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                // if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        send_event(
                            &mut store,
                            &window_listen,
                            tac_WindowEventType_Close,
                            window_listen_userdata,
                        );
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(_physical_size) => {
                        send_event(
                            &mut store,
                            &window_listen,
                            tac_WindowEventType_Resize,
                            window_listen_userdata,
                        );
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        send_event(
                            &mut store,
                            &window_listen,
                            tac_WindowEventType_Resize,
                            window_listen_userdata,
                        );
                    }
                    _ => {}
                }
                // }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                send_event(
                    &mut store,
                    &window_listen,
                    tac_WindowEventType_Redraw,
                    window_listen_userdata,
                );
                // state.update();
                // state.render();
                // match state.render() {
                //     Ok(_) => {}
                //     // Reconfigure the surface if it's lost or outdated
                //     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                //     // The system is out of memory, we should probably quit
                //     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                //     Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                // }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                // state.window().request_redraw();
            }
            _ => {}
        }
    });
}
