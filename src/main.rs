use std::fmt;

use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};
// use tacana::State;
use wasmer::{
    imports, Function, FunctionEnv, FunctionEnvMut, Instance, Memory, Module, Store, Table, Value,
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[allow(non_upper_case_globals)]
mod wgpu_native;

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

#[derive(Default)]
struct System {
    adapter: Option<wgpu::Adapter>,
    config: Option<wgpu::SurfaceConfiguration>,
    device: Option<wgpu::Device>,
    functions: Option<Table>,
    // TODO Track process ownership for multi-process mode.
    // TODO Different actual OS processes??? How to share graphics across that???
    // TODO Or just force a singleton instance?
    instance: Option<wgpu::Instance>,
    memory: Option<Memory>,
    queue: Option<wgpu::Queue>,
    surface: Option<wgpu::Surface>,
    // TODO Multiple windows? Would need messaging to create window on main thread and send back maybe.
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
            "wgpuCreateInstance" => Function::new_typed_with_env(&mut store, &env, wgpu_create_instance),
            "wgpuDeviceCreateSwapChain" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_swap_chain),
            "wgpuDeviceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_device_drop),
            "wgpuDeviceGetQueue" => Function::new_typed_with_env(&mut store, &env, wgpu_device_get_queue),
            "wgpuInstanceCreateSurface" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_create_surface),
            "wgpuInstanceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_drop),
            "wgpuInstanceRequestAdapter" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_request_adapter),
            "wgpuSurfaceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_surface_drop),
            "wgpuSurfaceGetPreferredFormat" => Function::new_typed_with_env(&mut store, &env, wgpu_surface_get_preferred_format),
            // wgpuSwapChainDrop
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
    let functions = system.functions.as_ref().unwrap();
    if system.device.is_none() {
        let adapter = system.adapter.as_ref().unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults(),
            },
            // Some(&std::path::Path::new("trace")), // Trace path
            None,
        ))
        .unwrap();
        system.device = Some(device);
        system.queue = Some(queue);
        // TODO Report error if none rather than panicking.
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

fn wgpu_create_instance(mut env: FunctionEnvMut<System>, descriptor: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let memory = system.memory.as_ref().unwrap();
    let size = memory.view(&store).size();
    println!("wgpuCreateInstance({descriptor}) for memory size {size:?}");
    if system.instance.is_none() {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        system.instance = Some(instance);
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
    let usage = match u32::from_le_bytes(buffer) {
        wgpu_native::WGPUTextureUsage_CopySrc => wgpu::TextureUsages::COPY_SRC,
        wgpu_native::WGPUTextureUsage_CopyDst => wgpu::TextureUsages::COPY_DST,
        wgpu_native::WGPUTextureUsage_TextureBinding => wgpu::TextureUsages::TEXTURE_BINDING,
        wgpu_native::WGPUTextureUsage_StorageBinding => wgpu::TextureUsages::STORAGE_BINDING,
        wgpu_native::WGPUTextureUsage_RenderAttachment => wgpu::TextureUsages::RENDER_ATTACHMENT,
        _ => panic!("bad usage"),
    };
    memory.read(address + 12, &mut buffer).unwrap();
    let format = wgpu_native::map_texture_format(u32::from_le_bytes(buffer)).unwrap();
    memory.read(address + 16, &mut buffer).unwrap();
    let width = u32::from_le_bytes(buffer);
    memory.read(address + 20, &mut buffer).unwrap();
    let height = u32::from_le_bytes(buffer);
    memory.read(address + 24, &mut buffer).unwrap();
    let present_mode = match u32::from_le_bytes(buffer) {
        wgpu_native::WGPUPresentMode_Immediate => wgpu::PresentMode::Immediate,
        wgpu_native::WGPUPresentMode_Mailbox => wgpu::PresentMode::Mailbox,
        wgpu_native::WGPUPresentMode_Fifo => wgpu::PresentMode::Fifo,
        _ => panic!("bad present mode"),
    };
    // Configure
    let config = wgpu::SurfaceConfiguration {
        usage,
        format,
        width,
        height,
        present_mode,
        alpha_mode: wgpu::CompositeAlphaMode::Opaque,
        view_formats: vec![],
    };
    let device = system.device.as_ref().unwrap();
    let surface = system.surface.as_ref().unwrap();
    surface.configure(device, &config);
    system.config = Some(config);
    1
}

fn wgpu_device_drop(_env: FunctionEnvMut<System>, device: u32) {
    // Let it die with the system.
    println!("wgpuDeviceDrop({device})");
}

fn wgpu_device_get_queue(_env: FunctionEnvMut<System>, adapter: u32) -> u32 {
    println!("wgpuDeviceGetQueue({adapter})");
    // TODO Assert that we already have a queue defined.
    1
}

fn wgpu_instance_create_surface(
    mut env: FunctionEnvMut<System>,
    instance: u32,
    descriptor: u32,
) -> u32 {
    println!("wgpuInstanceCreateSurface({instance}, {descriptor})");
    let system = env.data_mut();
    // TODO Read the descriptor and use the label or selector as a title???
    if system.surface.is_none() {
        let surface = unsafe {
            system
                .instance
                .as_ref()
                .unwrap()
                .create_surface(system.window.as_ref().unwrap())
        }
        .unwrap();
        system.surface = Some(surface);
    }
    1
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
    let (system, mut store) = env.data_and_store_mut();
    let functions = system.functions.as_ref().unwrap();
    let function_count = functions.size(&store);
    println!("wgpuInstanceRequestAdapter({instance}, {options}, {callback}, {userdata}) for functions {function_count}");
    if system.adapter.is_none() {
        let instance = system.instance.as_ref().unwrap();
        let surface = system.surface.as_ref().unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        });
        // TODO Report error if none rather than panicking.
        system.adapter = Some(pollster::block_on(adapter).unwrap());
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

fn wgpu_surface_drop(_env: FunctionEnvMut<System>, surface: u32) {
    // Let it die with the system.
    println!("wgpuSurfaceDrop({surface})");
}

// WGPU_EXPORT WGPUTextureFormat wgpuSurfaceGetPreferredFormat(WGPUSurface surface, WGPUAdapter adapter);

fn wgpu_surface_get_preferred_format(
    env: FunctionEnvMut<System>,
    surface: u32,
    adapter: u32,
) -> u32 {
    println!("wgpuSurfaceGetPreferredFormat({surface}, {adapter})");
    let system = env.data();
    let capabilities = system
        .surface
        .as_ref()
        .unwrap()
        .get_capabilities(system.adapter.as_ref().unwrap());
    let format = *capabilities.formats.first().unwrap();
    // TODO Store alpha mode for swap chain??? Requery there?
    wgpu_native::to_native_texture_format(format).unwrap()
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
// const tac_WindowEventType_Close: WindowEventType = 1;
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
                    } => *control_flow = ControlFlow::Exit,
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
