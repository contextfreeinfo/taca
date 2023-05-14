use std::fmt;

use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};
use tacana::State;
use wasmer::{imports, Function, Instance, Module, Store, Value};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
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

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Tacaná");

    // let state = State::new(window).await;
    // TODO https://users.rust-lang.org/t/can-you-turn-a-callback-into-a-future-into-async-await/49378/16
    // TODO Does this guarantee to wait on anything???
    // State::new(window, |state| {
    //     run_loop(event_loop, state);
    // });
    Ok(())
}

fn run_app(args: &RunArgs) -> Result<()> {
    println!("Run: {}", args.app);

    // let module_wat = r#"
    //     (import "host" "hello" (func $host_hello (param i32)))

    //     (func (export "add_one") (param $n i32) (result i32)
    //         (local.set $n (i32.add (local.get $n) (i32.const 1)))
    //         (call $host_hello (local.get $n))
    //         local.get $n
    //     )
    // "#;

    let mut store = Store::default();
    let module = Module::from_file(&store, args.app.as_str())?;
    let import_object = imports! {
        "env" => {
            "wgpuCreateInstance" => Function::new_typed(&mut store, |descriptor: u32| -> u32 {
                println!("wgpuCreateInstance({descriptor})");
                0
            }),
            "wgpuInstanceCreateSurface" => Function::new_typed(&mut store, |instance: u32, descriptor: u32| -> u32 {
                println!("wgpuInstanceCreateSurface({instance}, {descriptor})");
                0
            }),
            "wgpuInstanceRequestAdapter" => Function::new_typed(&mut store, |instance: u32, options: u32, callback: u32, userdata: u32| {
                println!("wgpuInstanceRequestAdapter({instance}, {options}, {callback}, {userdata})");
            }),
            "wgpuInstanceDrop" => Function::new_typed(&mut store, |instance: u32| {
                println!("wgpuInstanceDrop({instance})");
            }),
            "wgpuSurfaceDrop" => Function::new_typed(&mut store, |surface: u32| {
                println!("wgpuSurfaceDrop({surface})");
            }),
        },
        // TODO Combine ours with wasmer_wasix::WasiEnv
        "wasi_snapshot_preview1" => {
            "proc_exit" => Function::new_typed(&mut store, proc_exit),
        },
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let _memory = instance.exports.get_memory("memory")?;
    // See for memory access: https://github.com/wasmerio/wasmer/blob/ef5dbd498722d1852ef05774f2c50f886c32ba80/examples/wasi_manual_setup.rs
    // Something like this? -> wasi_env.data_mut(&mut store).set_memory(memory.clone());
    let _start = instance.exports.get_function("_start")?;
    // let _functions = instance.exports.get_function("__indirect_function_table")?;

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

fn _run_loop(event_loop: EventLoop<()>, mut state: State) {
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    // UPDATED!
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
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                state.render();
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
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}
