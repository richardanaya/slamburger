use common::KeyPoint;
use std::alloc::{alloc, Layout};
use std::slice;
mod common;
mod phase_1;
mod phase_2;
mod phase_3;
mod phase_4;
mod phase_5;
mod rand;
mod slam;

extern "C" {
    fn js_log(a: f64) -> f64;
}

pub fn signal(signal: f64) {
    unsafe {
        js_log(signal);
    }
}

pub fn log(signal: f64) {
    unsafe {
        js_log(signal);
    }
}

static mut VEC_PTR_SLOT_0: *mut u8 = 0 as *mut u8;
static mut VEC_LEN_SLOT_0: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn allocate_slot_0(size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(size).unwrap();
    let ptr = alloc(layout);

    VEC_PTR_SLOT_0 = ptr as *mut u8;
    VEC_LEN_SLOT_0 = size;

    VEC_PTR_SLOT_0
}

static mut VEC_PTR_SLOT_1: *mut u8 = 0 as *mut u8;
static mut VEC_LEN_SLOT_1: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn allocate_slot_1(size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(size).unwrap();
    let ptr = alloc(layout);

    VEC_PTR_SLOT_1 = ptr as *mut u8;
    VEC_LEN_SLOT_1 = size;

    VEC_PTR_SLOT_1
}

static mut VEC_KEYPOINTS_SLOT_0_PTR: *mut KeyPoint = 0 as *mut KeyPoint;
static mut VEC_KEYPOINTS_SLOT_0_LEN: usize = 0;

static mut VEC_KEYPOINTS_SLOT_1_PTR: *mut KeyPoint = 0 as *mut KeyPoint;
static mut VEC_KEYPOINTS_SLOT_1_LEN: usize = 0;

#[no_mangle]
pub unsafe fn calculate(width: usize, height: usize, slot: usize) -> usize {
    let slice_0 = slice::from_raw_parts_mut(VEC_PTR_SLOT_0, VEC_LEN_SLOT_0);
    let slice_1 = slice::from_raw_parts_mut(VEC_PTR_SLOT_1, VEC_LEN_SLOT_1);

    let mut image_a = common::Image {
        data: slice_0,
        width,
        height,
    };

    let slice = slice::from_raw_parts_mut(VEC_PTR_SLOT_1, VEC_LEN_SLOT_1);
    core::mem::forget(slice);

    let mut image_b = common::Image {
        data: slice_1,
        width,
        height,
    };

    if slot == 1 {
        let image = image_a;
        image_a = image_b;
        image_b = image;
    }

    let mut slam = slam::Slam::new(image_a, image_b);

    let (_result, matched_keypoints, keypoints_and_descriptors_a, keypoints_and_descriptors_b) =
        slam.calculate_pose();

    let layout = Layout::array::<KeyPoint>(keypoints_and_descriptors_a.0.len()).unwrap();
    let ptr = alloc(layout) as *mut KeyPoint;

    VEC_KEYPOINTS_SLOT_0_PTR = ptr;
    VEC_KEYPOINTS_SLOT_0_LEN = keypoints_and_descriptors_a.0.len();

    // copy data to the allocated memory
    ptr.copy_from_nonoverlapping(
        keypoints_and_descriptors_a.0.as_ptr(),
        keypoints_and_descriptors_a.0.len(),
    );

    let layout = Layout::array::<KeyPoint>(keypoints_and_descriptors_b.0.len()).unwrap();
    let ptr = alloc(layout) as *mut KeyPoint;

    VEC_KEYPOINTS_SLOT_1_PTR = ptr;
    VEC_KEYPOINTS_SLOT_1_LEN = keypoints_and_descriptors_b.0.len();

    // copy data to the allocated memory
    ptr.copy_from_nonoverlapping(
        keypoints_and_descriptors_b.0.as_ptr(),
        keypoints_and_descriptors_b.0.len(),
    );

    let grey = keypoints_and_descriptors_a.2;
    let mut rgb_grey = vec![0; grey.len() * 4];

    grey.iter().enumerate().for_each(|(i, &v)| {
        rgb_grey[i * 4] = v;
        rgb_grey[i * 4 + 1] = v;
        rgb_grey[i * 4 + 2] = v;
        rgb_grey[i * 4 + 3] = 255;
    });

    let layout = Layout::array::<u8>(rgb_grey.len()).unwrap();
    let ptr = alloc(layout) as *mut u8;

    VEC_GRAYSCALE_PTR = ptr;
    VEC_GRAYSCALE_LEN = rgb_grey.len();

    // copy data to the allocated memory
    ptr.copy_from_nonoverlapping(rgb_grey.as_ptr(), rgb_grey.len());

    matched_keypoints.len()
}

static mut VEC_GRAYSCALE_PTR: *mut u8 = 0 as *mut u8;
static mut VEC_GRAYSCALE_LEN: usize = 0;

#[no_mangle]
pub unsafe fn get_grayscale() -> *mut u8 {
    VEC_GRAYSCALE_PTR
}

#[no_mangle]
pub unsafe fn get_grayscale_len() -> usize {
    VEC_GRAYSCALE_LEN
}

#[no_mangle]
pub unsafe fn get_keypoints_slot_0() -> *mut KeyPoint {
    VEC_KEYPOINTS_SLOT_0_PTR
}

#[no_mangle]
pub unsafe fn get_keypoints_slot_0_len() -> usize {
    VEC_KEYPOINTS_SLOT_0_LEN
}

#[no_mangle]
pub unsafe fn get_keypoints_slot_1() -> *mut KeyPoint {
    VEC_KEYPOINTS_SLOT_1_PTR
}

#[no_mangle]
pub unsafe fn get_keypoints_slot_1_len() -> usize {
    VEC_KEYPOINTS_SLOT_1_LEN
}
