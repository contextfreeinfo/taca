use std::fmt;

use wasmer::{FunctionEnvMut, ValueType, WasmPtr, WasmRef};

use crate::app::System;

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmIOVec {
    buf: WasmPtr<u8>,
    size: u32,
}

pub fn args_get(_env: FunctionEnvMut<System>, _argv: u32, _argv_buf: u32) -> u32 {
    0
}

pub fn args_sizes_get(mut env: FunctionEnvMut<System>, argv_size: u32, argv_buf_size: u32) -> u32 {
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    view.write(argv_size as u64, &[0]).unwrap();
    view.write(argv_buf_size as u64, &[0]).unwrap();
    0
}

pub fn fd_close(_env: FunctionEnvMut<System>, _fd: u32) -> u32 {
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

pub fn fd_fdstat_get(mut env: FunctionEnvMut<System>, _fd: u32, fdstat: u32) -> u32 {
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

pub fn fd_seek(
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

pub fn fd_write(
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
pub struct ExitCode(u32);

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ExitCode {}

pub fn proc_exit(code: u32) -> std::result::Result<(), ExitCode> {
    println!("proc_exit({code})");
    Err(ExitCode(code))
}

pub fn random_get(_buf: u32, _buf_len: u32) -> u32 {
    // Claim failure for now.
    // https://github.com/ziglang/zig/blob/4d81e8ee915c3e012131cf90ed87cc8c6a01a934/stage1/wasi.c#L42
    29
}
