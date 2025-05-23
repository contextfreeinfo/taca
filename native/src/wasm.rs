use anyhow::Context;
use exports::taca::core::app;
use std::{fs, path::Path};
use taca::core::{console, key};
use wasmtime::{
    Config, Engine, Result, Store,
    component::{Component, Linker, bindgen},
};

// Generate bindings of the guest and host components.
bindgen!("taca" in "../taca.wit");

struct HostComponent;

impl console::Host for HostComponent {
    fn print(&mut self, text: String) {
        println!("{text}");
    }
}

impl key::Host for HostComponent {
    fn get_event(&mut self) -> key::Event {
        key::Event {
            code: key::Code::None,
            pressed: false,
        }
    }
}

struct MyState {
    host: HostComponent,
}

pub fn run() -> Result<()> {
    let engine = Engine::new(Config::new().wasm_component_model(true))?;
    let mut store = Store::new(
        &engine,
        MyState {
            host: HostComponent {},
        },
    );
    let mut linker = Linker::new(&engine);
    console::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;
    key::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;
    let component_c: &[u8] = include_bytes!("../../examples/c/hi/out/hi-component.wasm");
    let component_c = Component::from_binary(&engine, &component_c)?;
    let taca_c = Taca::instantiate(&mut store, &component_c, &linker)?;
    let app_c = taca_c.taca_core_app();
    let component_rust: &[u8] = include_bytes!(
        "../../examples/rust/hi/target/wasm32-unknown-unknown/release/hi-component.wasm"
    );
    let component_rust = Component::from_binary(&engine, &component_rust)?;
    let taca_rust = Taca::instantiate(&mut store, &component_rust, &linker)?;
    let app_rust = taca_rust.taca_core_app();
    app_c.call_update(&mut store, app::EventKind::Frame)?;
    app_rust.call_update(&mut store, app::EventKind::Frame)?;
    Ok(())
}
