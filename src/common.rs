#[repr(C)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct KeyPoint {
    pub x: f32,
    pub y: f32,
    pub orientation: f32,
}
#[derive(PartialEq, Debug, Clone)]
pub struct Descriptor(pub Vec<u8>);

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Image<'a> {
    pub width: usize,
    pub height: usize,
    pub data: &'a [u8],
}
