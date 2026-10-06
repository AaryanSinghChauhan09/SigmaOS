// PhotoFlare Image Editor
// Linux Mint PhotoFlare-inspired image editor with drawing tools, filters, and batch processing

use std::collections::HashMap;
use std::path::PathBuf;

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Tiff,
    Bmp,
    Gif,
    WebP,
    Tga,
    Ico,
    Xpm,
}

/// Drawing tool type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawingTool {
    Brush,
    Pencil,
    Eraser,
    Fill,
    Gradient,
    Line,
    Rectangle,
    Ellipse,
    Text,
    Selection,
    Move,
    Crop,
    ColorPicker,
}

/// Filter type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    Blur,
    Sharpen,
    Emboss,
    EdgeDetect,
    Grayscale,
    Sepia,
    Invert,
    Brightness,
    Contrast,
    Saturation,
    HueRotate,
    Noise,
    Pixelate,
    OilPaint,
    Vignette,
}

/// Adjustment type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjustmentType {
    Brightness,
    Contrast,
    Saturation,
    Hue,
    Lightness,
    Gamma,
    Exposure,
    Shadows,
    Highlights,
    Temperature,
    Tint,
}

/// Layer blend mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    ColorDodge,
    ColorBurn,
    Darken,
    Lighten,
    Difference,
    Exclusion,
}

/// Color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub fn hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color::rgb(r, g, b))
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(Color::new(r, g, b, a))
        } else {
            None
        }
    }
}

/// Brush settings
#[derive(Debug, Clone)]
pub struct BrushSettings {
    pub size: f64,
    pub hardness: f64,
    pub opacity: f64,
    pub spacing: f64,
    pub angle: f64,
    pub color: Color,
}

impl Default for BrushSettings {
    fn default() -> Self {
        BrushSettings {
            size: 10.0,
            hardness: 0.8,
            opacity: 1.0,
            spacing: 0.1,
            angle: 0.0,
            color: Color::rgb(0, 0, 0),
        }
    }
}

/// Layer
#[derive(Debug, Clone)]
pub struct Layer {
    pub id: usize,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
    pub blend_mode: BlendMode,
    pub position: (i32, i32),
    pub size: (u32, u32),
}

/// Image history entry
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub action: String,
    pub timestamp: String,
}

/// Image document
#[derive(Debug, Clone)]
pub struct ImageDocument {
    pub path: PathBuf,
    pub format: ImageFormat,
    pub size: (u32, u32),
    pub layers: Vec<Layer>,
    pub active_layer: Option<usize>,
    pub history: Vec<HistoryEntry>,
    pub history_index: usize,
    pub modified: bool,
}

/// Batch operation
#[derive(Debug, Clone)]
pub struct BatchOperation {
    pub operation_type: String,
    pub parameters: HashMap<String, String>,
}

/// PhotoFlare editor
#[derive(Debug, Clone)]
pub struct PhotoFlareEditor {
    pub documents: Vec<ImageDocument>,
    pub current_document: Option<usize>,
    pub current_tool: DrawingTool,
    pub brush_settings: BrushSettings,
    pub foreground_color: Color,
    pub background_color: Color,
    pub zoom: f64,
    pub pan: (i32, i32),
    pub grid_visible: bool,
    pub guides_visible: bool,
    pub snap_to_grid: bool,
}

impl Default for PhotoFlareEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl PhotoFlareEditor {
    pub fn new() -> Self {
        PhotoFlareEditor {
            documents: Vec::new(),
            current_document: None,
            current_tool: DrawingTool::Brush,
            brush_settings: BrushSettings::default(),
            foreground_color: Color::rgb(0, 0, 0),
            background_color: Color::rgb(255, 255, 255),
            zoom: 1.0,
            pan: (0, 0),
            grid_visible: false,
            guides_visible: false,
            snap_to_grid: false,
        }
    }

