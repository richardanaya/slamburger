use crate::common::KeyPoint;

fn is_corner(p: u8, circle: &[u8], threshold: u8) -> bool {
    // In the context of image processing and computer vision, a "corner" refers to a point in the
    // image where there is a significant change in intensity or color in multiple directions.
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
