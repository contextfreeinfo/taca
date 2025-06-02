use crate::archive::Archive;
use anyhow::Result;
use exports::taca::core::app;
use std::sync::{Arc, Mutex};
use taca::core::{archive, console, storage, task};
use wasmtime::{
    Config, Engine,
    component::{Component, Linker, Resource, ResourceTable, bindgen},
};
// use wasmtime_wasi::p2::bindings::sync::Command;
// use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView};

// Generate bindings of the guest and host components.
bindgen!({
    path: "../taca.wit",
    world: "taca",
    trappable_imports: true,
    with: {
        "taca:core/storage/bytes": crate::wasm::Task,
        "taca:core/storage/store": crate::wasm::Store,
    },
});

struct TacaCoreHost {
    archive: Archive,
    table: ResourceTable,
}

pub enum Task {
    StorageGetBytes(StorageGet<Vec<u8>>),
    StorageGetText(StorageGet<String>),
    StoragePut(StoragePut),
}

#[derive(Default)]
pub struct StorageGet<T> {
    value: Arc<Mutex<Option<T>>>,
    // task: Resource<task::Task>,
}

#[derive(Default)]
pub struct StoragePut {
    value: Arc<Mutex<bool>>,
    // task: Resource<task::Task>,
}

impl archive::Host for TacaCoreHost {
    fn get_bytes(&mut self, name: String) -> Result<Option<Vec<u8>>> {
        // TODO Log if error rather than missing?
        Ok(self.archive.read_bytes(&name).ok())
    }

    fn get_text(&mut self, name: String) -> Result<Option<String>> {
        // TODO Log if error rather than missing?
        Ok(self.archive.read_string(&name).ok())
    }
}

impl console::Host for TacaCoreHost {
    fn print(&mut self, text: String) -> Result<()> {
        println!("{text}");
        Ok(())
    }
}

pub struct Store {
    // TODO Wrap each around an Arc/Mutex or Rc/RefCell.
}

impl storage::Host for TacaCoreHost {
    fn access(&mut self) -> Result<Resource<storage::Store>> {
        Ok(self.table.push(Store {})?)
    }
}

impl storage::HostStore for TacaCoreHost {
    fn drop(&mut self, rep: Resource<storage::Store>) -> Result<()> {
        let _ = rep;
        Ok(())
    }

    fn get_bytes(
        &mut self,
        self_: Resource<storage::Store>,
        name: String,
    ) -> Result<Resource<storage::Bytes>> {
        let _ = self_;
        let _ = name;
        Ok(self.table.push(Task::StorageGetBytes(Default::default()))?)
    }

    fn get_text(
        &mut self,
        self_: Resource<storage::Store>,
        name: String,
    ) -> Result<Resource<storage::Text>> {
        let _ = self_;
        let _ = name;
        panic!()
    }

    fn set_bytes(
        &mut self,
        self_: Resource<storage::Store>,
        name: String,
        value: Vec<u8>,
    ) -> Result<Resource<task::Task>> {
        let _ = self_;
        let _ = name;
        let _ = value;
        panic!()
    }

    fn set_text(
        &mut self,
        self_: Resource<storage::Store>,
        name: String,
        value: String,
    ) -> Result<Resource<task::Task>> {
        let _ = self_;
        let _ = name;
        let _ = value;
        panic!()
    }
}

impl storage::HostBytes for TacaCoreHost {
    fn task(&mut self, self_: Resource<storage::Bytes>) -> Result<Option<Resource<task::Task>>> {
        let _ = self_;
        Ok(None)
    }

    fn take(&mut self, self_: Resource<storage::Bytes>) -> Result<Option<Vec<u8>>> {
        let _ = self_;
        Ok(None)
    }

    fn drop(&mut self, rep: Resource<storage::Bytes>) -> Result<()> {
        let _ = rep;
        Ok(())
    }
}

impl storage::HostText for TacaCoreHost {
    fn task(&mut self, self_: Resource<storage::Text>) -> Result<Option<Resource<task::Task>>> {
        let _ = self_;
        Ok(None)
    }

