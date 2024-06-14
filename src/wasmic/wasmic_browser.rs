pub fn wasmish(wasm: &[u8]) -> anyhow::Result<()> {
    unsafe {
        browser_run_wasm(wasm.as_ptr(), wasm.len());
    }
    Ok(())
}

pub fn print(text: &str) {
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
    crate::say_hi();
}

#[no_mangle]
pub extern "C" fn taca_crate_version() -> i32 {
    0
}
