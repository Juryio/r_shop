use image::RgbaImage;

pub fn grayscale(image: &mut RgbaImage) {
    for pixel in image.pixels_mut() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        // Luminosity method
        let gray = (r * 0.299 + g * 0.587 + b * 0.114) as u8;

        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }
}

pub fn invert(image: &mut RgbaImage) {
    for pixel in image.pixels_mut() {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
        // Alpha is untouched
    }
}

pub fn gaussian_blur(image: &mut RgbaImage, sigma: f32) {
    let blurred = imageproc::filter::gaussian_blur_f32(image, sigma);
    *image = blurred;
}

pub fn brightness_contrast(image: &mut RgbaImage, brightness: i32, contrast: f32) {
    let factor = (259.0 * (contrast + 255.0)) / (255.0 * (259.0 - contrast));

    for pixel in image.pixels_mut() {
        for i in 0..3 {
            let mut v = pixel[i] as f32;
            v = factor * (v - 128.0) + 128.0;
            v += brightness as f32;
            pixel[i] = v.clamp(0.0, 255.0) as u8;
        }
    }
}

pub fn sepia(image: &mut RgbaImage) {
    for pixel in image.pixels_mut() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        let tr = (r * 0.393 + g * 0.769 + b * 0.189).min(255.0);
        let tg = (r * 0.349 + g * 0.686 + b * 0.168).min(255.0);
        let tb = (r * 0.272 + g * 0.534 + b * 0.131).min(255.0);

        pixel[0] = tr as u8;
        pixel[1] = tg as u8;
        pixel[2] = tb as u8;
    }
}
