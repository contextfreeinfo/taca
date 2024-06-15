pub fn wasmish() -> anyhow::Result<()> {
    unsafe {
        browser_load_app();
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

    #[link_name = "loadApp"]
    pub fn browser_load_app();
}

#[no_mangle]
pub extern "C" fn hi() {
    crate::say_hi();
}

#[no_mangle]
pub extern "C" fn taca_crate_version() -> i32 {
    0
}

#[no_mangle]
fn taca_RenderingContext_applyBindings(context: u32, bindings: u32) {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_applyBindings {context} {bindings}"
    ));
}

#[no_mangle]
fn taca_RenderingContext_applyPipeline(context: u32, pipeline: u32) {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_applyPipeline {context} {pipeline}"
    ));
}

#[no_mangle]
fn taca_RenderingContext_beginPass(context: u32) {
    crate::wasmic::print(&format!("taca_RenderingContext_beginPass {context}"));
}

#[no_mangle]
fn taca_RenderingContext_commitFrame(context: u32) {
    crate::wasmic::print(&format!("taca_RenderingContext_commitFrame {context}"));
}

#[no_mangle]
fn taca_RenderingContext_draw(
    context: u32,
    base_element: u32,
    num_elements: u32,
    num_instances: u32,
) {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_draw {context} {base_element} {num_elements} {num_instances}"
    ));
}

#[no_mangle]
fn taca_RenderingContext_endPass(context: u32) {
    crate::wasmic::print(&format!("taca_RenderingContext_endPass {context}"));
}

#[no_mangle]
fn taca_RenderingContext_newBuffer(context: u32, typ: u32, usage: u32, info: u32) -> u32 {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_newBuffer {context} {typ} {usage} {info}"
    ));
    0
}

#[no_mangle]
fn taca_RenderingContext_newPipeline(context: u32, info: u32) -> u32 {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_newPipeline {context} {info}"
    ));
    0
}

#[no_mangle]
fn taca_RenderingContext_newShader(context: u32, bytes: u32) -> u32 {
    crate::wasmic::print(&format!(
        "taca_RenderingContext_newShader {context} {bytes}"
    ));
    0
}

#[no_mangle]
fn taca_Window_get() -> u32 {
    crate::wasmic::print("taca_Window_get");
    1
}

#[no_mangle]
fn taca_Window_newRenderingContext(window: u32) -> u32 {
    crate::wasmic::print(&format!("taca_Window_newRenderingContext {window}"));
    1
}
