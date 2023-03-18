use nalgebra::Matrix3;
use nalgebra::Vector3;

use crate::common::*;
use crate::phase_1;
use crate::phase_2;
use crate::phase_3;
use crate::phase_4;
use crate::phase_5;
use crate::rand::*;

pub struct Slam<'a> {
    image_a: Image<'a>,
    image_b: Image<'a>,
    random: Rand,
    patch_size: usize,
    num_pairs: usize,
    max_hamming_distance: usize,
}

impl<'a> Slam<'a> {
    pub fn new(image_a: Image<'a>, image_b: Image<'a>) -> Slam<'a> {
        let seed = 2523523;
        let random = Rand::new_with_seed(seed);
        Slam {
            image_a,
            image_b,
            random,
            patch_size: 16,
            num_pairs: 256,
            max_hamming_distance: 32,
        }
    }

    pub fn calculate_pose(
        &mut self,
    ) -> (
        Option<(Matrix3<f32>, Vector3<f32>)>,
        Vec<(KeyPoint, KeyPoint)>,
    ) {
        let key_points_with_descriptors_a = {
            let width = self.image_a.width;
            let height = self.image_a.height;
            let greyscale = phase_1::rgb_to_grayscale(self.image_a.data, width, height);
            let blurred_img = phase_1::greyscale_gaussian_blur(&greyscale, width, height);
            let threshold: u8 = 30;
            let keypoints = phase_2::fast_keypoints(&blurred_img, width, height, threshold);
            let key_points_with_orientation =
                phase_2::compute_orientations(&blurred_img, width, &keypoints);
            let descriptors = phase_3::compute_brief_descriptors(
                &blurred_img,
                width as u32,
                height as u32,
                &key_points_with_orientation,
                self.patch_size,
                self.num_pairs,
                &mut self.random,
            );
            (key_points_with_orientation, descriptors)
        };

        let key_points_with_descriptors_b = {
            let width = self.image_b.width;
            let height = self.image_b.height;
            let greyscale = phase_1::rgb_to_grayscale(self.image_b.data, width, height);
            let blurred_img = phase_1::greyscale_gaussian_blur(&greyscale, width, height);
            let threshold: u8 = 30;
            let keypoints = phase_2::fast_keypoints(&blurred_img, width, height, threshold);
            let key_points_with_orientation =
                phase_2::compute_orientations(&blurred_img, width, &keypoints);
            let descriptors = phase_3::compute_brief_descriptors(
                &blurred_img,
                width as u32,
                height as u32,
                &key_points_with_orientation,
                self.patch_size,
                self.num_pairs,
                &mut self.random,
            );
            (key_points_with_orientation, descriptors)
        };

        let (keypoints_a, descriptors_a) = key_points_with_descriptors_a;
        let (keypoints_b, descriptors_b) = key_points_with_descriptors_b;

        let matched_keypoints = phase_4::match_features(
            &keypoints_a,
            &descriptors_a,
            &keypoints_b,
            &descriptors_b,
            self.max_hamming_distance,
        );

        (
            phase_5::calculate_rotation_translation(&matched_keypoints, &mut self.random),
            matched_keypoints,
        )
    }
}
