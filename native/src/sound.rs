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
    pub rate: f32,
    pub rate_kind: u32,
}