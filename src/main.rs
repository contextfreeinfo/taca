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
    let window = WindowBuilder::new()
        .with_maximized(true)
        .build(&event_loop)
        .unwrap();
    window.set_title("Taca");

    let mut store = Store::default();
    let module = Module::from_file(&store, args.app.as_str())?;
    let env = FunctionEnv::new(&mut store, System::new(window));
    let import_object = imports! {
        "env" => {
            "taca_gpu_bufferWrite" => Function::new_typed_with_env(&mut store, &env, taca_gpu_buffer_write),
            "taca_gpu_draw" => Function::new_typed_with_env(&mut store, &env, taca_gpu_draw),
            "taca_gpu_indexBufferCreate" => Function::new_typed_with_env(&mut store, &env, taca_gpu_index_buffer_create),
            "taca_gpu_present" => Function::new_typed_with_env(&mut store, &env, taca_gpu_present),
            "taca_gpu_shaderCreate" => Function::new_typed_with_env(&mut store, &env, taca_gpu_shader_create),
            "taca_gpu_uniformBufferCreate" => Function::new_typed_with_env(&mut store, &env, taca_gpu_uniform_buffer_create),
            "taca_gpu_textureCreate" => Function::new_typed_with_env(&mut store, &env, taca_gpu_texture_create),
            "taca_gpu_vertexBufferCreate" => Function::new_typed_with_env(&mut store, &env, taca_gpu_vertex_buffer_create),
            "taca_keyEvent" => Function::new_typed_with_env(&mut store, &env, taca_key_event),
            "taca_windowInnerSize" => Function::new_typed_with_env(&mut store, &env, taca_window_inner_size),
            "taca_windowListen" => Function::new_typed_with_env(&mut store, &env, taca_window_listen),
            "taca_windowSetTitle" => Function::new_typed_with_env(&mut store, &env, taca_window_set_title),
            "wgpuAdapterDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_adapter_drop),
            "wgpuAdapterGetLimits" => Function::new_typed_with_env(&mut store, &env, wgpu_adapter_get_limits),
            "wgpuAdapterRequestDevice" => Function::new_typed_with_env(&mut store, &env, wgpu_adapter_request_device),
            "wgpuCommandEncoderBeginRenderPass" => Function::new_typed_with_env(&mut store, &env, wgpu_command_encoder_begin_render_pass),
            "wgpuCommandEncoderFinish" => Function::new_typed_with_env(&mut store, &env, wgpu_command_encoder_finish),
            "wgpuCreateInstance" => Function::new_typed_with_env(&mut store, &env, wgpu_create_instance),
            "wgpuDeviceCreateBindGroup" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_bind_group),
            "wgpuDeviceCreateBindGroupLayout" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_bind_group_layout),
            "wgpuDeviceCreateBuffer" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_buffer),
            "wgpuDeviceCreateCommandEncoder" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_command_encoder),
            "wgpuDeviceCreatePipelineLayout" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_pipeline_layout),
            "wgpuDeviceCreateRenderPipeline" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_render_pipeline),
            "wgpuDeviceCreateShaderModule" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_shader_module),
            "wgpuDeviceCreateSwapChain" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_swap_chain),
            "wgpuDeviceCreateTexture" => Function::new_typed_with_env(&mut store, &env, wgpu_device_create_texture),
            "wgpuDeviceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_device_drop),
            "wgpuDeviceGetQueue" => Function::new_typed_with_env(&mut store, &env, wgpu_device_get_queue),
            "wgpuDeviceSetUncapturedErrorCallback" => Function::new_typed_with_env(&mut store, &env, wgpu_device_set_uncaptured_error_callback),
            "wgpuInstanceCreateSurface" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_create_surface),
            "wgpuInstanceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_drop),
            "wgpuInstanceRequestAdapter" => Function::new_typed_with_env(&mut store, &env, wgpu_instance_request_adapter),
            "wgpuQueueSubmit" => Function::new_typed_with_env(&mut store, &env, wgpu_queue_submit),
            "wgpuQueueWriteBuffer" => Function::new_typed_with_env(&mut store, &env, wgpu_queue_write_buffer),
            "wgpuQueueWriteTexture" => Function::new_typed_with_env(&mut store, &env, wgpu_queue_write_texture),
            "wgpuRenderPassEncoderDraw" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_draw),
            "wgpuRenderPassEncoderDrawIndexed" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_draw_indexed),
            "wgpuRenderPassEncoderEnd" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_end),
            "wgpuRenderPassEncoderSetBindGroup" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_set_bind_group),
            "wgpuRenderPassEncoderSetPipeline" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_set_pipeline),
            "wgpuRenderPassEncoderSetIndexBuffer" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_set_index_buffer),
            "wgpuRenderPassEncoderSetVertexBuffer" => Function::new_typed_with_env(&mut store, &env, wgpu_render_pass_encoder_set_vertex_buffer),
            "wgpuSurfaceDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_surface_drop),
            "wgpuSurfaceGetPreferredFormat" => Function::new_typed_with_env(&mut store, &env, wgpu_surface_get_preferred_format),
            "wgpuSwapChainDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_swap_chain_drop),
            "wgpuSwapChainGetCurrentTextureView" => Function::new_typed_with_env(&mut store, &env, wgpu_swap_chain_get_current_texture_view),
            "wgpuSwapChainPresent" => Function::new_typed_with_env(&mut store, &env, wgpu_swap_chain_present),
            "wgpuTextureCreateView" => Function::new_typed_with_env(&mut store, &env, wgpu_texture_create_view),
            "wgpuTextureDestroy" => Function::new_typed_with_env(&mut store, &env, wgpu_texture_destroy),
            "wgpuTextureViewDrop" => Function::new_typed_with_env(&mut store, &env, wgpu_texture_view_drop),
        },
        // TODO Combine ours with wasmer_wasix::WasiEnv???
        "wasi_snapshot_preview1" => {
            "args_get" => Function::new_typed_with_env(&mut store, &env, wasi_args_get),
            "args_sizes_get" => Function::new_typed_with_env(&mut store, &env, wasi_args_sizes_get),
            "fd_close" => Function::new_typed_with_env(&mut store, &env, wasi_fd_close),
            "fd_fdstat_get" => Function::new_typed_with_env(&mut store, &env, wasi_fd_fdstat_get),
            "fd_seek" => Function::new_typed_with_env(&mut store, &env, wasi_fd_seek),
            "fd_write" => Function::new_typed_with_env(&mut store, &env, wasi_fd_write),
            "proc_exit" => Function::new_typed(&mut store, wasi_proc_exit),
        },
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let env_mut = env.as_mut(&mut store);
    env_mut.memory = Some(instance.exports.get_memory("memory")?.clone());
    // See for memory access: https://github.com/wasmerio/wasmer/blob/ef5dbd498722d1852ef05774f2c50f886c32ba80/examples/wasi_manual_setup.rs
    // Something like this: wasi_env.data_mut(&mut store).set_memory(memory.clone());
    // TODO Check function type to see if cli args are expected?
    let _start = instance.exports.get_function("_start")?;
    env_mut.functions = instance
        .exports
        .get_table("__indirect_function_table")
        .ok()
        .map(|it| it.clone());
    env_mut.named_window_listen = instance
        .exports
        .get_function("windowListen")
        .ok()
        .map(|it| it.clone());

    match _start.call(&mut store, &[]) {
        Ok(_) => {
            // println!("Non error termination");
        }
        Err(err) => match err.downcast::<ExitCode>() {
            Ok(exit_code) => {
                if exit_code.0 != 0 {
                    bail!("Exit code: {exit_code}");
                }
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

fn wasi_args_get(_env: FunctionEnvMut<System>, _argv: u32, _argv_buf: u32) -> u32 {
    0
}

fn wasi_args_sizes_get(mut env: FunctionEnvMut<System>, argv_size: u32, argv_buf_size: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    view.write(argv_size as u64, &[0]).unwrap();
    view.write(argv_buf_size as u64, &[0]).unwrap();
    0
}

fn wasi_fd_close(_env: FunctionEnvMut<System>, _fd: u32) -> u32 {
    0
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasiFdStat {
    file_type: u8,
    _fill1: u8,
    flags: u16,
    _fill2: u32,
    rights_base: u64,
    rights_inheriting: u64,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum WasiFileType {
    Unknown = 0,
    BlockDevice,
    CharacterDevice,
    Directory,
    RegularFile,
    SocketDgram,
    SocketStream,
    SymbolicLink,
}

fn wasi_fd_fdstat_get(mut env: FunctionEnvMut<System>, _fd: u32, fdstat: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let fdstat = WasmRef::<WasiFdStat>::new(&view, fdstat as u64);
    fdstat
        .write(WasiFdStat {
            file_type: WasiFileType::CharacterDevice as u8,
            _fill1: 0,
            flags: 0,
            _fill2: 0,
            rights_base: 0,
            rights_inheriting: 0,
        })
        .unwrap();
    0
}

fn wasi_fd_seek(
    mut env: FunctionEnvMut<System>,
    _fd: u32,
    _filedelta: u64,
    _whence: u32,
    new_offset: u32,
) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    view.write(new_offset as u64, &[0]).unwrap();
    1
}

fn wasi_fd_write(
    mut env: FunctionEnvMut<System>,
    fd: u32,
    iovec: u32,
    len: u32,
    nwritten: u32,
) -> u32 {
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

fn wasi_proc_exit(code: u32) -> std::result::Result<(), ExitCode> {
    println!("proc_exit({code})");
    Err(ExitCode(code))
}

mod gpu;
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

use crate::gpu::*;
use crate::system::*;
use crate::webgpu::*;
use crate::window::*;
