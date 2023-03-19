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
    blur_radius: f32,
}

impl<'a> Slam<'a> {
    pub fn new(image_a: Image<'a>, image_b: Image<'a>) -> Slam<'a> {
        let seed = 2523523;
        let random = Rand::new_with_seed(seed);
        Slam {
            image_a,
            image_b,
            random,
            patch_size: 100,
            num_pairs: 500,
            max_hamming_distance: 300,
            blur_radius: 3.0,
        }
    }

    pub fn calculate_pose(
        &mut self,
    ) -> (
        Option<(Matrix3<f32>, Vector3<f32>)>,
        Vec<(KeyPoint, KeyPoint)>,
        (Vec<KeyPoint>, Vec<Descriptor>, Vec<u8>),
        (Vec<KeyPoint>, Vec<Descriptor>),
    ) {
        let key_points_with_descriptors_a = {
            let width = self.image_a.width;
            let height = self.image_a.height;

            // PHASE 1  -  Convert RGB image to greyscale and blur it with a Gaussian filter
            let greyscale = phase_1::rgb_to_grayscale(self.image_a.data, width, height);
            let blurred_img =
                phase_1::greyscale_gaussian_blur(&greyscale, width, height, self.blur_radius);
            let threshold: u8 = 30;

            // PHASE 2  -  Detect FAST keypoints and compute their orientations
            let keypoints = phase_2::fast_keypoints(&blurred_img, width, height, threshold);
            let key_points_with_orientation =
                phase_2::compute_orientations(&blurred_img, width, &keypoints);

            // PHASE 3  -  Compute BRIEF descriptors for each keypoint so we can visually match them
            let sampling_pattern = phase_3::generate_sampling_pattern(
                &mut self.random,
                self.patch_size,
                self.num_pairs,
            );
            let descriptors = phase_3::compute_brief_descriptors(
                &blurred_img,
                width as u32,
                height as u32,
                &key_points_with_orientation,
                &sampling_pattern,
            );
            (key_points_with_orientation, descriptors, blurred_img)
        };

        let key_points_with_descriptors_b = {
            let width = self.image_b.width;
            let height = self.image_b.height;

            // PHASE 1  -  Convert RGB image to greyscale and blur it with a Gaussian filter
            let greyscale = phase_1::rgb_to_grayscale(self.image_b.data, width, height);
            let blurred_img =
                phase_1::greyscale_gaussian_blur(&greyscale, width, height, self.blur_radius);
            let threshold: u8 = 30;

            // PHASE 2  -  Detect FAST keypoints and compute their orientations
            let keypoints = phase_2::fast_keypoints(&blurred_img, width, height, threshold);
            let key_points_with_orientation =
                phase_2::compute_orientations(&blurred_img, width, &keypoints);

            // PHASE 3  -  Compute BRIEF descriptors for each keypoint so we can visually match them
            let sampling_pattern = phase_3::generate_sampling_pattern(
                &mut self.random,
                self.patch_size,
                self.num_pairs,
            );
            let descriptors = phase_3::compute_brief_descriptors(
                &blurred_img,
                width as u32,
                height as u32,
                &key_points_with_orientation,
                &sampling_pattern,
            );
            (key_points_with_orientation, descriptors)
        };

        // PHASE 4  -  Match features between image A and B using Hamming distance of BRIEF descriptors
        let (keypoints_a, descriptors_a, blurred_img) = key_points_with_descriptors_a;
        let (keypoints_b, descriptors_b) = key_points_with_descriptors_b;

        let matched_keypoints = phase_4::match_features(
            &keypoints_a,
            &descriptors_a,
            &keypoints_b,
            &descriptors_b,
            self.max_hamming_distance,
        );

        // PHASE 5  -  RANSAC to find the best rotation and translation using 8 point algorithm
        (
            phase_5::calculate_rotation_translation(&matched_keypoints, &mut self.random),
            matched_keypoints,
            (keypoints_a, descriptors_a, blurred_img),
            (keypoints_b, descriptors_b),
        )
    }
}
