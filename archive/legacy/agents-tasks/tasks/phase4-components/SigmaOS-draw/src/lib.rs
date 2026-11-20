//! SigmaOS vector and raster drawing application.
//!
//! Provides layers, shape tools, and export to standard image formats.

#![warn(missing_docs)]

/// A drawing layer.
#[derive(Debug, Clone)]
pub struct Layer {
    /// Name of the layer.
    pub name: String,
    /// Whether the layer is visible.
    pub visible: bool,
    /// Opacity from 0.0 to 1.0.
    pub opacity: f32,
}

/// A shape that can be drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// Rectangle.
    Rectangle,
    /// Ellipse.
    Ellipse,
    /// Freehand path.
    Path,
    /// Text.
    Text,
}

/// A document being edited.
#[derive(Debug, Clone)]
pub struct Document {
    /// Width of the canvas in pixels.
    pub width: u32,
    /// Height of the canvas in pixels.
    pub height: u32,
    /// Ordered layers in the document.
    pub layers: Vec<Layer>,
}

/// Trait for export backends.
pub trait Exporter {
    /// Export the document to a file path.
    fn export(&self, doc: &Document, path: &str) -> Result<(), DrawError>;
}

/// Errors returned by the drawing application.
#[derive(Debug)]
pub enum DrawError {
    /// The export format is unsupported.
    UnsupportedFormat(String),
    /// The file could not be written.
    WriteFailed(String),
}

impl std::fmt::Display for DrawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawError::UnsupportedFormat(fmt) => write!(f, "unsupported format: {fmt}"),
            DrawError::WriteFailed(msg) => write!(f, "write failed: {msg}"),
        }
    }
}

impl std::error::Error for DrawError {}
