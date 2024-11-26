use wasmer::ValueType;
use winit::keyboard::KeyCode;

#[derive(Clone, Copy, Debug, Default, ValueType)]
#[repr(C)]
pub struct KeyEvent {
    pub pressed: bool,
    pub key: i32,
    pub modifiers: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Key {
    None = 0,
    ArrowUp = 1,
    ArrowDown = 2,
    ArrowLeft = 3,
    ArrowRight = 4,
    Space = 5,
    Escape = 6,
}

impl From<KeyCode> for Key {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::ArrowDown => Key::ArrowDown,
            KeyCode::ArrowLeft => Key::ArrowLeft,
            KeyCode::ArrowRight => Key::ArrowRight,
            KeyCode::ArrowUp => Key::ArrowUp,
            KeyCode::Escape => Key::Escape,
            KeyCode::Space => Key::Space,
            _ => Key::None,
        }
    }
}
