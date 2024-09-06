use wasmer::ValueType;
use winit::keyboard::NamedKey;

#[derive(Clone, Copy, Debug, Default, ValueType)]
#[repr(C)]
pub struct KeyEvent {
    pub key: i32,
    pub pressed: bool,
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

impl From<NamedKey> for Key {
    fn from(value: NamedKey) -> Self {
        match value {
            NamedKey::ArrowDown => Key::ArrowDown,
            NamedKey::ArrowLeft => Key::ArrowLeft,
            NamedKey::ArrowRight => Key::ArrowRight,
            NamedKey::ArrowUp => Key::ArrowUp,
            NamedKey::Escape => Key::Escape,
            NamedKey::Space => Key::Space,
            _ => Key::None,
        }
    }
}
