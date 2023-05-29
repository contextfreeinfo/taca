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
            "wgpuAdapterGetLimits" => Function::new_typed_with_env(&mut store, &env, wgpu_adapter_get_limits),
            "wgpuAdapterRequestDevice" => Function::new_typed_with_env(&mut store, &env, wgpu_adapter_request_device),
            "wgpuCommandEncoderBeginRenderPass" => Function::new_typed_with_env(&mut store, &env, wgpu_command_encoder_begin_render_pass),
            "wgpuCommandEncoderFinish" => Function::new_typed_with_env(&mut store, &env, wgpu_command_encoder_finish),
            "wgpuCreateInstance" => Function::new_typed_with_env(&mut store, &env, wgpu_create_instance),
            "wgpuDeviceCreateBuffer" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_buffer),
            "wgpuDeviceCreateCommandEncoder" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_command_encoder),
            "wgpuDeviceCreatePipelineLayout" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_pipeline_layout),
            "wgpuDeviceCreateRenderPipeline" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_render_pipeline),
            "wgpuDeviceCreateShaderModule" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_shader_module),
            "wgpuDeviceCreateSwapChain" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_swap_chain),
            "wgpuDeviceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_device_drop),
            "wgpuDeviceGetQueue" => Function::new_typed_with_env(&mut store, &env, wgpu_device_get_queue),
            "wgpuDeviceSetUncapturedErrorCallback" => Function::new_typed_with_env(&mut store, &env, wgpu_device_set_uncaptured_error_callback),
            "wgpuInstanceCreateSurface" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_create_surface),
            "wgpuInstanceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_drop),
            "wgpuInstanceRequestAdapter" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_request_adapter),
            "wgpuQueueSubmit" => Function::new_typed_with_env(&mut store, &env, wgpu_queue_submit),
            "wgpuQueueWriteBuffer" => Function::new_typed_with_env(&mut store, &env, wgpu_queue_write_buffer),
            "wgpuRenderPassEncoderDraw" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_draw),
            "wgpuRenderPassEncoderEnd" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_end),
            "wgpuRenderPassEncoderSetPipeline" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_set_pipeline),
            "wgpuRenderPassEncoderSetVertexBuffer" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_set_vertex_buffer),
            "wgpuSurfaceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_surface_drop),
            "wgpuSurfaceGetPreferredFormat" => Function::new_typed_with_env(&mut store, &env, wgpu_surface_get_preferred_format),
            "wgpuSwapChainDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_swap_chain_drop),
            "wgpuSwapChainGetCurrentTextureView" => Function::new_typed_with_env(&mut store, &env, wgpu_swap_chain_get_current_texture_view),
            "wgpuSwapChainPresent" => Function::new_typed_with_env(&mut store, &env, wgpu_swap_chain_present),
            "wgpuTextureViewDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_texture_view_drop),
        },
        // TODO Combine ours with wasmer_wasix::WasiEnv???
        "wasi_snapshot_preview1" => {
            "fd_write" => Function::new_typed_with_env(&mut store, &env, fd_write),
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

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmIOVec {
    buf: WasmPtr<u8>,
    size: u32,
}

fn fd_write(mut env: FunctionEnvMut<System>, fd: u32, iovec: u32, len: u32, nwritten: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let mut count = 0u32;
    for io in WasmPtr::<WasmIOVec>::new(iovec)
        .slice(&view, len)
        .unwrap()
        .iter()
    {
        let io = io.read().unwrap();
        // TODO Support arbitrary bytes to output streams? Depends on config???
        let text = io.buf.read_utf8_string(&view, io.size).unwrap();
        match fd {
            1 => print!("{}", text),
            _ => eprint!("{}", text),
        }
        count += io.size;
    }
    WasmRef::<u32>::new(&view, nwritten as u64)
        .write(count)
        .unwrap();
    0
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

mod system;
mod webgpu;
mod window;

use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};
use std::fmt;
use wasmer::{
    imports, Function, FunctionEnv, FunctionEnvMut, Instance, Module, Store, ValueType, WasmPtr,
    WasmRef,
};
use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::system::*;
use crate::webgpu::*;
use crate::window::*;
