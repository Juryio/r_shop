use image::{Rgba, RgbaImage};

pub fn flood_fill(image: &mut RgbaImage, start_x: u32, start_y: u32, fill_color: Rgba<u8>) {
    if start_x >= image.width() || start_y >= image.height() {
        return;
    }

    let target_color = *image.get_pixel(start_x, start_y);
    if target_color == fill_color {
        return;
    }

    let mut stack = vec![(start_x, start_y)];

    while let Some((x, y)) = stack.pop() {
        if *image.get_pixel(x, y) == target_color {
            image.put_pixel(x, y, fill_color);

            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < image.width() - 1 {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < image.height() - 1 {
                stack.push((x, y + 1));
            }
        }
    }
}
