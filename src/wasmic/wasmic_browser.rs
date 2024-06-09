pub fn wasmish(wasm: &[u8]) -> anyhow::Result<()> {
    unsafe {
        browser_run_wasm(wasm.as_ptr(), wasm.len());
    }
    Ok(())
}

fn print(text: &[u8]) {
    unsafe {
        browser_print(text.as_ptr(), text.len());
    }
}

extern "C" {
    #[link_name = "print"]
    pub fn browser_print(text: *const u8, len: usize);

    #[link_name = "runWasm"]
    pub fn browser_run_wasm(wasm: *const u8, len: usize);
}

#[no_mangle]
pub extern "C" fn hi() {
    print("Hi!".as_bytes());
}

#[no_mangle]
pub extern "C" fn taca_crate_version() -> i32 {
    0
}
