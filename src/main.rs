use wasmer::{imports, Function, Instance, Module, Store, Value};

fn main() -> anyhow::Result<()> {
    let module_wat = r#"
        (import "host" "hello" (func $host_hello (param i32)))

        (func (export "hello") (param $n i32) (result i32)
            (local.set $n (i32.add (local.get $n) (i32.const 1)))
            (call $host_hello (local.get $n))
            local.get $n
        )
    "#;

    let mut store = Store::default();
    let module = Module::new(&store, &module_wat)?;
    let import_object = imports! {
        "host" => {
            "hello" => Function::new_typed(&mut store, |n: i32| {
                println!("Hello {n}");
            }),
        }
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("hello")?;
    let result = add_one.call(&mut store, &[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));

    Ok(())
}
