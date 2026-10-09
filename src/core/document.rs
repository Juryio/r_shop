use crate::core::layer::{BlendMode, Layer};
use image::{Rgba, RgbaImage};

/// The main document holding the layers and managing canvas size.
pub struct Document {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
    pub active_layer_index: usize,
    // Store only the modified layer instead of the entire document
    // (layer_index, layer_state)
    undo_stack: Vec<(usize, Layer)>,
    redo_stack: Vec<(usize, Layer)>,
    pub is_dirty: bool,
}

impl Document {
    pub fn new(width: u32, height: u32) -> Self {
        let mut bg_layer = Layer::new("Background".to_string(), width, height);
        bg_layer.fill(Rgba([255, 255, 255, 255])); // White background by default

        Self {
            width,
            height,
            layers: vec![bg_layer],
            active_layer_index: 0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            is_dirty: true,
        }
    }

    pub fn save_state(&mut self) {
        if self.active_layer_index < self.layers.len() {
            let layer_state = self.layers[self.active_layer_index].clone_data();
            self.undo_stack.push((self.active_layer_index, layer_state));
            self.redo_stack.clear(); // Clear redo stack on new action
            self.is_dirty = true;
        }
    }

    pub fn undo(&mut self) {
        if let Some((idx, state)) = self.undo_stack.pop() {
            if idx < self.layers.len() {
                let current_state = self.layers[idx].clone_data();
                self.redo_stack.push((idx, current_state));
                self.layers[idx] = state;
                self.is_dirty = true;
            }
        }
    }

    pub fn redo(&mut self) {
        if let Some((idx, state)) = self.redo_stack.pop() {
            if idx < self.layers.len() {
                let current_state = self.layers[idx].clone_data();
                self.undo_stack.push((idx, current_state));
                self.layers[idx] = state;
                self.is_dirty = true;
            }
        }
    }

    pub fn add_layer(&mut self, name: String) {
        self.is_dirty = true;
        let new_layer = Layer::new(name, self.width, self.height);
        self.layers.push(new_layer);
        self.active_layer_index = self.layers.len() - 1;
    }

    pub fn get_active_layer_mut(&mut self) -> Option<&mut Layer> {
        self.is_dirty = true;
        self.layers.get_mut(self.active_layer_index)
    }

    /// Renders all visible layers into a single flattened image.
    /// Uses basic alpha blending.
    pub fn render(&self) -> RgbaImage {
        let mut result = RgbaImage::new(self.width, self.height);

        for layer in &self.layers {
            if !layer.visible {
                continue;
            }

            let opacity_f32 = layer.opacity as f32 / 255.0;

            for (x, y, pixel) in layer.data.enumerate_pixels() {
                if x < self.width && y < self.height {
                    let dst = result.get_pixel_mut(x, y);
                    let src = pixel;

                    let src_a = (src[3] as f32 / 255.0) * opacity_f32;
                    let dst_a = dst[3] as f32 / 255.0;

                    let out_a = src_a + dst_a * (1.0 - src_a);
                    if out_a > 0.0 {
                        for c in 0..3 {
                            let src_c = src[c] as f32 / 255.0;
                            let dst_c = dst[c] as f32 / 255.0;

                            let blended_c = match layer.blend_mode {
                                BlendMode::Normal => src_c,
                                BlendMode::Multiply => src_c * dst_c,
                                BlendMode::Screen => 1.0 - (1.0 - src_c) * (1.0 - dst_c),
                            };

                            let out_c = (blended_c * src_a + dst_c * dst_a * (1.0 - src_a)) / out_a;
                            dst[c] = (out_c * 255.0) as u8;
                        }
                        dst[3] = (out_a * 255.0) as u8;
                    }
                }
            }
        }

        result
    }
}
