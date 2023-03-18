use crate::{
    common::{Descriptor, KeyPoint},
    rand::Rand,
};
use std::iter;

pub fn compute_brief_descriptors(
    image: &[u8],
    width: u32,
    height: u32,
    keypoints: &[KeyPoint],
    patch_size: usize,
    num_pairs: usize,
    rnd: &mut Rand,
) -> Vec<Descriptor> {
    let sampling_pattern = generate_sampling_pattern(rnd, patch_size, num_pairs);

    keypoints
        .iter()
        .map(|kp| compute_descriptor(image, width, height, kp, &sampling_pattern))
        .collect()
}

fn generate_sampling_pattern(
    rng: &mut Rand,
    patch_size: usize,
    num_pairs: usize,
) -> Vec<((f32, f32), (f32, f32))> {
    iter::repeat_with(|| {
        let x1 = rng.gen_range(-(patch_size as f32 / 2.0)..=(patch_size as f32 / 2.0));
        let y1 = rng.gen_range(-(patch_size as f32 / 2.0)..=(patch_size as f32 / 2.0));
        let x2 = rng.gen_range(-(patch_size as f32 / 2.0)..=(patch_size as f32 / 2.0));
        let y2 = rng.gen_range(-(patch_size as f32 / 2.0)..=(patch_size as f32 / 2.0));
        ((x1, y1), (x2, y2))
    })
    .take(num_pairs)
    .collect()
}

fn compute_descriptor(
    image: &[u8],
    width: u32,
    height: u32,
    keypoint: &KeyPoint,
    sampling_pattern: &[((f32, f32), (f32, f32))],
) -> Descriptor {
    let mut descriptor = Vec::new();
    let mut bit_index = 0;
    let mut current_byte = 0u8;

    for &((x1, y1), (x2, y2)) in sampling_pattern {
        let (x1_rotated, y1_rotated) = rotate_point(x1, y1, keypoint.orientation);
        let (x2_rotated, y2_rotated) = rotate_point(x2, y2, keypoint.orientation);

        let (x1_final, y1_final) = (
            (keypoint.x as f32 + x1_rotated)
                .min(width as f32 - 1.0)
                .max(0.0) as u32,
            (keypoint.y as f32 + y1_rotated)
                .min(height as f32 - 1.0)
                .max(0.0) as u32,
        );
        let (x2_final, y2_final) = (
            (keypoint.x as f32 + x2_rotated)
                .min(width as f32 - 1.0)
                .max(0.0) as u32,
            (keypoint.y as f32 + y2_rotated)
                .min(height as f32 - 1.0)
                .max(0.0) as u32,
        );

        let intensity1 = image[(y1_final * width + x1_final) as usize];
        let intensity2 = image[(y2_final * width + x2_final) as usize];

        if intensity1 > intensity2 {
            current_byte |= 1 << bit_index;
        }

        bit_index += 1;

        if bit_index == 8 {
            descriptor.push(current_byte);
            current_byte = 0;
            bit_index = 0;
        }
    }

    if bit_index > 0 {
        descriptor.push(current_byte);
    }

    Descriptor(descriptor)
}

fn rotate_point(x: f32, y: f32, angle: f32) -> (f32, f32) {
    let sin_angle = angle.sin();
    let cos_angle = angle.cos();
    (x * cos_angle - y * sin_angle, x * sin_angle + y * cos_angle)
}
