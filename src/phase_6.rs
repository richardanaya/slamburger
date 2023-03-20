use nalgebra::{Matrix3, Vector3};

pub fn decompose_essential_matrix(essential: Matrix3<f64>) -> (Matrix3<f64>, Vector3<f64>) {
    // Compute the singular value decomposition of the essential matrix
    let svd = essential.svd(true, true);

    // Extract the singular values and vectors
    let u = svd.u.unwrap();
    let v = svd.v_t.unwrap();
    let s = svd.singular_values;

    // Compute the rotation matrix
    let w = Matrix3::new(0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let rotation = u * w * v.transpose();

    // Compute the translation vector
    let translation: Vector3<f64> = u.column(2).into();

    (rotation, translation)
}