    fn take(&mut self, self_: Resource<storage::Text>) -> Result<Option<String>> {
        let _ = self_;
        Ok(None)
    }

    fn drop(&mut self, rep: Resource<storage::Text>) -> Result<()> {
        let _ = rep;
        Ok(())
    }
}

impl task::Host for TacaCoreHost {}

impl task::HostTask for TacaCoreHost {
    fn done(&mut self, self_: Resource<task::Task>) -> Result<bool> {
        let _ = self_;
        Ok(false)
    }

    fn failed(&mut self, self_: Resource<task::Task>) -> Result<bool> {
        let _ = self_;
        Ok(false)
    }

    fn drop(&mut self, rep: Resource<task::Task>) -> Result<()> {
        let _ = rep;
        Ok(())
    }
}

struct TacaState {
    taca_core: TacaCoreHost,
    // These two are required basically as a standard way to enable the impl of IoView and
    // WasiView.
    // impl of WasiView is required by [`wasmtime_wasi::p2::add_to_linker_sync`]
    // pub resource_table: ResourceTable,
    // pub wasi_ctx: WasiCtx,
    // You can add other custom host states if needed
}

// impl IoView for TacaState {
//     fn table(&mut self) -> &mut ResourceTable {
//         &mut self.resource_table
//     }
// }

// impl WasiView for TacaState {
//     fn ctx(&mut self) -> &mut WasiCtx {
//         &mut self.wasi_ctx
//     }
// }

pub struct Runtime {
    engine: Engine,
    store: wasmtime::Store<TacaState>,
    taca: Taca,
}

pub fn run(archive: Archive) -> Result<Runtime> {
    let component_bytes = archive.read_bytes("app.wasm")?;
    // See: https://github.com/bytecodealliance/wasmtime/blob/main/examples/wasip2/main.rs
    // See: https://github.com/bytecodealliance/wasmtime/blob/main/examples/wasip1/main.rs
    let engine = Engine::new(Config::new().wasm_component_model(true))?;
    // let wasi_ctx = WasiCtxBuilder::new()
    //     .allow_tcp(false)
    //     .allow_udp(false)
    //     .build();
    let mut store = wasmtime::Store::new(
        &engine,
        TacaState {
            taca_core: TacaCoreHost {
                archive,
                table: ResourceTable::new(),
            },
            // resource_table: ResourceTable::new(),
            // wasi_ctx,
        },
    );
    let mut linker = Linker::new(&engine);
    // wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    archive::add_to_linker(&mut linker, |state: &mut TacaState| &mut state.taca_core)?;
    console::add_to_linker(&mut linker, |state: &mut TacaState| &mut state.taca_core)?;
    // key::add_to_linker(&mut linker, |state: &mut TacaState| &mut state.taca_core)?;
    storage::add_to_linker(&mut linker, |state: &mut TacaState| &mut state.taca_core)?;
    task::add_to_linker(&mut linker, |state: &mut TacaState| &mut state.taca_core)?;
    // Use provided file.
    // let component_c: &[u8] = include_bytes!("../../examples/c/hi/out/hi-component.wasm");
    let component = Component::from_binary(&engine, &component_bytes)?;
    let taca = Taca::instantiate(&mut store, &component, &linker)?;
    let app = taca.taca_core_app();
    // let component_rust: &[u8] = include_bytes!(
    //     "../../examples/rust/hi/target/wasm32-unknown-unknown/release/hi-component.wasm"
    // );
    // let component_rust = Component::from_binary(&engine, component_rust)?;
    // let taca_rust = Taca::instantiate(&mut store, &component_rust, &linker)?;
    // let app_rust = taca_rust.taca_core_app();
    app.call_update(&mut store, app::Event::Frame(0.0))?;
    // app_rust.call_update(&mut store, app::EventKind::Frame)?;
    Ok(Runtime {
        engine,
        store,
        taca,
    })
}
