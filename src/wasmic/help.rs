#![allow(non_snake_case)]

#[derive(Debug)]
#[repr(C)]
pub struct BufferSlice {
    pub ptr: usize,
    pub size: usize,
    pub item_size: usize,
}
