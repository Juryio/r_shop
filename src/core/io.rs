use std::path::Path;
use image::{ImageError, GenericImageView};
use crate::core::document::Document;
use crate::core::layer::Layer;

pub fn open_image<P: AsRef<Path>>(path: P) -> Result<Document, ImageError> {
    let img = image::open(&path)?;
    let (width, height) = img.dimensions();
    let name = path
        .as_ref()
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unnamed")
        .to_string();

    let mut doc = Document::new(name.clone(), width, height);

    // Convert to RGBA as the standard working format
    let rgba_image = img.into_rgba8();

    let layer = Layer::from_image("Background", rgba_image);
    doc.add_layer(layer);

    Ok(doc)
}

pub fn save_image<P: AsRef<Path>>(document: &Document, path: P) -> Result<(), ImageError> {
    // For now, save the first layer.
    // In a full implementation, we'd composite all visible layers first.
    if let Some(layer) = document.layers.first() {
        layer.pixels.save(path)?;
    }

    Ok(())
}
