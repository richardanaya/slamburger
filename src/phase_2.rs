use crate::common::KeyPoint;

type SpiralPatternPositions = [(isize, isize); 12];
type SpiralIntensity = [u8; 12];

fn is_corner_in_spiral(
    center_point_intensity: u8,
    circle: &SpiralIntensity,
    threshold_for_intensity_difference: u8,
    needed_consecutive_intensity_differences: u8,
) -> bool {
    // In the context of image processing and computer vision, a "corner" refers to a point in the
    // image where there is a significant change in intensity or color in multiple directions.

    let mut count = 0;
    let mut consecutive = 0;

    for &pixel in circle.iter() {
        let is_signifigant_intensity_difference = (pixel as i32 - center_point_intensity as i32)
            .abs()
            > threshold_for_intensity_difference as i32;
        if is_signifigant_intensity_difference {
            consecutive += 1;
            if consecutive == needed_consecutive_intensity_differences {
                println!("hey");
                return true;
            }
        } else {
            count += consecutive;
            consecutive = 0;
        }
    }

    // we accomulate consequent pixels in the circle even if seperated, if we have enough, we have a corner
    count + consecutive >= needed_consecutive_intensity_differences
}

// The offsets of the pixels in a circlar spiral around a pixel.
const SPIRAL_PATTERN: SpiralPatternPositions = [
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

// This function takes an image, its dimensions, and a threshold as input and returns a list of
// keypoints (corners) in the image. It iterates over the image and for each pixel, it checks if
// the pixel is a corner by comparing it to the pixels in a circle around it. If the pixel is a
// corner, it is added to the list of keypoints.
pub fn fast_keypoints(
    img: &[u8],
    width: usize,
    height: usize,
    threshold: u8,
) -> Vec<(usize, usize)> {
    let mut keypoints = Vec::new();
    println!("iterating from 3 to {}", height as isize - 3);
    println!("iterating from 3 to {}", width as isize - 3);
    for y in 3..(height as isize - 3) {
        for x in 3..(width as isize - 3) {
            println!("{} {} ", x, y);
            // get the intensity at x y
            let intensity = img[(y * width as isize + x) as usize];

            // get the surrounding spiral intensities
            let spiral_intensities: SpiralIntensity = [
                img[(y + SPIRAL_PATTERN[0].1) as usize * width
                    + (x + SPIRAL_PATTERN[0].0) as usize],
                img[(y + SPIRAL_PATTERN[1].1) as usize * width
                    + (x + SPIRAL_PATTERN[1].0) as usize],
                img[(y + SPIRAL_PATTERN[2].1) as usize * width
                    + (x + SPIRAL_PATTERN[2].0) as usize],
                img[(y + SPIRAL_PATTERN[3].1) as usize * width
                    + (x + SPIRAL_PATTERN[3].0) as usize],
                img[(y + SPIRAL_PATTERN[4].1) as usize * width
                    + (x + SPIRAL_PATTERN[4].0) as usize],
                img[(y + SPIRAL_PATTERN[5].1) as usize * width
                    + (x + SPIRAL_PATTERN[5].0) as usize],
                img[(y + SPIRAL_PATTERN[6].1) as usize * width
                    + (x + SPIRAL_PATTERN[6].0) as usize],
                img[(y + SPIRAL_PATTERN[7].1) as usize * width
                    + (x + SPIRAL_PATTERN[7].0) as usize],
                img[(y + SPIRAL_PATTERN[8].1) as usize * width
                    + (x + SPIRAL_PATTERN[8].0) as usize],
                img[(y + SPIRAL_PATTERN[9].1) as usize * width
                    + (x + SPIRAL_PATTERN[9].0) as usize],
                img[(y + SPIRAL_PATTERN[10].1) as usize * width
                    + (x + SPIRAL_PATTERN[10].0) as usize],
                img[(y + SPIRAL_PATTERN[11].1) as usize * width
                    + (x + SPIRAL_PATTERN[11].0) as usize],
            ];

            if is_corner_in_spiral(intensity, &spiral_intensities, threshold, 9) {
                keypoints.push((x as usize, y as usize));
            }
        }
    }
    keypoints
}

/* This function takes an image, its dimensions, and a list
of keypoints (corners) as input and computes the dominant
orientation of the gradient for each keypoint. It calculates
the weighted sum of the gradients in a circle around each keypoint
using the pixel values and the circle offsets. The final output is a
vector of KeyPoint structures, each containing the x, y coordinates and
orientation of the gradient for the corresponding keypoint. */
pub fn compute_orientations(
    img: &[u8],
    width: usize,
    keypoints: &[(usize, usize)],
) -> Vec<KeyPoint> {
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

/****************/
/*  UNIT TESTS  */
/****************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_corner_in_spiral() {
        // Test with a simple spiral pattern where the center pixel is a corner
        let circle: SpiralIntensity = [10, 10, 200, 10, 10, 10, 10, 10, 10, 10, 10, 10];
        let center_point_intensity = 200;
        let threshold_for_intensity_difference = 50;
        let needed_consecutive_intensity_differences = 9;
        let is_corner = is_corner_in_spiral(
            center_point_intensity,
            &circle,
            threshold_for_intensity_difference,
            needed_consecutive_intensity_differences,
        );
        assert_eq!(is_corner, true);

        // Test with a simple spiral pattern where the center pixel is not a corner
        let circle: SpiralIntensity = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];
        let center_point_intensity = 10;
        let threshold_for_intensity_difference = 50;
        let needed_consecutive_intensity_differences = 9;
        let is_corner = is_corner_in_spiral(
            center_point_intensity,
            &circle,
            threshold_for_intensity_difference,
            needed_consecutive_intensity_differences,
        );
        assert_eq!(is_corner, false);

        // Test with a more complex spiral pattern where the center pixel is a corner
        let circle: SpiralIntensity = [10, 10, 100, 10, 10, 10, 200, 10, 10, 10, 10, 10];
        let center_point_intensity = 200;
        let threshold_for_intensity_difference = 50;
        let needed_consecutive_intensity_differences = 9;
        let is_corner = is_corner_in_spiral(
            center_point_intensity,
            &circle,
            threshold_for_intensity_difference,
            needed_consecutive_intensity_differences,
        );
        assert_eq!(is_corner, true);

        println!("start");

        // Test with a more complex spiral pattern where the center pixel is a corner
        // notice we have multiple consecutive intensity differences that are greater than the threshold
        let circle: SpiralIntensity = [10, 10, 100, 10, 10, 10, 100, 10, 10, 10, 10, 10];
        let center_point_intensity = 100;
        let threshold_for_intensity_difference = 50;
        let needed_consecutive_intensity_differences = 9;
        let is_corner = is_corner_in_spiral(
            center_point_intensity,
            &circle,
            threshold_for_intensity_difference,
            needed_consecutive_intensity_differences,
        );
        assert_eq!(is_corner, true);
    }

    #[test]
    fn test_fast_keypoints() {
        // Test with a simple 5x5 image where the corner pixel has a high intensity value
        // this grid is too small to test spirals so we should return nothing
        let img = [
            10, 10, 10, 10, 10, //
            10, 200, 10, 10, 10, //
            10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, //
        ];
        let width = 5;
        let height = 5;
        let threshold = 50;
        let keypoints = fast_keypoints(&img, width, height, threshold);
        assert_eq!(keypoints, vec![]);

        println!("start");

        // Test with a simple 9x9 image where there are no corners
        let img = [
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 200, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
        ];
        let threshold = 50;
        let keypoints = fast_keypoints(&img, 9, 9, threshold);
        assert_eq!(keypoints, vec![(4, 4)]);

        // Test with a larger 9x9 image where there are two corners
        let img = [
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 200, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 0, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 200, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
            10, 10, 10, 10, 10, 10, 10, 10, 10, //
        ];
        let threshold = 50;
        let keypoints = fast_keypoints(&img, 9, 9, threshold);
        assert_eq!(keypoints, vec![(3, 3), (5, 5)]);
    }
}