    /// Create new document
    pub fn new_document(&mut self, width: u32, height: u32, format: ImageFormat) -> usize {
        let doc = ImageDocument {
            path: PathBuf::from("Untitled"),
            format,
            size: (width, height),
            layers: vec![Layer {
                id: 0,
                name: "Background".to_string(),
                visible: true,
                locked: false,
                opacity: 1.0,
                blend_mode: BlendMode::Normal,
                position: (0, 0),
                size: (width, height),
            }],
            active_layer: Some(0),
            history: Vec::new(),
            history_index: 0,
            modified: false,
        };

        self.documents.push(doc);
        self.current_document = Some(self.documents.len() - 1);
        self.documents.len() - 1
    }

    /// Open document
    pub fn open_document(&mut self, path: PathBuf) -> Result<usize, String> {
        let format = path
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext.to_lowercase().as_str() {
                "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
                "png" => Some(ImageFormat::Png),
                "tiff" | "tif" => Some(ImageFormat::Tiff),
                "bmp" => Some(ImageFormat::Bmp),
                "gif" => Some(ImageFormat::Gif),
                "webp" => Some(ImageFormat::WebP),
                "tga" => Some(ImageFormat::Tga),
                "ico" => Some(ImageFormat::Ico),
                "xpm" => Some(ImageFormat::Xpm),
                _ => None,
            })
            .ok_or_else(|| "Unsupported image format".to_string())?;

        let doc = ImageDocument {
            path: path.clone(),
            format,
            size: (1920, 1080),
            layers: vec![Layer {
                id: 0,
                name: "Layer 1".to_string(),
                visible: true,
                locked: false,
                opacity: 1.0,
                blend_mode: BlendMode::Normal,
                position: (0, 0),
                size: (1920, 1080),
            }],
            active_layer: Some(0),
            history: Vec::new(),
            history_index: 0,
            modified: false,
        };

