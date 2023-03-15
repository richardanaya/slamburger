use std::alloc::{alloc, dealloc, Layout};
use std::slice;

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KeyPoint {
    pub x: f32,
    pub y: f32,
    pub orientation: f32,
}

static mut VEC_GRAYSCALE_PTR: *mut u8 = 0 as *mut u8;
static mut VEC_GRAYSCALE_LEN: usize = 0;

static mut VEC_KEYPOINTS_PTR: *mut KeyPoint = 0 as *mut KeyPoint;
static mut VEC_KEYPOINTS_LEN: usize = 0;

#[no_mangle]
pub unsafe fn calculate(width: usize, height: usize) -> usize {
    let slice = slice::from_raw_parts_mut(VEC_PTR, VEC_LEN);

    let keypoints = get_image_fast_keypoints(slice, width, height);

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

pub fn get_image_fast_keypoints(img: &mut [u8], width: usize, height: usize) -> Vec<KeyPoint> {
    let greyscale = rgb_to_grayscale(img, width, height);
    let blurred_img = greyscale_gaussian_blur(&greyscale, width, height);
    let threshold: u8 = 30;
    let keypoints = fast_keypoints(&blurred_img, width, height, threshold);
    compute_orientations(&blurred_img, width, &keypoints)
}

pub fn rgb_to_grayscale<'a>(img: &'a mut [u8], width: usize, height: usize) -> &'a [u8] {
    for y in 0..height {
        for x in 0..width {
            let idx = 4 * (y * width + x);
            let r = img[idx] as f32;
            let g = img[idx + 1] as f32;
            let b = img[idx + 2] as f32;
            let g = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            img[y * width + x] = g;
        }
    }
    &img[0..(width * height)]
}

pub fn greyscale_gaussian_blur(img: &[u8], width: usize, height: usize) -> Vec<u8> {
    const KERNEL_SIZE: usize = 5;
    const KERNEL: [f32; KERNEL_SIZE] = [0.06136, 0.24477, 0.38774, 0.24477, 0.06136];

    let mut output = vec![0u8; img.len()];
    let mut buffer = vec![0f32; width * height];

    // Perform horizontal blur
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            for i in 0..KERNEL_SIZE {
                let index =
                    (x as i32 - 2 + i as i32).clamp(0, width as i32 - 1) as usize + y * width;
                sum += KERNEL[i] * img[index] as f32;
            }
            buffer[x + y * width] = sum;
        }
    }

    // Perform vertical blur
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            for i in 0..KERNEL_SIZE {
                let index =
                    x + (y as i32 - 2 + i as i32).clamp(0, height as i32 - 1) as usize * width;
                sum += KERNEL[i] * buffer[index];
            }
            output[x + y * width] = sum.round() as u8;
        }
    }

    output
}

fn is_corner(p: u8, circle: &[u8], threshold: u8) -> bool {
    let mut count = 0;
    let mut consecutive = 0;

    for (_, &pixel) in circle.iter().enumerate() {
        if (pixel as i32 - p as i32).abs() > threshold as i32 {
            consecutive += 1;
            if consecutive == 9 {
                return true;
            }
        } else {
            count += consecutive;
            consecutive = 0;
        }
    }
    count + consecutive >= 9
}

const OFFSETS: [(i32, i32); 12] = [
    (-3, 0),
    (0, 3),
    (3, 0),
    (0, -3),
    (-1, 3),
    (1, 3),
    (3, 1),
    (3, -1),
    (1, -3),
    (-1, -3),
    (-3, 1),
    (-3, -1),
];

fn fast_keypoints(img: &[u8], width: usize, height: usize, threshold: u8) -> Vec<(usize, usize)> {
    let mut keypoints = Vec::new();
    for y in 3..(height - 3) {
        for x in 3..(width - 3) {
            let p = img[y * width + x];
            let circle: Vec<u8> = OFFSETS
                .iter()
                .map(|&(dx, dy)| img[(y as i32 + dy) as usize * width + (x as i32 + dx) as usize])
                .collect();

            if is_corner(p, &circle, threshold) {
                keypoints.push((x, y));
            }
        }
    }
    keypoints
}

fn compute_orientations(img: &[u8], width: usize, keypoints: &[(usize, usize)]) -> Vec<KeyPoint> {
    let circle_offsets = [
        (-1, -3),
        (0, -3),
        (1, -3),
        (-2, -2),
        (2, -2),
        (-3, -1),
        (3, -1),
        (-3, 0),
        (3, 0),
        (-3, 1),
        (3, 1),
        (-2, 2),
        (2, 2),
        (-1, 3),
        (0, 3),
        (1, 3),
    ];

    keypoints
        .iter()
        .map(|&(x, y)| {
            let mut m_x = 0.0;
            let mut m_y = 0.0;

            for &(dx, dy) in circle_offsets.iter() {
                let x_offset = (x as i32 + dx) as usize;
                let y_offset = (y as i32 + dy) as usize;
                let w = img[y_offset * width + x_offset] as f32;

                m_x += w * dx as f32;
                m_y += w * dy as f32;
            }

            KeyPoint {
                // this represents the dominant direction of the gradient
                orientation: m_y.atan2(m_x),
                x: x as f32,
                y: y as f32,
            }
        })
        .collect()
}
