use nalgebra::Matrix3;
use nalgebra::Vector3;

use crate::common::*;
use crate::phase_1;
use crate::phase_2;
use crate::phase_3;
use crate::phase_4;
use crate::phase_5;
use crate::phase_6;
use crate::rand::*;

pub struct Slam<'a> {
    image_a: Image<'a>,
    image_b: Image<'a>,
    random: Rand,
    patch_size: usize,
    num_pairs: usize,
    max_hamming_distance: usize,
    blur_radius: f32,
    essential_num_iterations: usize,
    essential_threshold: f32,
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
            essential_num_iterations: 1000,
            essential_threshold: 0.01,
        }
    }

    pub fn calculate_pose(
        &mut self,
    ) -> (
        Option<(Matrix3<f64>, Vector3<f64>)>,
        Vec<(KeyPoint, KeyPoint)>,
        (Vec<KeyPoint>, Vec<Descriptor>, Vec<u8>),
        (Vec<KeyPoint>, Vec<Descriptor>),
    ) {
        let (key_points_with_orientation_a, blurred_image_a) = {
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

            (key_points_with_orientation, blurred_img)
        };

        let (key_points_with_orientation_b, blurred_image_b) = {
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

            (key_points_with_orientation, blurred_img)
        };

        // PHASE 3  -  Compute BRIEF descriptors for each keypoint so we can visually match them
        let sampling_pattern =
            phase_3::generate_sampling_pattern(&mut self.random, self.patch_size, self.num_pairs);

        let descriptors_a = phase_3::compute_brief_descriptors(
            &blurred_image_a,
            self.image_a.width as u32,
            self.image_a.height as u32,
            &key_points_with_orientation_b,
            &sampling_pattern,
        );

        let descriptors_b = phase_3::compute_brief_descriptors(
            &blurred_image_b,
            self.image_b.width as u32,
            self.image_b.height as u32,
            &key_points_with_orientation_b,
            &sampling_pattern,
        );

        // PHASE 4  -  Match features between the two images

        let matched_keypoints = phase_4::match_features(
            &key_points_with_orientation_a,
            &descriptors_a,
            &key_points_with_orientation_b,
            &descriptors_b,
            self.max_hamming_distance,
        );

        // PHASE 5  -  RANSAC to find the best rotation and translation using 8 point algorithm
        let essential_matrix = phase_5::estimate_essential_ransac(
            &matched_keypoints,
            *&self.essential_num_iterations,
            *&self.essential_threshold as f64,
            &mut self.random,
        );

        // PHASE 6  -  Decompose the essential matrix to find the rotation and translation
        let decomposed_essential = if let Some(essential) = essential_matrix {
            Some(phase_6::decompose_essential_matrix(essential))
        } else {
            None
        };

        (
            decomposed_essential,
            matched_keypoints,
            (
                key_points_with_orientation_a,
                descriptors_a,
                blurred_image_a,
            ),
            (key_points_with_orientation_b, descriptors_b),
        )
    }
}
