use std::{fmt, num::Wrapping};

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

struct System {
    adapter: Option<wgpu::Adapter>,
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
    window: Window,
}

impl System {
    fn new(window: Window) -> System {
        System {
            adapter: None,
            device: None,
            functions: None,
            instance: None,
            memory: None,
            queue: None,
            surface: None,
            window,
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
            "wgpuAdapterRequestDevice" => Function::new_typed_with_env(&mut store, &env, wgpu_adapter_request_device),
            "wgpuCreateInstance" => Function::new_typed_with_env(&mut store, &env, wgpu_create_instance),
            "wgpuDeviceGetQueue" => Function::new_typed_with_env(&mut store, &env, wgpu_device_get_queue),
            "wgpuInstanceCreateSurface" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_create_surface),
            "wgpuInstanceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_drop),
            "wgpuInstanceRequestAdapter" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_request_adapter),
            "wgpuSurfaceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_surface_drop),
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

    Ok(())
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
                .create_surface(&system.window)
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

// fn _run_loop(event_loop: EventLoop<()>, mut state: State) {
//     event_loop.run(move |event, _, control_flow| {
//         match event {
//             Event::WindowEvent {
//                 ref event,
//                 window_id,
//             } if window_id == state.window().id() => {
//                 if !state.input(event) {
//                     // UPDATED!
//                     match event {
//                         WindowEvent::CloseRequested
//                         | WindowEvent::KeyboardInput {
//                             input:
//                                 KeyboardInput {
//                                     state: ElementState::Pressed,
//                                     virtual_keycode: Some(VirtualKeyCode::Escape),
//                                     ..
//                                 },
//                             ..
//                         } => *control_flow = ControlFlow::Exit,
//                         WindowEvent::Resized(physical_size) => {
//                             state.resize(*physical_size);
//                         }
//                         WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
//                             // new_inner_size is &&mut so w have to dereference it twice
//                             state.resize(**new_inner_size);
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//             Event::RedrawRequested(window_id) if window_id == state.window().id() => {
//                 state.update();
//                 state.render();
//                 // match state.render() {
//                 //     Ok(_) => {}
//                 //     // Reconfigure the surface if it's lost or outdated
//                 //     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
//                 //     // The system is out of memory, we should probably quit
//                 //     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

//                 //     Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
//                 // }
//             }
//             Event::RedrawEventsCleared => {
//                 // RedrawRequested will only trigger once, unless we manually
//                 // request it.
//                 state.window().request_redraw();
//             }
//             _ => {}
//         }
//     });
// }
