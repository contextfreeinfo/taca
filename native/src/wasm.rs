use crate::part::Part;
use exports::taca::core::app;
use taca::core::{archive, console, storage, task};
use wasmtime::{
    Config,
    Engine,
    Result,
    Store,
    component::{Component, Linker, Resource, bindgen},
    // TODO Feature gate wasi access?
    // component::ResourceTable,
};
// use wasmtime_wasi::p2::bindings::sync::Command;
// use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView};

// Generate bindings of the guest and host components.
bindgen!("taca" in "../taca.wit");

struct HostComponent;

impl archive::Host for HostComponent {
    fn get_bytes(&mut self, name: String) -> Option<Vec<u8>> {
        let _ = name;
        None
    }

    fn get_text(&mut self, name: String) -> Option<String> {
        let _ = name;
        None
    }
}

impl console::Host for HostComponent {
    fn print(&mut self, text: String) {
        println!("{text}");
    }
}

impl storage::Host for HostComponent {
    fn get(
        &mut self,
        store: Option<Resource<storage::Store>>,
        key: String,
    ) -> Resource<task::Task> {
        let _ = store;
        let _ = key;
        panic!()
    }

    fn get_bytes(&mut self, task: Resource<task::Task>) -> Option<Vec<u8>> {
        let _ = task;
        None
    }

    fn get_text(&mut self, task: Resource<task::Task>) -> Option<String> {
        let _ = task;
        None
    }

    fn put_bytes(
        &mut self,
        store: Option<Resource<storage::Store>>,
        key: String,
        value: Vec<u8>,
    ) -> Resource<task::Task> {
        let _ = store;
        let _ = key;
        let _ = value;
        panic!()
    }

    fn put_text(
        &mut self,
        store: Option<Resource<storage::Store>>,
        key: String,
        value: String,
    ) -> Resource<task::Task> {
        let _ = store;
        let _ = key;
        let _ = value;
        panic!()
    }
}

impl storage::HostStore for HostComponent {
    fn drop(&mut self, rep: Resource<storage::Store>) -> Result<()> {
        let _ = rep;
        Ok(())
    }
}

impl task::Host for HostComponent {}

impl task::HostTask for HostComponent {
    fn finished(&mut self, self_: Resource<task::Task>) -> bool {
        let _ = self_;
        false
    }

    fn drop(&mut self, rep: Resource<task::Task>) -> Result<()> {
        let _ = rep;
        Ok(())
    }
}

struct MyState {
    host: HostComponent,
    // These two are required basically as a standard way to enable the impl of IoView and
    // WasiView.
    // impl of WasiView is required by [`wasmtime_wasi::p2::add_to_linker_sync`]
    // pub resource_table: ResourceTable,
    // pub wasi_ctx: WasiCtx,
    // You can add other custom host states if needed
}

// impl IoView for MyState {
//     fn table(&mut self) -> &mut ResourceTable {
//         &mut self.resource_table
//     }
// }

// impl WasiView for MyState {
//     fn ctx(&mut self) -> &mut WasiCtx {
//         &mut self.wasi_ctx
//     }
// }

pub fn run(part: Part) -> Result<()> {
    // See: https://github.com/bytecodealliance/wasmtime/blob/main/examples/wasip2/main.rs
    // See: https://github.com/bytecodealliance/wasmtime/blob/main/examples/wasip1/main.rs
    let engine = Engine::new(Config::new().wasm_component_model(true))?;
    // let wasi_ctx = WasiCtxBuilder::new()
    //     .allow_tcp(false)
    //     .allow_udp(false)
    //     .build();
    let mut store = Store::new(
        &engine,
        MyState {
            host: HostComponent {},
            // resource_table: ResourceTable::new(),
            // wasi_ctx,
        },
    );
    let mut linker = Linker::new(&engine);
    // wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    console::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;
    // key::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;
    storage::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;
    task::add_to_linker(&mut linker, |state: &mut MyState| &mut state.host)?;
    // Use provided file.
    let component_bytes = part.read_bytes("app.wasm")?;
    // let component_c: &[u8] = include_bytes!("../../examples/c/hi/out/hi-component.wasm");
    let component_c = Component::from_binary(&engine, &component_bytes)?;
    let taca_c = Taca::instantiate(&mut store, &component_c, &linker)?;
    let app_c = taca_c.taca_core_app();
    // let component_rust: &[u8] = include_bytes!(
    //     "../../examples/rust/hi/target/wasm32-unknown-unknown/release/hi-component.wasm"
    // );
    // let component_rust = Component::from_binary(&engine, component_rust)?;
    // let taca_rust = Taca::instantiate(&mut store, &component_rust, &linker)?;
    // let app_rust = taca_rust.taca_core_app();
    app_c.call_update(&mut store, app::Event::Frame(0.0))?;
    // app_rust.call_update(&mut store, app::EventKind::Frame)?;
    Ok(())
}
