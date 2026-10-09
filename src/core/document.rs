use crate::core::layer::Layer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    Srgb,
    Linear,
    // Add more as needed
}

#[derive(Clone)]
pub struct Document {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub resolution: f32, // ppi
    pub color_space: ColorSpace,
    pub layers: Vec<Layer>,
}

impl Document {
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            name: name.into(),
            width,
            height,
            resolution: 72.0,
            color_space: ColorSpace::Srgb,
            layers: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }
}
