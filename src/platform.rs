// #[repr(C)]
pub struct Platform {
    pub buffer: Vec<u8>,
}

impl Platform {
    pub fn new(buffer_len: usize) -> Platform {
        Platform {
            buffer: vec![0; buffer_len],
        }
    }
}
