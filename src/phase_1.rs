pub fn rgb_to_grayscale(img: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut greyscale_image = vec![0u8; width * height];
    for y in 0..height {
        for x in 0..width {
            let idx = 4 * (y * width + x);
            let r = img[idx] as f32;
            let g = img[idx + 1] as f32;
            let b = img[idx + 2] as f32;
            // these look like magic numbers, but there is some logic behind them
            // https://en.wikipedia.org/wiki/Grayscale#Converting_color_to_grayscale
            // check out luma coding
            let g = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            greyscale_image[y * width + x] = g;
        }
    }
    greyscale_image
}

use std::f32::consts::PI;

pub fn greyscale_gaussian_blur(
    img: &[u8],
    width: usize,
    height: usize,
    blur_radius: f32,
) -> Vec<u8> {
    let kernel_size = (blur_radius * 2.0).ceil() as usize | 1;
    let half_kernel = (kernel_size / 2) as i32;

    let mut kernel = vec![0f32; kernel_size];
    let mut kernel_sum = 0f32;

    for i in 0..kernel_size {
        let x = i as f32 - half_kernel as f32;
        let value =
            (-x * x / (2.0 * blur_radius * blur_radius)).exp() / (blur_radius * (2.0 * PI).sqrt());
        kernel[i] = value;
        kernel_sum += value;
    }

    // Normalize the kernel
    for i in 0..kernel_size {
        kernel[i] /= kernel_sum;
    }

    let mut output = vec![0u8; img.len()];
    let mut buffer = vec![0f32; width * height];

    // Perform horizontal blur
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            for i in 0..kernel_size {
                let index = (x as i32 - half_kernel + i as i32).clamp(0, width as i32 - 1) as usize
                    + y * width;
                sum += kernel[i] * img[index] as f32;
            }
            buffer[x + y * width] = sum;
        }
    }

    // Perform vertical blur
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            for i in 0..kernel_size {
                let index = x
                    + (y as i32 - half_kernel + i as i32).clamp(0, height as i32 - 1) as usize
                        * width;
                sum += kernel[i] * buffer[index];
            }
            output[x + y * width] = sum.round() as u8;
        }
    }

    output
}

/****************/
/*  UNIT TESTS  */
/****************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_grayscale() {
        let input_image: [u8; 12] = [
            255, 0, 0, 255, // Red
            0, 255, 0, 255, // Green
            0, 0, 255, 255, // Blue
        ];

        let expected_output: Vec<u8> = vec![
            76,  // Red
            149, // Green
            29,  // Blue
        ];

        let output = rgb_to_grayscale(&input_image, 3, 1);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_rgb_to_grayscale_empty_image() {
        let input_image: [u8; 0] = [];
        let expected_output: Vec<u8> = vec![];
        let output = rgb_to_grayscale(&input_image, 0, 0);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_greyscale_gaussian_blur() {
        let input_image: [u8; 15] = [
            200, 200, 200, 200, 200, // Row 1
            100, 100, 100, 100, 100, // Row 2
            50, 50, 50, 50, 50, // Row 3
        ];

        let expected_output: Vec<u8> = vec![
            166, 166, 166, 166, 166, 115, 115, 115, 115, 115, 71, 71, 71, 71, 71,
        ];

        let output = greyscale_gaussian_blur(&input_image, 5, 3);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_greyscale_gaussian_blur_empty_image() {
        let input_image: [u8; 0] = [];
        let expected_output: Vec<u8> = vec![];
        let output = greyscale_gaussian_blur(&input_image, 0, 0);
        assert_eq!(output, expected_output);
    }
    #[test]
    fn test_rgb_to_grayscale_black_and_white() {
        let input_image: [u8; 8] = [
            0, 0, 0, 255, // Black
            255, 255, 255, 255, // White
        ];

        let expected_output: Vec<u8> = vec![
            0,   // Black
            255, // White
        ];

        let output = rgb_to_grayscale(&input_image, 2, 1);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_rgb_to_grayscale_multiple_rows() {
        let input_image: [u8; 24] = [
            255, 0, 0, 255, // Red
            0, 255, 0, 255, // Green
            255, 255, 0, 255, // Yellow
            0, 0, 255, 255, // Blue
            255, 0, 255, 255, // Magenta
            0, 255, 255, 255, // Cyan
        ];

        let expected_output: Vec<u8> = vec![76, 149, 225, 29, 105, 178];

        let output = rgb_to_grayscale(&input_image, 3, 2);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_greyscale_gaussian_blur_single_pixel() {
        let input_image: [u8; 1] = [128];
        let expected_output: Vec<u8> = vec![128];
        let output = greyscale_gaussian_blur(&input_image, 1, 1);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_greyscale_gaussian_blur_uniform_image() {
        let input_image: [u8; 9] = [128, 128, 128, 128, 128, 128, 128, 128, 128];

        let expected_output: Vec<u8> = vec![128, 128, 128, 128, 128, 128, 128, 128, 128];

        let output = greyscale_gaussian_blur(&input_image, 3, 3);
        assert_eq!(output, expected_output);
    }
}
