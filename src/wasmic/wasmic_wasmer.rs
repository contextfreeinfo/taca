use wasmer::{imports, Function, FunctionEnv, FunctionEnvMut, Instance, Module, Store, Value};

pub fn wasmish(wasm: &[u8]) -> anyhow::Result<()> {
    let mut store = Store::default();
    let module = Module::new(&store, wasm)?;
    let env = FunctionEnv::new(&mut store, 5);
    let import_object = imports! {
        "env" => {
             "hi" => Function::new_typed_with_env(&mut store, &env, hi),
        }
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let start = instance.exports.get_function("_start")?;
    start.call(&mut store, &[])?;

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&mut store, &[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));

    Ok(())
}

pub fn print(text: &str) {
    println!("{text}");
}

fn hi(mut _env: FunctionEnvMut<i32>) {
    crate::say_hi();
}
