use image::{Rgba, RgbaImage};

pub fn draw_line(
    image: &mut RgbaImage,
    start: (f32, f32),
    end: (f32, f32),
    color: Rgba<u8>,
    thickness: f32,
    hardness: f32, // 0.0 (soft) to 1.0 (hard)
    erase: bool,
) {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let distance = (dx * dx + dy * dy).sqrt();
    let steps = distance.ceil() as i32;

    for i in 0..=steps {
        let t = if steps == 0 { 0.0 } else { i as f32 / steps as f32 };
        let cx = start.0 + dx * t;
        let cy = start.1 + dy * t;

        // Draw a circle at each step to create thickness
        let radius = thickness / 2.0;
        let min_y = (cy - radius).floor() as i32;
        let max_y = (cy + radius).ceil() as i32;
        let min_x = (cx - radius).floor() as i32;
        let max_x = (cx + radius).ceil() as i32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x >= 0
                    && y >= 0
                    && (x as u32) < image.width()
                    && (y as u32) < image.height()
                {
                    let dx2 = x as f32 - cx;
                    let dy2 = y as f32 - cy;
                    let dist_sq = dx2 * dx2 + dy2 * dy2;
                    let rad_sq = radius * radius;

                    if dist_sq <= rad_sq {
                        let dist = dist_sq.sqrt();
                        // Calculate opacity falloff based on hardness
                        // Hardness 1.0 -> step function (no falloff)
                        // Hardness 0.0 -> linear falloff from center
                        let falloff = if hardness >= 0.99 {
                            1.0
                        } else {
                            let core_radius = radius * hardness;
                            if dist <= core_radius {
                                1.0
                            } else {
                                1.0 - (dist - core_radius) / (radius - core_radius)
                            }
                        };

                        let existing = image.get_pixel(x as u32, y as u32);

                        if erase {
                            let a = existing[3] as f32 / 255.0;
                            // Reduce alpha proportional to falloff
                            let new_a = a * (1.0 - falloff);

                            image.put_pixel(x as u32, y as u32, Rgba([
                                existing[0],
                                existing[1],
                                existing[2],
                                (new_a * 255.0) as u8,
                            ]));
                        } else {
                            // Blend with existing pixel
                            let alpha = (color[3] as f32 / 255.0) * falloff;
                            let inv_alpha = 1.0 - alpha;

                            let r = (color[0] as f32 * alpha + existing[0] as f32 * inv_alpha) as u8;
                            let g = (color[1] as f32 * alpha + existing[1] as f32 * inv_alpha) as u8;
                            let b = (color[2] as f32 * alpha + existing[2] as f32 * inv_alpha) as u8;

                            // Composite alpha correctly (SrcOver)
                            let current_a = existing[3] as f32 / 255.0;
                            let out_a = alpha + current_a * inv_alpha;

                            image.put_pixel(x as u32, y as u32, Rgba([r, g, b, (out_a * 255.0) as u8]));
                        }
                    }
                }
            }
        }
    }
}
