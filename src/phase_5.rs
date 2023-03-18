use crate::common::KeyPoint;
use crate::rand::ChooseMultiple;
use crate::rand::Rand;
use nalgebra::{Matrix3, Vector3, SVD};

pub fn calculate_rotation_translation(
    keypoints: &[(KeyPoint, KeyPoint)],
    rnd: &mut Rand,
) -> Option<(Matrix3<f32>, Vector3<f32>)> {
    let e = ransac_essential_matrix(&keypoints, 0.01, 1000, rnd)?;

    // Decompose essential matrix using SVD
    let svd = SVD::new(e, true, true);
    let u = svd.u.unwrap();
    let v_t = svd.v_t.unwrap();
    let w = Matrix3::new(0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);

    let r1 = u * w * v_t;
    let r2 = u * w.transpose() * v_t;

    let t = u.column(2);

    // Choose the correct rotation matrix based on the determinant
    let rotation = if r1.determinant() > 0.0 && r2.determinant() > 0.0 {
        if (r1 - r2).norm() < 1e-6 {
            r1
        } else {
            // Ambiguity in the correct rotation matrix
            return None;
        }
    } else if r1.determinant() > 0.0 {
        r1
    } else if r2.determinant() > 0.0 {
        r2
    } else {
        // Both rotation matrices have a negative determinant
        return None;
    };

    Some((rotation, t.into()))
}

fn ransac_essential_matrix(
    keypoints: &[(KeyPoint, KeyPoint)],
    threshold: f32,
    iterations: usize,
    rnd: &mut Rand,
) -> Option<Matrix3<f32>> {
    let n = keypoints.len();
    if n < 8 {
        return None;
    }

    let mut best_e = None;
    let mut best_inliers = Vec::new();

    for _ in 0..iterations {
        // 1. Randomly select a subset of 8 keypoint pairs
        let subset_indices: Vec<usize> = (0..n).collect();
        let subset_indices = subset_indices.choose_multiple(rnd, 8);

        let kps1_subset: Vec<KeyPoint> = subset_indices.iter().map(|&i| keypoints[i].0).collect();
        let kps2_subset: Vec<KeyPoint> = subset_indices.iter().map(|&i| keypoints[i].1).collect();

        // 2. Compute the essential matrix using the selected subsetm
        if let Some(e) = compute_essential_matrix(&kps1_subset, &kps2_subset) {
            // 3. Evaluate the number of inliers
            let mut inliers = Vec::new();
            for i in 0..n {
                let x1 = keypoints[i].0.x;
                let y1 = keypoints[i].0.y;
                let x2 = keypoints[i].1.x;
                let y2 = keypoints[i].1.y;

                let p1 = Vector3::new(x1, y1, 1.0);
                let p2 = Vector3::new(x2, y2, 1.0);
                let error = p2.transpose() * e * p1;

                if *error.abs().get(0).unwrap() < threshold {
                    inliers.push(i);
                }
            }

            // 4. Update the best model if the current model has more inliers
            if inliers.len() > best_inliers.len() {
                best_e = Some(e);
                best_inliers = inliers;
            }
        }
    }

    // 5. Refine the essential matrix using the best inliers
    if let Some(_) = best_e {
        let kps1_inliers: Vec<KeyPoint> = best_inliers.iter().map(|&i| keypoints[i].0).collect();
        let kps2_inliers: Vec<KeyPoint> = best_inliers.iter().map(|&i| keypoints[i].1).collect();
        return compute_essential_matrix(&kps1_inliers, &kps2_inliers);
    }

    None
}

fn compute_essential_matrix(kps1: &[KeyPoint], kps2: &[KeyPoint]) -> Option<Matrix3<f32>> {
    // Assuming keypoints are normalized, i.e., intrinsic matrix is identity

    let n = kps1.len();
    if n < 8 || n != kps2.len() {
        return None;
    }

    let mut a = Matrix3::<f32>::zeros();

    for i in 0..n {
        let x1 = kps1[i].x;
        let y1 = kps1[i].y;
        let x2 = kps2[i].x;
        let y2 = kps2[i].y;

        a += Matrix3::new(x1 * x2, x1 * y2, x1, y1 * x2, y1 * y2, y1, x2, y2, 1.0);
    }

    let svd = SVD::new(a, true, true);
    let mut e = svd.u.unwrap() * svd.v_t.unwrap();

    // Enforce the rank-2 constraint on the essential matrix
    let svd_e = SVD::new(e.clone(), true, true);
    let mut s = svd_e.singular_values;
    s[2] = 0.0;
    let s_diag = Matrix3::from_diagonal(&s);
    e = svd_e.u.unwrap() * s_diag * svd_e.v_t.unwrap();

    Some(e)
}
