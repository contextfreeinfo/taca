pub type WindowEventType = u32;
#[allow(non_upper_case_globals)]
const tac_WindowEventType_Close: WindowEventType = 1;
#[allow(non_upper_case_globals)]
const tac_WindowEventType_Redraw: WindowEventType = 2;
#[allow(non_upper_case_globals)]
const tac_WindowEventType_Resize: WindowEventType = 3;

pub fn run_loop(event_loop: EventLoop<()>, mut store: Store, env: FunctionEnv<System>) {
    // let (system, mut store) = env.data_and_store_mut();
    // let window = system.window.as_ref().unwrap();
    let mut modifiers: ModifiersState = ModifiersState::empty();
    event_loop.run(move |event, _, control_flow| {
        let system = env.as_ref(&store);
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
                        Value::I32(event_type.try_into().unwrap()),
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
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        send_event(
                            &mut store,
                            &window_listen,
                            tac_WindowEventType_Close,
                            window_listen_userdata,
                        );
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    } => {
                        if *key == VirtualKeyCode::F11
                            || *key == VirtualKeyCode::Return && modifiers.alt()
                        {
                            window.set_fullscreen(match window.fullscreen() {
                                Some(_) => None,
                                None => Some(winit::window::Fullscreen::Borderless(None)),
                            })
                        }
                    }
                    WindowEvent::ModifiersChanged(state) => {
                        modifiers = *state;
                    }
                    WindowEvent::Resized(_physical_size) => {
                        send_event(
                            &mut store,
                            &window_listen,
                            tac_WindowEventType_Resize,
                            window_listen_userdata,
                        );
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        send_event(
                            &mut store,
                            &window_listen,
                            tac_WindowEventType_Resize,
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
                    tac_WindowEventType_Redraw,
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
                std::thread::sleep(std::time::Duration::from_millis(16));
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                // state.window().request_redraw();
            }
            _ => {}
        }
    });
}

pub fn tac_window_inner_size(mut env: FunctionEnvMut<System>, result: u32) {
    println!("tac_windowInnerSize({result})");
    let (system, mut store) = env.data_and_store_mut();
    let memory = system.memory.as_ref().unwrap().view(&mut store);
    let size = system.window.as_ref().unwrap().inner_size();
    let result = result as u64;
    memory.write(result, &size.width.to_le_bytes()).unwrap();
    memory
        .write(result + 4, &size.height.to_le_bytes())
        .unwrap();
}

pub fn tac_window_listen(mut env: FunctionEnvMut<System>, callback: u32, userdata: u32) {
    println!("tac_windowListen({callback}, {userdata})");
    let (mut system, mut store) = env.data_and_store_mut();
    let functions = system.functions.as_ref().unwrap();
    let value = functions.get(&mut store, callback).unwrap();
    system.window_listen = Some(value.unwrap_funcref().as_ref().unwrap().clone());
    system.window_listen_userdata = userdata;
}

use crate::system::*;
use wasmer::{Function, FunctionEnv, FunctionEnvMut, Store, Value};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};
