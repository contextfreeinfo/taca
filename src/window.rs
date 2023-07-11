pub fn run_loop(event_loop: EventLoop<()>, mut store: Store, env: FunctionEnv<System>) {
    // let (system, mut store) = env.data_and_store_mut();
    // let window = system.window.as_ref().unwrap();
    let mut modifiers: ModifiersState = ModifiersState::empty();
    event_loop.run(move |event, _, control_flow| {
        let mut system = env.as_mut(&mut store);
        let window = system.window.as_ref().unwrap();
        let window_listen = system.window_listen.as_ref().unwrap().clone();
        let window_listen_userdata = system.window_listen_userdata;
        fn send_event(
            store: &mut Store,
            function: &Function,
            event_type: WindowEventType,
            userdata: u32,
        ) {
            function
                .call(
                    store,
                    &[
                        // TODO How to put u32 into here? How to just let it wrap?
                        Value::I32((event_type as u32).try_into().unwrap()),
                        Value::I32(userdata.try_into().unwrap()),
                    ],
                )
                .unwrap();
        }
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                // if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => {
                        send_event(
                            &mut store,
                            &window_listen,
                            WindowEventType::Close,
                            window_listen_userdata,
                        );
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    } => {
                        if *state == ElementState::Pressed
                            && (*key == VirtualKeyCode::F11
                                || *key == VirtualKeyCode::Return && modifiers.alt())
                        {
                            window.set_fullscreen(match window.fullscreen() {
                                Some(_) => None,
                                None => Some(winit::window::Fullscreen::Borderless(None)),
                            })
                        } else {
                            system.key_event = Some(KeyEvent {
                                code: convert_key(*key),
                                pressed: *state == ElementState::Pressed,
                            });
                            send_event(
                                &mut store,
                                &window_listen,
                                WindowEventType::Key,
                                window_listen_userdata,
                            );
                        }
                    }
                    WindowEvent::ModifiersChanged(state) => {
                        modifiers = *state;
                    }
                    WindowEvent::Resized(_physical_size) => {
                        send_event(
                            &mut store,
                            &window_listen,
                            WindowEventType::Resize,
                            window_listen_userdata,
                        );
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        send_event(
                            &mut store,
                            &window_listen,
                            WindowEventType::Resize,
                            window_listen_userdata,
                        );
                    }
                    _ => {}
                }
                // }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                send_event(
                    &mut store,
                    &window_listen,
                    WindowEventType::Redraw,
                    window_listen_userdata,
                );
                // state.update();
                // state.render();
                // match state.render() {
                //     Ok(_) => {}
                //     // Reconfigure the surface if it's lost or outdated
                //     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                //     // The system is out of memory, we should probably quit
                //     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                //     Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                // }
            }
            Event::RedrawEventsCleared => {
                // std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / 60.0));
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                // TODO Require redraw request from app before requesting?
                window.request_redraw();
            }
            _ => {}
        }
    });
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum WindowEventType {
    Close = 1,
    Key = 2,
    Redraw = 3,
    Resize = 4,
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum KeyCode {
    Undefined = 0,
    Left = 1,
    Up = 2,
    Right = 3,
    Down = 4,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct KeyEvent {
    code: KeyCode,
    pressed: bool,
}

fn convert_key(wkey: VirtualKeyCode) -> KeyCode {
    match wkey {
        VirtualKeyCode::Left => KeyCode::Left,
        VirtualKeyCode::Up => KeyCode::Up,
        VirtualKeyCode::Right => KeyCode::Right,
        VirtualKeyCode::Down => KeyCode::Down,
        _ => KeyCode::Undefined,
    }
}

#[derive(Copy, Clone, Debug, ValueType)]
#[repr(C)]
struct WasmKeyEvent {
    code: u32,
    pressed: bool,
}

pub fn taca_key_event(mut env: FunctionEnvMut<System>, result: u32) {
    // println!("taca_keyEvent({result})");
    let (system, mut store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&mut store);
    let key_event = match system.key_event {
        Some(key_event) => WasmKeyEvent {
            code: key_event.code as u32,
            pressed: key_event.pressed,
        },
        None => WasmKeyEvent {
            code: KeyCode::Undefined as u32,
            pressed: false,
        },
    };
    let result = result as u64;
    WasmRef::<WasmKeyEvent>::new(&view, result)
        .write(key_event)
        .unwrap();
}

pub fn taca_window_inner_size(mut env: FunctionEnvMut<System>, result: u32) {
    println!("taca_windowInnerSize({result})");
    let (system, mut store) = env.data_and_store_mut();
    let memory = system.memory.as_ref().unwrap().view(&mut store);
    let size = system.window.as_ref().unwrap().inner_size();
    let result = result as u64;
    memory.write(result, &size.width.to_le_bytes()).unwrap();
    memory
        .write(result + 4, &size.height.to_le_bytes())
        .unwrap();
}

pub fn taca_window_listen(mut env: FunctionEnvMut<System>, callback: u32, userdata: u32) {
    println!("taca_windowListen({callback}, {userdata})");
    let (mut system, mut store) = env.data_and_store_mut();
    let functions = system.functions.as_ref().unwrap();
    system.window_listen = match callback {
        // Support option for named export in case some compilers don't like
        // exporting function tables.
        0 => system.named_window_listen.clone(),
        _ => {
            let value = functions.get(&mut store, callback).unwrap();
            Some(value.unwrap_funcref().as_ref().unwrap().clone())
        }
    };
    system.window_listen_userdata = userdata;
}

pub fn taca_window_set_title(mut env: FunctionEnvMut<System>, title: u32) {
    println!("taca_windowSetTitle({title})");
    let (system, store) = env.data_and_store_mut();
    let view = system.memory.as_ref().unwrap().view(&store);
    let title = WasmPtr::<u8>::new(title)
        .read_utf8_string_with_nul(&view)
        .unwrap();
    let window = system.window.as_ref().unwrap();
    window.set_title(title.as_str());
}

use crate::system::*;
use wasmer::{Function, FunctionEnv, FunctionEnvMut, Store, Value, ValueType, WasmPtr, WasmRef};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};
