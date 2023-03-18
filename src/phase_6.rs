use nalgebra::linalg::SVD;
use nalgebra::{
    DMatrix, DMatrixSlice, DMatrixView, DVector, DVectorSlice, Dyn, Dynamic, Matrix3, Matrix4,
    Point2, Point3, RowVector, Vector3, Vector6, U12, U3,
};
use std::ops::AddAssign;

/// Build a camera intrinsic matrix.
///
/// # Arguments
///
/// * `f_x` - The focal length in the x direction (in pixels)
/// * `f_y` - The focal length in the y direction (in pixels)
/// * `c_x` - The x-coordinate of the optical center or principal point of the camera (in pixels)
/// * `c_y` - The y-coordinate of the optical center or principal point of the camera (in pixels)
///
/// # Example
///
/// ```
/// let f_x = 800.0;
/// let f_y = 800.0;
/// let c_x = 320.0;
/// let c_y = 240.0;
///
/// let intrinsic_matrix = build_intrinsic_matrix(f_x, f_y, c_x, c_y);
/// println!("Intrinsic matrix: {:?}", intrinsic_matrix);
/// ```
pub fn build_intrinsic_matrix(f_x: f64, f_y: f64, c_x: f64, c_y: f64) -> Matrix3<f64> {
    Matrix3::new(f_x, 0.0, c_x, 0.0, f_y, c_y, 0.0, 0.0, 1.0)
}

// This function estimates the 3D camera pose (rotation and translation) given a set of 2D keypoints from an image,
// their corresponding 3D keypoints in the real world, and the camera's intrinsic matrix.
// It returns the estimated camera pose (rotation and translation) if successful.

pub fn perspective_n_point(
    keypoints_2d: &[Point2<f64>],
    keypoints_3d: &[Point3<f64>],
    intrinsic_matrix: Matrix3<f64>,
) -> Option<(Matrix3<f64>, Vector3<f64>)> {
    // Check if there are enough keypoints for the estimation
    if keypoints_2d.len() < 6 || keypoints_3d.len() < 6 {
        return None;
    }

    // Normalize the 2D keypoints using the inverse of the intrinsic matrix
    let intrinsic_matrix_inv = intrinsic_matrix.try_inverse().unwrap();

    let keypoints_2d_normalized: Vec<Point2<f64>> = keypoints_2d
        .iter()
        .map(|keypoint| intrinsic_matrix_inv * keypoint.coords.to_homogeneous())
        .map(|coords| Point2::new(coords.x / coords.z, coords.y / coords.z))
        .collect();

    // Create the design matrix for the DLT algorithm
    let mut design_matrix = DMatrix::zeros(2 * keypoints_2d.len(), 12);

    // Populate the design matrix using the normalized 2D keypoints and 3D keypoints
    for (i, (keypoint_2d, keypoint_3d)) in keypoints_2d_normalized
        .iter()
        .zip(keypoints_3d.iter())
        .enumerate()
    {
        let row1 = [
            0.0,
            0.0,
            0.0,
            0.0,
            -keypoint_3d.x,
            -keypoint_3d.y,
            -keypoint_3d.z,
            -1.0,
            keypoint_2d.y * keypoint_3d.x,
            keypoint_2d.y * keypoint_3d.y,
            keypoint_2d.y * keypoint_3d.z,
            keypoint_2d.y,
        ];
        let row2 = [
            keypoint_3d.x,
            keypoint_3d.y,
            keypoint_3d.z,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            -keypoint_2d.x * keypoint_3d.x,
            -keypoint_2d.x * keypoint_3d.y,
            -keypoint_2d.x * keypoint_3d.z,
            -keypoint_2d.x,
        ];

        // create 12 column 1 row matrix
        let row1 = DMatrix::from_row_slice(1, 12, &row1);
        let row2 = DMatrix::from_row_slice(1, 12, &row2);

        // insert into design matrix
        design_matrix
            .view_mut((2 * i, 0), (2, 12))
            .add_assign(&row1);
        design_matrix
            .view_mut(((2 * i) + 1, 0), (2, 12))
            .add_assign(&row2);
    }

    // Compute the SVD of the design matrix
    let svd = design_matrix.try_svd(true, true, 1e-7, 100).unwrap();

    // Get the last column of the V^T matrix (last row of V)
    let h = svd.v_t.unwrap().column(11).into_owned();

    // Reshape the last column of the V^T matrix into a 3x4 projection matrix
    let projection_matrix = Matrix4::from_iterator(h.iter().cloned())
        .into_owned()
        .transpose();

    // Extract the rotation and translation from the projection matrix
    let rotation = projection_matrix.fixed_view::<3, 3>(0, 0).into_owned();
    let translation = projection_matrix.fixed_view::<3, 1>(0, 3).into_owned();

    // Return the rotation and translation
    Some((rotation, translation))
}

