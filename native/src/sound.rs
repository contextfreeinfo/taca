use kira::sound::static_sound::StaticSoundData;
use wasmer::ValueType;

#[derive(Debug)]
pub struct Sound {
    pub data: Option<StaticSoundData>,
}

#[derive(Clone, Copy, Debug, Default, ValueType)]
#[repr(C)]
pub struct SoundPlayInfoExtern {
    pub sound: u32,
    pub delay: f32,
    pub rate: f32,
    pub rate_kind: u32,
    pub volume: f32,
    pub volume_kind: u32,
}
