// SPDX-License-Identifier: MIT
// SigmaOS - Image Editor
// Inspired by GIMP and Krita
// Provides image editing capabilities

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    PNG,
    JPEG,
    WEBP,
    BMP,
    TIFF,
    GIF,
}

/// Color mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    RGB,
    RGBA,
    Grayscale,
    CMYK,
}

/// Tool type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolType {
    Brush,
    Eraser,
    Selection,
    Move,
    Crop,
    Text,
    Fill,
    Gradient,
    ColorPicker,
    Zoom,
}

/// Blend mode
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
}

/// Layer
#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub opacity: u8,
    pub blend_mode: BlendMode,
    pub locked: bool,
}

/// Selection
#[derive(Debug, Clone)]
pub struct Selection {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub active: bool,
}

/// Image editor
pub struct ImageEditor {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub color_mode: ColorMode,
    pub current_tool: ToolType,
    pub brush_size: u32,
    pub brush_opacity: u8,
    pub foreground_color: String,
    pub background_color: String,
    pub layers: Vec<Layer>,
    pub active_layer: Option<usize>,
    pub selection: Selection,
    pub zoom: f32,
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
    pub file_path: String,
    pub modified: bool,
}

impl ImageEditor {
    pub fn new() -> Self {
        Self {
            width: 1920,
            height: 1080,
            format: ImageFormat::PNG,
            color_mode: ColorMode::RGBA,
            current_tool: ToolType::Brush,
            brush_size: 10,
            brush_opacity: 255,
            foreground_color: "#000000".to_string(),
            background_color: "#FFFFFF".to_string(),
            layers: vec![Layer {
                name: "Background".to_string(),
                visible: true,
                opacity: 255,
                blend_mode: BlendMode::Normal,
                locked: false,
            }],
            active_layer: Some(0),
            selection: Selection {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                active: false,
            },
            zoom: 1.0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            file_path: String::new(),
            modified: false,
        }
    }

    /// Set tool
    pub fn set_tool(&mut self, tool: ToolType) {
        self.current_tool = tool;
    }

    /// Set brush size
    pub fn set_brush_size(&mut self, size: u32) {
        self.brush_size = size;
    }

    /// Set brush opacity
    pub fn set_brush_opacity(&mut self, opacity: u8) {
        self.brush_opacity = opacity;
    }

    /// Set foreground color
    pub fn set_foreground_color(&mut self, color: String) {
        self.foreground_color = color;
    }

    /// Set background color
    pub fn set_background_color(&mut self, color: String) {
        self.background_color = color;
    }

    /// Add layer
    pub fn add_layer(&mut self, name: String) {
        let layer = Layer {
            name,
            visible: true,
            opacity: 255,
            blend_mode: BlendMode::Normal,
            locked: false,
        };
        self.layers.push(layer);
        self.active_layer = Some(self.layers.len() - 1);
    }

    /// Remove layer
    pub fn remove_layer(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.layers.len() {
            return Err("Layer index out of bounds");
        }
        if self.layers.len() == 1 {
            return Err("Cannot remove the last layer");
        }
        self.layers.remove(index);
        if self.active_layer == Some(index) {
            self.active_layer = Some(0);
        }
        Ok(())
    }

    /// Set active layer
    pub fn set_active_layer(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.layers.len() {
            return Err("Layer index out of bounds");
        }
        self.active_layer = Some(index);
        Ok(())
    }

    /// Toggle layer visibility
    pub fn toggle_layer_visibility(&mut self, index: usize) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(index).ok_or("Layer not found")?;
        layer.visible = !layer.visible;
        Ok(())
    }

    /// Set layer opacity
    pub fn set_layer_opacity(&mut self, index: usize, opacity: u8) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(index).ok_or("Layer not found")?;
        layer.opacity = opacity;
        Ok(())
    }

    /// Set layer blend mode
    pub fn set_layer_blend_mode(
        &mut self,
        index: usize,
        blend_mode: BlendMode,
    ) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(index).ok_or("Layer not found")?;
        layer.blend_mode = blend_mode;
        Ok(())
    }

    /// Set selection
    pub fn set_selection(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.selection = Selection {
            x,
            y,
            width,
            height,
            active: true,
        };
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection.active = false;
    }

    /// Set zoom
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1).min(10.0);
    }

    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.25).min(10.0);
    }

    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.25).max(0.1);
    }

    /// Reset zoom
    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
    }

    /// Undo
    pub fn undo(&mut self) -> Result<(), &'static str> {
        if self.undo_stack.is_empty() {
            return Err("Nothing to undo");
        }
        let state = self.undo_stack.pop().unwrap();
        self.redo_stack.push(state);
        Ok(())
    }

    /// Redo
    pub fn redo(&mut self) -> Result<(), &'static str> {
        if self.redo_stack.is_empty() {
            return Err("Nothing to redo");
        }
        let state = self.redo_stack.pop().unwrap();
        self.undo_stack.push(state);
        Ok(())
    }

    /// Save
    pub fn save(&mut self, path: String) -> Result<(), &'static str> {
        self.file_path = path;
        self.modified = false;
        Ok(())
    }

    /// Load
    pub fn load(&mut self, path: String) -> Result<(), &'static str> {
        self.file_path = path;
        self.modified = false;
        Ok(())
    }

    /// Get layers
    pub fn get_layers(&self) -> &[Layer] {
        &self.layers
    }

    /// Layer count
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}

/// Global image editor
static mut IMAGE_EDITOR: Option<ImageEditor> = None;

/// Initialize image editor
pub fn init_image_editor() {
    unsafe {
        IMAGE_EDITOR = Some(ImageEditor::new());
    }
}

/// Get image editor
pub fn get_image_editor() -> &'static mut ImageEditor {
    unsafe { IMAGE_EDITOR.as_mut().expect("Image editor not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_editor_creation() {
        let ie = ImageEditor::new();
        assert_eq!(ie.width, 1920);
        assert_eq!(ie.height, 1080);
    }

    #[test]
    fn test_add_layer() {
        let mut ie = ImageEditor::new();
        ie.add_layer("New Layer".to_string());
        assert_eq!(ie.layer_count(), 2);
    }

    #[test]
    fn test_zoom() {
        let mut ie = ImageEditor::new();
        ie.zoom_in();
        assert!(ie.zoom > 1.0);
        ie.zoom_out();
        assert!(ie.zoom < 1.25);
    }
}