// This function refines the camera pose (rotation and translation) using the Levenberg-Marquardt algorithm
// to minimize the difference between the actual 2D keypoints and the projected 2D keypoints.
// It takes the normalized 2D keypoints, the corresponding 3D keypoints, and initial estimates for
// the camera rotation and translation as inputs, and returns the refined rotation and translation.
fn refine_pose_levenberg_marquardt(
    keypoints_2d_normalized: &[Point2<f64>],
    keypoints_3d: &[Point3<f64>],
    initial_rotation: &Matrix3<f64>,
    initial_translation: &Vector3<f64>,
) -> (Matrix3<f64>, Vector3<f64>) {
    // Set the maximum number of iterations for the optimization
    let max_iterations = 100;

    // Set the termination threshold for the optimization
    let termination_threshold = 1e-8;

    // Initialize the camera rotation and translation with the initial estimates
    let mut rotation = initial_rotation.clone();
    let mut translation = initial_translation.clone();

    /*  // Initialize the damping factor for the Levenberg-Marquardt algorithm
    let mut damping_factor = 1e-4;

    // Iterate through the optimization process
    for _ in 0..max_iterations {
        // Compute the Jacobian matrix and the error vector using the current camera pose
        let (jacobian, error_vector) = compute_jacobian_and_error_vector(
            keypoints_2d_normalized,
            keypoints_3d,
            &rotation,
            &translation,
        );

        // Calculate the normal equation components (left-hand side and right-hand side)
        let normal_equation_lhs = jacobian.transpose() * &jacobian;
        let normal_equation_rhs = jacobian.transpose() * error_vector;

        // Update the damping factor and normal equation left-hand side
        let identity = Matrix6::<f64>::identity();
        let lhs_with_damping: Matrix6<f64> = normal_equation_lhs + damping_factor * identity;

        // Solve the normal equation to get the pose increment (delta_rotation, delta_translation)
        let pose_increment = lhs_with_damping.solve(&normal_equation_rhs).unwrap();

        // Update the camera pose by adding the pose increment
        let angle = pose_increment.fixed_rows::<3>(0).norm();
        let axis = nalgebra::Unit::new_normalize(Vector3::from_iterator(
            pose_increment.fixed_rows::<3>(0).iter().cloned(),
        ));
        let delta_rotation = Matrix3::from_scaled_axis(axis.into_inner() * angle);
        let delta_translation = pose_increment.fixed_rows::<3>(3).into_owned();

        // Apply the updates to the camera pose
        rotation = delta_rotation * rotation;
        translation += delta_translation;

        // Check if the pose increment is smaller than the termination threshold
        if pose_increment.norm() < termination_threshold {
            break;
        }
    }
    */

    // Return the refined camera pose (rotation and translation)
    (rotation, translation)
}

/*At a high level, this function calculates the difference between the
actual 2D keypoints detected in the image and the projected 2D keypoints
obtained from their corresponding 3D keypoints in the real world,
given a specific camera rotation and translation. The differences, also
known as reprojection errors, are stored in an error vector which the function
returns. This helps to understand how well the current camera pose matches
the observed 2D keypoints.

 */
// Input:
// - keypoints_2d_normalized: &[Point2<f64>] - normalized 2D keypoints
// - keypoints_3d: &[Point3<f64>] - corresponding 3D keypoints
// - rotation: &Matrix3<f64> - rotation matrix
// - translation: &Vector3<f64> - translation vector
// Output:
// - Vector<f64> - reprojection error for each keypoint
fn compute_reprojection_error(
    keypoints_2d_normalized: &[Point2<f64>],
    keypoints_3d: &[Point3<f64>],
    rotation: &Matrix3<f64>,
    translation: &Vector3<f64>,
) -> DVector<f64> {
    let num_points = keypoints_2d_normalized.len();
    let mut error = DVector::zeros(num_points * 2);
    for i in 0..num_points {
        let projected_point = rotation * keypoints_3d[i].coords + translation;
        let projected_point_normalized = Point2::new(
            projected_point.x / projected_point.z,
            projected_point.y / projected_point.z,
        );

        error[2 * i] = keypoints_2d_normalized[i].x - projected_point_normalized.x;
        error[2 * i + 1] = keypoints_2d_normalized[i].y - projected_point_normalized.y;
    }

    error
}

