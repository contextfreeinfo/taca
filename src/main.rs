use wasmer::{imports, Function, Instance, Module, Store, Value};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let module_wat = r#"
        (import "host" "hello" (func $host_hello (param i32)))

        (func (export "add_one") (param $n i32) (result i32)
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

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&mut store, &[Value::I32(42)])?;
    println!("After: {}", result[0].unwrap_i32());

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
