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
