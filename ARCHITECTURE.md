# r_shop Architecture

## Foundational Architecture

The application is structured into the following main components:
- `core`: State management, data structures, and file I/O (Exclusive domain of this module).
- `ui`: Graphical user interface components (eframe/egui).
- `filters`: Image processing filters.

### Core Domain (`src/core/`)

The core module is responsible for the foundational data models and state management of the application. It acts as the single source of truth for the workspace.

*   **Workspace**: Manages multiple documents and tracks the currently active document.
*   **Document**: Represents a single image being edited. Contains dimensions, resolution, color space information, and a stack of layers.
*   **Layer**: Represents a single image layer. It contains pixel data (buffer), masks, blend modes, opacity, and visibility status.
*   **History**: Implements an Undo/Redo stack using the Command Pattern to manage state changes safely and reversibly.
*   **IO**: Handles reading from and writing to standard image formats.

---
*(Appended structural definitions will follow below during implementation)*

### Core Structural Definitions

#### `Layer`
```rust
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
}

pub struct Layer {
    pub name: String,
    pub pixels: image::RgbaImage,
    pub mask: Option<image::GrayImage>,
    pub blend_mode: BlendMode,
    pub opacity: f32,
    pub visible: bool,
}
```

#### `Document`
```rust
pub enum ColorSpace {
    Srgb,
    Linear,
}

pub struct Document {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub resolution: f32, // ppi
    pub color_space: ColorSpace,
    pub layers: Vec<Layer>,
}
```

#### `History`
```rust
pub trait Command {
    fn execute(&mut self, document: &mut Document);
    fn undo(&mut self, document: &mut Document);
}

pub struct History {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}
```

#### `Workspace`
```rust
pub struct Workspace {
    pub documents: Vec<Document>,
    pub active_document_index: Option<usize>,
}
```

#### `IO`
```rust
pub fn open_image<P: AsRef<std::path::Path>>(path: P) -> Result<Document, image::ImageError>;
pub fn save_image<P: AsRef<std::path::Path>>(document: &Document, path: P) -> Result<(), image::ImageError>;
```
