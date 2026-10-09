use image::RgbaImage;

/// Represents a single layer in the document.
pub struct Layer {
    pub name: String,
    pub data: RgbaImage,
    pub visible: bool,
    pub opacity: u8,
}

impl Layer {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Self {
            name,
            data: RgbaImage::new(width, height),
            visible: true,
            opacity: 255,
        }
    }

    /// Fills the layer with a solid color, useful for backgrounds
    pub fn fill(&mut self, color: image::Rgba<u8>) {
        for pixel in self.data.pixels_mut() {
            *pixel = color;
        }
    }

    pub fn clone_data(&self) -> Layer {
        Self {
            name: self.name.clone(),
            data: self.data.clone(),
            visible: self.visible,
            opacity: self.opacity,
        }
    }
}
