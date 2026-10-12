// SPDX-License-Identifier: MIT
// SigmaOS - Photocraft (Photo Editing Framework)
// Inspired by GIMP (https://www.gimp.org/), Krita (https://krita.org/)
// Provides photo editing capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Image layer
#[derive(Debug)]
pub struct ImageLayer {
    pub id: u32,
    pub name: String,
    pub visible: AtomicBool,
    pub opacity: f32,
    pub blend_mode: BlendMode,
}

/// Blend mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
}

/// Filter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    Blur,
    Sharpen,
    Noise,
    Brightness,
    Contrast,
    Saturation,
    Hue,
}

/// Adjustment
#[derive(Debug, Clone, Copy)]
pub struct Adjustment {
    pub filter: Filter,
    pub value: f32,
}

/// Photo editor
pub struct PhotoEditor {
    pub layers: BTreeMap<u32, ImageLayer>,
    pub current_image: Option<String>,
    pub next_layer_id: AtomicU32,
    pub active: AtomicBool,
    pub history: Vec<String>,
}

impl PhotoEditor {
    pub fn new() -> Self {
        Self {
            layers: BTreeMap::new(),
            current_image: None,
            next_layer_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
            history: Vec::new(),
        }
    }
    
    /// Initialize photo editor
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }
    
    /// Load image
    pub fn load_image(&mut self, path: String) -> Result<(), &'static str> {
        self.current_image = Some(path.clone());
        self.history.push(format!("Load: {}", path));
        Ok(())
    }
    
    /// Create layer
    pub fn create_layer(&mut self, name: String) -> u32 {
        let id = self.next_layer_id.fetch_add(1, Ordering::Relaxed);
        let layer = ImageLayer {
            id,
            name,
            visible: AtomicBool::new(true),
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
        };
        self.layers.insert(id, layer);
        id
    }
    
    /// Get layer
    pub fn get_layer(&self, id: u32) -> Option<&ImageLayer> {
        self.layers.get(&id)
    }
    
    /// Apply filter
    pub fn apply_filter(&mut self, layer_id: u32, filter: Filter, value: f32) -> Result<(), &'static str> {
        self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        self.history.push(format!("Apply filter: {:?} with value {}", filter, value));
        Ok(())
    }
    
    /// Adjust layer opacity
    pub fn adjust_opacity(&mut self, layer_id: u32, opacity: f32) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.opacity = opacity;
        Ok(())
    }
    
    /// Set blend mode
    pub fn set_blend_mode(&mut self, layer_id: u32, blend_mode: BlendMode) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.blend_mode = blend_mode;
        Ok(())
    }
    
    /// Undo
    pub fn undo(&mut self) -> Result<(), &'static str> {
        if self.history.is_empty() {
            return Err("No history to undo");
        }
        self.history.pop();
        Ok(())
    }
    
    /// Get current image
    pub fn get_current_image(&self) -> Option<&String> {
        self.current_image.as_ref()
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for PhotoEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Global photo editor
static mut PHOTOCRAFT: Option<PhotoEditor> = None;

/// Initialize photocraft
pub fn init_photocraft() {
    unsafe {
        PHOTOCRAFT = Some(PhotoEditor::new());
    }
}

/// Get global photo editor
pub fn get_photocraft() -> Option<&'static mut PhotoEditor> {
    unsafe {
        PHOTOCRAFT.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_photocraft_init() {
        let mut editor = PhotoEditor::new();
        editor.init();
        assert!(editor.is_active());
    }
    
    #[test]
    fn test_load_image() {
        let mut editor = PhotoEditor::new();
        editor.init();
        editor.load_image("/path/to/image.jpg".to_string()).unwrap();
        assert!(editor.get_current_image().is_some());
    }
    
    #[test]
    fn test_create_layer() {
        let mut editor = PhotoEditor::new();
        editor.init();
        let id = editor.create_layer("Background".to_string());
        assert!(editor.get_layer(id).is_some());
    }
    
    #[test]
    fn test_apply_filter() {
        let mut editor = PhotoEditor::new();
        editor.init();
        let id = editor.create_layer("Layer1".to_string());
        editor.apply_filter(id, Filter::Blur, 50.0).unwrap();
        assert_eq!(editor.history.len(), 1);
    }
}
