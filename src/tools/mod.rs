pub mod brush;
pub mod bucket;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Brush,
    Fill,
}