/*
   This function takes the 2D keypoints from an image, their
   corresponding 3D keypoints in the real world, a rotation matrix,
   and a translation vector as inputs. It calculates how sensitive the
   projected 2D keypoints are to changes in the 3D camera pose (rotation
    and translation). This sensitivity information is stored in a matrix called
    the Jacobian matrix. The function also calculates the difference between the actual 2D
    keypoints and the projected 2D keypoints based on the current 3D camera pose. This
    difference is stored in an error vector. The function returns both the
    Jacobian matrix and the error vector.
*/

// Compute the Jacobian matrix and the error vector for the given pose estimate
// Input:
// - keypoints_2d_normalized: &[Point2<f64>] - normalized 2D keypoints
// - keypoints_3d: &[Point3<f64>] - corresponding 3D keypoints
// - R: &Matrix3<f64> - rotation matrix
// - t: &Vector3<f64> - translation vector
// Output:
// - (DMatrix<f64>, DVector<f64>) - Jacobian matrix and error vector
fn compute_jacobian_and_error_vector(
    keypoints_2d_normalized: &[Point2<f64>],
    keypoints_3d: &[Point3<f64>],
    rotation: &Matrix3<f64>,
    translation: &Vector3<f64>,
) -> (DMatrix<f64>, DVector<f64>) {
    // Get the number of keypoints
    let num_points = keypoints_2d_normalized.len();

    // Initialize the Jacobian matrix with the appropriate size (num_points * 2 rows, 6 columns)
    let mut jacobian = DMatrix::zeros(num_points * 2, 6);

    // Initialize the error vector with the appropriate size (num_points * 2 elements)
    let mut error_vector = DVector::zeros(num_points * 2);

    // Iterate through all keypoints
    for i in 0..num_points {
        // Get the 3D keypoint coordinates
        let keypoint_coordinates = keypoints_3d[i].coords;

        // Rotate the 3D keypoint using the rotation matrix
        let rotated_keypoint_coordinates = rotation * keypoint_coordinates;

        // Add the translation vector to get the projected point in 3D space
        let projected_point = rotated_keypoint_coordinates + translation;

        // Calculate the normalized image coordinates (u, v) by dividing the x and y coordinates by the z coordinate
        let u = projected_point.x / projected_point.z;
        let v = projected_point.y / projected_point.z;

        // Compute the elements of the first row of the Jacobian matrix for this keypoint
        let jacobian_row1 = Vector6::new(
            -1.0 / projected_point.z, // derivative with respect to x-axis translation
            0.0,                      // derivative with respect to y-axis translation
            u / projected_point.z,    // derivative with respect to z-axis translation
            u * v,                    // derivative with respect to rotation around the x-axis
            -(1.0 + u * u),           // derivative with respect to rotation around the y-axis
            v,                        // derivative with respect to rotation around the z-axis
        );

        // Compute the elements of the second row of the Jacobian matrix for this keypoint
        let jacobian_row2 = Vector6::new(
            0.0,                      // derivative with respect to x-axis translation
            -1.0 / projected_point.z, // derivative with respect to y-axis translation
            v / projected_point.z,    // derivative with respect to z-axis translation
            1.0 + v * v,              // derivative with respect to rotation around the x-axis
            -u * v,                   // derivative with respect to rotation around the y-axis
            -u,                       // derivative with respect to rotation around the z-axis
        );
        // Copy the first row of the Jacobian matrix for this keypoint
        jacobian
            .rows_mut(2 * i, 1)
            .copy_from(&jacobian_row1.transpose());

        // Copy the second row of the Jacobian matrix for this keypoint
        jacobian
            .rows_mut(2 * i + 1, 1)
            .copy_from(&jacobian_row2.transpose());

        // Calculate the error between the normalized 2D keypoint and the projected point in the image (u and v coordinates)
        let error_u = keypoints_2d_normalized[i].x - u;
        let error_v = keypoints_2d_normalized[i].y - v;

        // Add the errors to the error vector
        error_vector[2 * i] = error_u;
        error_vector[2 * i + 1] = error_v;
    }

    // Return the Jacobian matrix and error vector
    (jacobian, error_vector)
}
