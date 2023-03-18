use std::alloc::{alloc, dealloc, Layout};
use std::slice;
mod phase_1;
mod phase_2;
use common::Descriptor;
use common::KeyPoint;
mod common;
mod phase_3;
mod phase_4;
mod phase_5;
mod rand;

static mut VEC_PTR: *mut u8 = 0 as *mut u8;
static mut VEC_LEN: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn allocate_vec(size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(size).unwrap();
    let ptr = alloc(layout);

    VEC_PTR = ptr as *mut u8;
    VEC_LEN = size;

    VEC_PTR
}

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

    let (keypoints, descriptors) = get_keypoints_with_descriptors_from_image(slice, width, height);

    let _matched_keypoints =
        phase_4::match_features(&keypoints, &descriptors, &keypoints, &descriptors, 32);

    // now do phase 5
    /*let camera = phase_5::build_intrinsic_matrix(0.5, 0.5, 0.5, 0.5);
    let (rotation, translation) =
        phase_6::perspective_n_point(keypoints_2d, keypoints_3d, intrinsic_matrix);*/

    let layout = Layout::array::<KeyPoint>(keypoints.len()).unwrap();
    let ptr = alloc(layout) as *mut KeyPoint;

    VEC_KEYPOINTS_PTR = ptr;
    VEC_KEYPOINTS_LEN = keypoints.len();

    // copy data to the allocated memory
    ptr.copy_from_nonoverlapping(keypoints.as_ptr(), keypoints.len());

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

pub fn get_keypoints_with_descriptors_from_image(
    img: &mut [u8],
    width: usize,
    height: usize,
) -> (Vec<KeyPoint>, Vec<Descriptor>) {
    // PHASE 1 - convert to grayscale and blur, that way our keypoints are more accurate
    // otherwise we'll get WAY to many identified features of the images to track and will slow things down
    let greyscale = phase_1::rgb_to_grayscale(img, width, height);
    let blurred_img = phase_1::greyscale_gaussian_blur(&greyscale, width, height);

    // PHASE 2 - FAST keypoints
    // This basically finds the corners of the image the the orientation direction it's facing,
    // we call these keypoints
    let threshold: u8 = 30;
    let keypoints = phase_2::fast_keypoints(&blurred_img, width, height, threshold);
    let key_points_with_orientation =
        phase_2::compute_orientations(&blurred_img, width, &keypoints);

    // PHASE 3 - BREIF descriptors
    // This basically looks at surrounding pixels and formulates a sequence of bits that can be used
    // to compare the similarity of two keypoints

    // The patch size is the size of the square we look at around the keypoint
    let patch_size = 16;
    // The number of pairs of pixels we look at to form our sequence of bits
    let num_pairs = 256;
    // The seed is used to generate random numbers for the pairs of pixels we look at
    // we kee the seed equal for every image so that we can compare the same keypoints
    // between images and not overfit to certain patterns
    let seed = 2523523;

    let descriptors = phase_3::compute_brief_descriptors(
        &blurred_img,
        width as u32,
        height as u32,
        &key_points_with_orientation,
        patch_size,
        num_pairs,
        seed,
    );

    (key_points_with_orientation, descriptors)
}
