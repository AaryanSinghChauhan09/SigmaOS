// SPDX-License-Identifier: MIT
// SigmaOS - Designcraft (Design Tools Framework)
// Inspired by Figma (https://www.figma.com/), Canva (https://www.canva.com/)
// Provides design and layout capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

/// Design element
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Rectangle,
    Circle,
    Text,
    Image,
    Group,
}

/// Canvas
#[derive(Debug, Clone)]
pub struct Canvas {
    pub id: u32,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub background: Color,
}

/// Design editor
pub struct DesignEditor {
    pub project: Option<String>,
    pub canvases: BTreeMap<u32, Canvas>,
    pub next_canvas_id: AtomicU32,
    pub active: AtomicBool,
}

impl DesignEditor {
    pub fn new() -> Self {
        Self {
            project: None,
            canvases: BTreeMap::new(),
            next_canvas_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize design editor
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }
    
    /// Create project
    pub fn create_project(&mut self, name: String) {
        self.project = Some(name);
    }
    
    /// Create canvas
    pub fn create_canvas(&mut self, name: String, width: f32, height: f32) -> u32 {
        let id = self.next_canvas_id.fetch_add(1, Ordering::Relaxed);
        let canvas = Canvas {
            id,
            name,
            width,
            height,
            background: Color::rgb(255, 255, 255),
        };
        self.canvases.insert(id, canvas);
        id
    }
    
    /// Set canvas background
    pub fn set_canvas_background(&mut self, canvas_id: u32, color: Color) -> Result<(), &'static str> {
        let canvas = self.canvases.get_mut(&canvas_id).ok_or("Canvas not found")?;
        canvas.background = color;
        Ok(())
    }
    
    /// Get canvas
    pub fn get_canvas(&self, id: u32) -> Option<&Canvas> {
        self.canvases.get(&id)
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for DesignEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Global designcraft
static mut DESIGNCRAFT: Option<DesignEditor> = None;

/// Initialize designcraft
pub fn init_designcraft() {
    unsafe {
        DESIGNCRAFT = Some(DesignEditor::new());
    }
}

/// Get global designcraft
pub fn get_designcraft() -> Option<&'static mut DesignEditor> {
    unsafe {
        DESIGNCRAFT.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_designcraft_init() {
        let mut editor = DesignEditor::new();
        editor.init();
        assert!(editor.is_active());
    }
    
    #[test]
    fn test_create_project() {
        let mut editor = DesignEditor::new();
        editor.init();
        editor.create_project("Design Project".to_string());
        assert!(editor.project.is_some());
    }
    
    #[test]
    fn test_create_canvas() {
        let mut editor = DesignEditor::new();
        editor.init();
        editor.create_canvas("Canvas1".to_string(), 1920.0, 1080.0);
        assert_eq!(editor.canvases.len(), 1);
    }
}
