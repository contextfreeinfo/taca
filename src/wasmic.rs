use wasmer::{imports, Function, Instance, Module, Store};

pub fn wasmish() -> anyhow::Result<()> {
    let module_wat = include_str!("hi.wat");

    let mut store = Store::default();
    let module = Module::new(&store, &module_wat)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {
        "" => {
             "hi" => Function::new_typed(&mut store, || println!("Hello")),
        }
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("run")?;
    add_one.call(&mut store, &[])?;
    // assert_eq!(result[0], Value::I32(43));

    Ok(())
}
