use image::RgbaImage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    // Add more as needed
}

#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub pixels: RgbaImage,
    pub mask: Option<image::GrayImage>, // 8-bit mask
    pub blend_mode: BlendMode,
    pub opacity: f32, // 0.0 to 1.0
    pub visible: bool,
}

impl Layer {
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            name: name.into(),
            pixels: RgbaImage::new(width, height),
            mask: None,
            blend_mode: BlendMode::Normal,
            opacity: 1.0,
            visible: true,
        }
    }

    pub fn from_image(name: impl Into<String>, image: RgbaImage) -> Self {
        Self {
            name: name.into(),
            pixels: image,
            mask: None,
            blend_mode: BlendMode::Normal,
            opacity: 1.0,
            visible: true,
        }
    }
}
