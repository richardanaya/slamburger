#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KeyPoint {
    pub x: f32,
    pub y: f32,
    pub orientation: f32,
}

pub struct Descriptor(pub Vec<u8>);

pub struct Image<'a> {
    pub width: usize,
    pub height: usize,
    pub data: &'a [u8],
}