        self.documents.push(doc);
        self.current_document = Some(self.documents.len() - 1);
        Ok(self.documents.len() - 1)
    }

    /// Save document
    pub fn save_document(&mut self, index: usize) -> Result<(), String> {
        if let Some(doc) = self.documents.get_mut(index) {
            doc.modified = false;
            Ok(())
        } else {
            Err("Document not found".to_string())
        }
    }

    /// Close document
    pub fn close_document(&mut self, index: usize) -> Result<(), String> {
        if index < self.documents.len() {
            self.documents.remove(index);
            if self.current_document == Some(index) {
                self.current_document = if self.documents.is_empty() {
                    None
                } else if index >= self.documents.len() {
                    Some(self.documents.len() - 1)
                } else {
                    Some(index)
                };
            }
            Ok(())
        } else {
            Err("Document not found".to_string())
        }
    }

    /// Get current document
    pub fn get_current_document(&self) -> Option<&ImageDocument> {
        self.current_document
            .and_then(|idx| self.documents.get(idx))
    }

    /// Set current tool
    pub fn set_tool(&mut self, tool: DrawingTool) {
        self.current_tool = tool;
    }

    /// Apply filter
    pub fn apply_filter(&mut self, filter: FilterType, intensity: f64) -> Result<(), String> {
        if let Some(idx) = self.current_document {
            if let Some(doc) = self.documents.get_mut(idx) {
                doc.history.push(HistoryEntry {
                    action: format!("Apply filter: {:?}", filter),
                    timestamp: "now".to_string(),
                });
                doc.history_index = doc.history.len() - 1;
                doc.modified = true;
                Ok(())
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Apply adjustment
    pub fn apply_adjustment(&mut self, adjustment: AdjustmentType, value: f64) -> Result<(), String> {
        if let Some(idx) = self.current_document {
            if let Some(doc) = self.documents.get_mut(idx) {
                doc.history.push(HistoryEntry {
                    action: format!("Apply adjustment: {:?}", adjustment),
                    timestamp: "now".to_string(),
                });
                doc.history_index = doc.history.len() - 1;
                doc.modified = true;
                Ok(())
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Add layer
    pub fn add_layer(&mut self, name: String) -> Result<(), String> {
        if let Some(idx) = self.current_document {
            if let Some(doc) = self.documents.get_mut(idx) {
                let layer_id = doc.layers.len();
                let size = doc.size;
                doc.layers.push(Layer {
                    id: layer_id,
                    name,
                    visible: true,
                    locked: false,
                    opacity: 1.0,
                    blend_mode: BlendMode::Normal,
                    position: (0, 0),
                    size,
                });
                doc.active_layer = Some(layer_id);
                doc.history.push(HistoryEntry {
                    action: "Add layer".to_string(),
                    timestamp: "now".to_string(),
                });
                doc.history_index = doc.history.len() - 1;
                doc.modified = true;
                Ok(())
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Remove layer
    pub fn remove_layer(&mut self, layer_id: usize) -> Result<(), String> {
        if let Some(idx) = self.current_document {
            if let Some(doc) = self.documents.get_mut(idx) {
                if doc.layers.len() > 1 {
                    doc.layers.retain(|l| l.id != layer_id);
                    doc.active_layer = doc.layers.last().map(|l| l.id);
                    doc.history.push(HistoryEntry {
                        action: "Remove layer".to_string(),
                        timestamp: "now".to_string(),
                    });
                    doc.history_index = doc.history.len() - 1;
                    doc.modified = true;
                    Ok(())
                } else {
                    Err("Cannot remove last layer".to_string())
                }
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Set active layer
    pub fn set_active_layer(&mut self, layer_id: usize) -> Result<(), String> {
        if let Some(idx) = self.current_document {
            if let Some(doc) = self.documents.get_mut(idx) {
                if doc.layers.iter().any(|l| l.id == layer_id) {
                    doc.active_layer = Some(layer_id);
                    Ok(())
                } else {
                    Err("Layer not found".to_string())
                }
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Set layer visibility
    pub fn set_layer_visibility(&mut self, layer_id: usize, visible: bool) -> Result<(), String> {
        if let Some(idx) = self.current_document {
            if let Some(doc) = self.documents.get_mut(idx) {
                if let Some(layer) = doc.layers.iter_mut().find(|l| l.id == layer_id) {
                    layer.visible = visible;
                    doc.modified = true;
                    Ok(())
                } else {
                    Err("Layer not found".to_string())
                }
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Set layer opacity
    pub fn set_layer_opacity(&mut self, layer_id: usize, opacity: f64) -> Result<(), String> {
        if let Some(idx) = self.current_document {
            if let Some(doc) = self.documents.get_mut(idx) {
                if let Some(layer) = doc.layers.iter_mut().find(|l| l.id == layer_id) {
                    layer.opacity = opacity.clamp(0.0, 1.0);
                    doc.modified = true;
                    Ok(())
                } else {
                    Err("Layer not found".to_string())
                }
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Set layer blend mode
    pub fn set_layer_blend_mode(&mut self, layer_id: usize, blend_mode: BlendMode) -> Result<(), String> {
        if let Some(idx) = self.current_document {
            if let Some(doc) = self.documents.get_mut(idx) {
                if let Some(layer) = doc.layers.iter_mut().find(|l| l.id == layer_id) {
                    layer.blend_mode = blend_mode;
                    doc.modified = true;
                    Ok(())
                } else {
                    Err("Layer not found".to_string())
                }
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Undo
    pub fn undo(&mut self) -> Result<(), String> {
        if let Some(idx) = self.current_document {
            if let Some(doc) = self.documents.get_mut(idx) {
                if doc.history_index > 0 {
                    doc.history_index -= 1;
                    Ok(())
                } else {
                    Err("Nothing to undo".to_string())
                }
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Redo
    pub fn redo(&mut self) -> Result<(), String> {
        if let Some(idx) = self.current_document {
            if let Some(doc) = self.documents.get_mut(idx) {
                if doc.history_index < doc.history.len() - 1 {
                    doc.history_index += 1;
                    Ok(())
                } else {
                    Err("Nothing to redo".to_string())
                }
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Set zoom
    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom.clamp(0.1, 32.0);
    }

    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.25).clamp(0.1, 32.0);
    }

    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.25).clamp(0.1, 32.0);
    }

    /// Reset zoom
    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
    }

    /// Set foreground color
    pub fn set_foreground_color(&mut self, color: Color) {
        self.foreground_color = color;
    }

    /// Set background color
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// Toggle grid
    pub fn toggle_grid(&mut self) {
        self.grid_visible = !self.grid_visible;
    }

    /// Toggle guides
    pub fn toggle_guides(&mut self) {
        self.guides_visible = !self.guides_visible;
    }

    /// Toggle snap to grid
    pub fn toggle_snap_to_grid(&mut self) {
        self.snap_to_grid = !self.snap_to_grid;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> (usize, usize, usize) {
        (
            self.documents.len(),
            self.documents.iter().map(|d| d.layers.len()).sum(),
            self.documents.iter().map(|d| d.history.len()).sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photoflare_creation() {
        let editor = PhotoFlareEditor::new();
        assert_eq!(editor.documents.len(), 0);
        assert_eq!(editor.current_tool, DrawingTool::Brush);
    }

    #[test]
    fn test_new_document() {
        let mut editor = PhotoFlareEditor::new();
        let idx = editor.new_document(1920, 1080, ImageFormat::Png);
        assert_eq!(editor.documents.len(), 1);
        assert_eq!(editor.current_document, Some(idx));
    }

    #[test]
    fn test_open_document() {
        let mut editor = PhotoFlareEditor::new();
        let path = PathBuf::from("/test/image.png");
        let idx = editor.open_document(path).unwrap();
        assert_eq!(editor.documents.len(), 1);
        assert_eq!(editor.current_document, Some(idx));
    }

    #[test]
    fn test_layers() {
        let mut editor = PhotoFlareEditor::new();
        editor.new_document(1920, 1080, ImageFormat::Png);
        editor.add_layer("Layer 2".to_string()).unwrap();
        let doc = editor.get_current_document().unwrap();
        assert_eq!(doc.layers.len(), 2);
        editor.remove_layer(1).unwrap();
        let doc = editor.get_current_document().unwrap();
        assert_eq!(doc.layers.len(), 1);
    }

    #[test]
    fn test_layer_visibility() {
        let mut editor = PhotoFlareEditor::new();
        editor.new_document(1920, 1080, ImageFormat::Png);
        editor.set_layer_visibility(0, false).unwrap();
        let doc = editor.get_current_document().unwrap();
        assert!(!doc.layers[0].visible);
    }

    #[test]
    fn test_layer_opacity() {
        let mut editor = PhotoFlareEditor::new();
        editor.new_document(1920, 1080, ImageFormat::Png);
        editor.set_layer_opacity(0, 0.5).unwrap();
        let doc = editor.get_current_document().unwrap();
        assert_eq!(doc.layers[0].opacity, 0.5);
    }

    #[test]
    fn test_blend_mode() {
        let mut editor = PhotoFlareEditor::new();
        editor.new_document(1920, 1080, ImageFormat::Png);
        editor.set_layer_blend_mode(0, BlendMode::Multiply).unwrap();
        let doc = editor.get_current_document().unwrap();
        assert_eq!(doc.layers[0].blend_mode, BlendMode::Multiply);
    }

    #[test]
    fn test_zoom() {
        let mut editor = PhotoFlareEditor::new();
        editor.zoom_in();
        assert!(editor.zoom > 1.0);
        editor.zoom_out();
        assert!(editor.zoom < 1.25);
        editor.reset_zoom();
        assert_eq!(editor.zoom, 1.0);
    }

    #[test]
    fn test_colors() {
        let mut editor = PhotoFlareEditor::new();
        let color = Color::rgb(255, 0, 0);
        editor.set_foreground_color(color);
        assert_eq!(editor.foreground_color, color);
    }

    #[test]
    fn test_color_hex() {
        let color = Color::hex("#FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_grid_toggle() {
        let mut editor = PhotoFlareEditor::new();
        editor.toggle_grid();
        assert!(editor.grid_visible);
        editor.toggle_grid();
        assert!(!editor.grid_visible);
    }

    #[test]
    fn test_statistics() {
        let mut editor = PhotoFlareEditor::new();
        editor.new_document(1920, 1080, ImageFormat::Png);
        editor.add_layer("Layer 2".to_string()).unwrap();
        let (docs, layers, history) = editor.get_statistics();
        assert_eq!(docs, 1);
        assert_eq!(layers, 2);
        assert_eq!(history, 1);
    }
}
