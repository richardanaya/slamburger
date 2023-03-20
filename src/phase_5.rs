use nalgebra::{DMatrix, Matrix3, Vector3, SVD};

use crate::common::*;
use crate::rand::*;

fn keypoints_to_essential(keypoints: &[(KeyPoint, KeyPoint)]) -> Matrix3<f64> {
    // Construct a matrix A from the keypoints
    let mut a = DMatrix::<f64>::zeros(keypoints.len(), 9);
    for (i, &(ref p1, ref p2)) in keypoints.iter().enumerate() {
        a[(i, 0)] = p1.x as f64 * p2.x as f64;
        a[(i, 1)] = p1.y as f64 * p2.x as f64;
        a[(i, 2)] = p2.x as f64;
        a[(i, 3)] = p1.x as f64 * p2.y as f64;
        a[(i, 4)] = p1.y as f64 * p2.y as f64;
        a[(i, 5)] = p2.y as f64;
        a[(i, 6)] = p1.x as f64;
        a[(i, 7)] = p1.y as f64;
        a[(i, 8)] = 1.0;
    }

    // Compute the singular value decomposition of A
    let svd = a.svd(true, true);

    // Extract the singular values and vectors
    let v = svd.v_t.unwrap();

    // Extract the nullspace of A (i.e., the last column of V)
    let e_vec = v.column(8);

    // Reshape the nullspace vector into a 3x3 matrix
    let essential_matrix = Matrix3::from_row_slice(&[
        e_vec[0], e_vec[1], e_vec[2], e_vec[3], e_vec[4], e_vec[5], e_vec[6], e_vec[7], e_vec[8],
    ]);

    essential_matrix
}

fn choose_multiple_keypoints(
    key_points: &Vec<(KeyPoint, KeyPoint)>,
    num_keypoints: usize,
    random: &mut Rand,
) -> Vec<(KeyPoint, KeyPoint)> {
    key_points.choose_multiple(random, num_keypoints)
}

pub fn estimate_essential_ransac(
    key_points: &Vec<(KeyPoint, KeyPoint)>,
    num_iterations: usize,
    inlier_threshold: f64,
    rnd: &mut Rand,
) -> Option<Matrix3<f64>> {
    if key_points.len() < 8 {
        return None;
    }
    let mut best_essential_matrix = None;
    let mut best_num_inliers = 0;

    for _ in 0..num_iterations {
        // Choose a random subset of keypoints
        let subset = choose_multiple_keypoints(key_points, 9, rnd);

        // Compute the essential matrix using the 8-point algorithm
        let essential_matrix = keypoints_to_essential(&subset);

        // Compute the number of inliers that are consistent with the essential matrix
        let mut num_inliers = 0;
        for &(ref p1, ref p2) in key_points.iter() {
            // Compute the epipolar lines corresponding to each keypoint
            let l1 = essential_matrix * Vector3::new(p2.x as f64, p2.y as f64, 1.0);
            let l2 = essential_matrix.transpose() * Vector3::new(p1.x as f64, p1.y as f64, 1.0);

            // Compute the reprojection error for each key
            let error1 = (l1.dot(&Vector3::new(p1.x as f64, p1.y as f64, 1.0))).abs()
                / (l1[0].powi(2) + l1[1].powi(2)).sqrt();
            let error2 = (l2.dot(&Vector3::new(p2.x as f64, p2.y as f64, 1.0))).abs()
                / (l2[0].powi(2) + l2[1].powi(2)).sqrt();

            // If the sum of the errors is below the inlier threshold, count this as an inlier
            if error1 + error2 < inlier_threshold {
                num_inliers += 1;
            }
        }

        // If this solution has more inliers than any previous solution, update the best estimate
        if num_inliers > best_num_inliers {
            best_essential_matrix = Some(essential_matrix);
            best_num_inliers = num_inliers;
        }
    }

    best_essential_matrix
}
