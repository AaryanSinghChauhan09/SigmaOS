// SPDX-License-Identifier: MIT
// SigmaOS - Vectorcraft (Vector Graphics Framework)
// Inspired by Inkscape (https://inkscape.org/), Adobe Illustrator
// Provides vector graphics editing capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Shape
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Rectangle,
    Circle,
    Ellipse,
    Line,
    Path,
    Text,
}

/// Path point
#[derive(Debug, Clone, Copy)]
pub struct PathPoint {
    pub x: f32,
    pub y: f32,
}

/// Path
#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<PathPoint>,
    pub closed: bool,
}

/// Vector layer
pub struct VectorLayer {
    pub id: u32,
    pub name: String,
    pub shapes: Vec<Shape>,
    pub paths: Vec<Path>,
}

/// Vector editor
pub struct VectorEditor {
    pub layers: BTreeMap<u32, VectorLayer>,
    pub next_layer_id: AtomicU32,
    pub active: AtomicBool,
}

impl VectorEditor {
    pub fn new() -> Self {
        Self {
            layers: BTreeMap::new(),
            next_layer_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize vector editor
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }
    
    /// Create layer
    pub fn create_layer(&mut self, name: String) -> u32 {
        let id = self.next_layer_id.fetch_add(1, Ordering::Relaxed);
        let layer = VectorLayer {
            id,
            name,
            shapes: Vec::new(),
            paths: Vec::new(),
        };
        self.layers.insert(id, layer);
        id
    }
    
    /// Add shape to layer
    pub fn add_shape(&mut self, layer_id: u32, shape: Shape) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.shapes.push(shape);
        Ok(())
    }
    
    /// Add path to layer
    pub fn add_path(&mut self, layer_id: u32, path: Path) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.paths.push(path);
        Ok(())
    }
    
    /// Get layer
    pub fn get_layer(&self, id: u32) -> Option<&VectorLayer> {
        self.layers.get(&id)
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for VectorEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Global vectorcraft
static mut VECTORCRAFT: Option<VectorEditor> = None;

/// Initialize vectorcraft
pub fn init_vectorcraft() {
    unsafe {
        VECTORCRAFT = Some(VectorEditor::new());
    }
}

/// Get global vectorcraft
pub fn get_vectorcraft() -> Option<&'static mut VectorEditor> {
    unsafe {
        VECTORCRAFT.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vectorcraft_init() {
        let mut editor = VectorEditor::new();
        editor.init();
        assert!(editor.is_active());
    }
    
    #[test]
    fn test_create_layer() {
        let mut editor = VectorEditor::new();
        editor.init();
        let id = editor.create_layer("Layer1".to_string());
        assert!(editor.get_layer(id).is_some());
    }
    
    #[test]
    fn test_add_shape() {
        let mut editor = VectorEditor::new();
        editor.init();
        let id = editor.create_layer("Layer1".to_string());
        editor.add_shape(id, Shape::Rectangle).unwrap();
        assert_eq!(editor.get_layer(id).unwrap().shapes.len(), 1);
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: alloc::string::String,
}
