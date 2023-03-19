use common::KeyPoint;
use std::alloc::{alloc, dealloc, Layout};
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

static mut VEC_KEYPOINTS_PTR: *mut KeyPoint = 0 as *mut KeyPoint;
static mut VEC_KEYPOINTS_LEN: usize = 0;

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

    let (_result, _matched_keypoints, keypoints_and_descriptors_a, keypoints_and_descriptors_b) =
        slam.calculate_pose();

    keypoints_and_descriptors_a.0.len() - keypoints_and_descriptors_b.0.len()

    /*

    let (_result, matched_keypoints) = slam.calculate_pose();

    // split matched_keypoints into two vectors
    let mut keypoints_a = Vec::new();
    let mut keypoints_b = Vec::new();
    for (keypoint_a, keypoint_b) in matched_keypoints {
        keypoints_a.push(keypoint_a);
        keypoints_b.push(keypoint_b);
    }

    let layout = Layout::array::<KeyPoint>(keypoints_a.len()).unwrap();
    let ptr = alloc(layout) as *mut KeyPoint;

    VEC_KEYPOINTS_PTR = ptr;
    VEC_KEYPOINTS_LEN = keypoints_a.len();

    // copy data to the allocated memory
    ptr.copy_from_nonoverlapping(keypoints_a.as_ptr(), keypoints_a.len());

    VEC_KEYPOINTS_LEN*/
}

/*
#[no_mangle]
pub unsafe extern "C" fn get_value(i: usize) -> u8 {
    let slice = slice::from_raw_parts_mut(VEC_PTR, VEC_LEN);
    slice[i]
}

#[no_mangle]
pub unsafe extern "C" fn deallocate_vec() {
    let layout = Layout::array::<u32>(VEC_LEN).unwrap();
    dealloc(VEC_PTR as *mut u8, layout);
}

static mut VEC_GRAYSCALE_PTR: *mut u8 = 0 as *mut u8;
static mut VEC_GRAYSCALE_LEN: usize = 0;

static mut VEC_KEYPOINTS_PTR: *mut KeyPoint = 0 as *mut KeyPoint;
static mut VEC_KEYPOINTS_LEN: usize = 0;

#[no_mangle]
pub unsafe fn calculate(width: usize, height: usize) -> usize {
    let slice = slice::from_raw_parts_mut(VEC_PTR, VEC_LEN);

    let image_a = common::Image {
        data: slice,
        width,
        height,
    };

    let image_b = common::Image {
        data: slice,
        width,
        height,
    };

    let mut slam = slam::Slam::new(image_a, image_b);

    let (_result, matched_keypoints) = slam.calculate_pose();

    // split matched_keypoints into two vectors
    let mut keypoints_a = Vec::new();
    let mut keypoints_b = Vec::new();
    for (keypoint_a, keypoint_b) in matched_keypoints {
        keypoints_a.push(keypoint_a);
        keypoints_b.push(keypoint_b);
    }

    let layout = Layout::array::<KeyPoint>(keypoints_a.len()).unwrap();
    let ptr = alloc(layout) as *mut KeyPoint;

    VEC_KEYPOINTS_PTR = ptr;
    VEC_KEYPOINTS_LEN = keypoints_a.len();

    // copy data to the allocated memory
    ptr.copy_from_nonoverlapping(keypoints_a.as_ptr(), keypoints_a.len());

    VEC_KEYPOINTS_LEN
}

#[no_mangle]
pub unsafe fn get_keypoints() -> *mut KeyPoint {
    VEC_KEYPOINTS_PTR
}

#[no_mangle]
pub unsafe fn get_grey_scale() -> *mut u8 {
    VEC_GRAYSCALE_PTR
}

#[no_mangle]
pub unsafe fn get_grey_scale_len() -> usize {
    VEC_GRAYSCALE_LEN
}
*/
