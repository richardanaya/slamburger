pub fn rgb_to_grayscale<'a>(img: &'a mut [u8], width: usize, height: usize) -> &'a [u8] {
    for y in 0..height {
        for x in 0..width {
            let idx = 4 * (y * width + x);
            let r = img[idx] as f32;
            let g = img[idx + 1] as f32;
            let b = img[idx + 2] as f32;
            let g = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            img[y * width + x] = g;
        }
    }
    &img[0..(width * height)]
}

pub fn greyscale_gaussian_blur(img: &[u8], width: usize, height: usize) -> Vec<u8> {
    const KERNEL_SIZE: usize = 5;
    const KERNEL: [f32; KERNEL_SIZE] = [0.06136, 0.24477, 0.38774, 0.24477, 0.06136];

    let mut output = vec![0u8; img.len()];
    let mut buffer = vec![0f32; width * height];

    // Perform horizontal blur
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            for i in 0..KERNEL_SIZE {
                let index =
                    (x as i32 - 2 + i as i32).clamp(0, width as i32 - 1) as usize + y * width;
                sum += KERNEL[i] * img[index] as f32;
            }
            buffer[x + y * width] = sum;
        }
    }

    // Perform vertical blur
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            for i in 0..KERNEL_SIZE {
                let index =
                    x + (y as i32 - 2 + i as i32).clamp(0, height as i32 - 1) as usize * width;
                sum += KERNEL[i] * buffer[index];
            }
            output[x + y * width] = sum.round() as u8;
        }
    }

    output
}
