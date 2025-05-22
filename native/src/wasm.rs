use anyhow::Context;
use std::{fs, path::Path};

use wasmtime::{
    Config, Engine, Result, Store,
    component::{Component, Linker, bindgen},
};

// Generate bindings of the guest and host components.
bindgen!("taca-api" in "../taca.wit");

struct HostComponent;

// Implementation of the host interface defined in the wit file.
// impl host::Host for HostComponent {
//     fn multiply(&mut self, a: f32, b: f32) -> f32 {
//         a * b
//     }
// }

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
    // let mut linker = Linker::new(&engine);
    // host::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;
    Ok(())
}
