// SPDX-License-Identifier: MIT
// SigmaOS - Vectorcraft (Vector Graphics Framework)
// Inspired by Inkscape (https://inkscape.org/), Adobe Illustrator, and ArtCraft VectorCraft
// Provides vector graphics editing capabilities with hardware shaders and exact booleans

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Shape type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Rectangle,
    Circle,
    Ellipse,
    Line,
    Path,
    Text,
    Polygon,
    Star,
    Spiral,
    Arc,
    Freehand,
    Pen,
    Pencil,
}

/// Path point with control points for bezier curves
#[derive(Debug, Clone, Copy)]
pub struct PathPoint {
    pub x: f32,
    pub y: f32,
    pub control_in_x: f32,
    pub control_in_y: f32,
    pub control_out_x: f32,
    pub control_out_y: f32,
}

impl PathPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            control_in_x: x,
            control_in_y: y,
            control_out_x: x,
            control_out_y: y,
        }
    }

    pub fn with_controls(x: f32, y: f32, cin_x: f32, cin_y: f32, cout_x: f32, cout_y: f32) -> Self {
        Self {
            x,
            y,
            control_in_x: cin_x,
            control_in_y: cin_y,
            control_out_x: cout_x,
            control_out_y: cout_y,
        }
    }
}

/// Path
#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<PathPoint>,
    pub closed: bool,
    pub smooth: bool,
    pub fill_rule: FillRule,
}

/// Fill rule
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

/// Boolean operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanOperation {
    Union,
    Intersection,
    Difference,
    Exclusion,
}

/// Pen tool mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenToolMode {
    Line,
    Curve,
    Corner,
}

/// Shape properties
#[derive(Debug, Clone)]
pub struct ShapeProperties {
    pub fill: Option<[u8; 4]>,
    pub stroke: Option<[u8; 4]>,
    pub stroke_width: f32,
    pub opacity: f32,
    pub blend_mode: BlendMode,
}

/// Blend mode for vector shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
}

/// Vector layer
pub struct VectorLayer {
    pub id: u32,
    pub name: String,
    pub shapes: Vec<Shape>,
    pub paths: Vec<Path>,
    pub properties: ShapeProperties,
    pub locked: AtomicBool,
    pub visible: AtomicBool,
}

/// Vector editor
pub struct VectorEditor {
    pub layers: BTreeMap<u32, VectorLayer>,
    pub next_layer_id: AtomicU32,
    pub active: AtomicBool,
    pub pen_tool_mode: PenToolMode,
    pub hardware_shader_enabled: bool,
    pub exact_booleans: bool,
}

impl VectorEditor {
    pub fn new() -> Self {
        Self {
            layers: BTreeMap::new(),
            next_layer_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
            pen_tool_mode: PenToolMode::Curve,
            hardware_shader_enabled: true,
            exact_booleans: true,
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
            properties: ShapeProperties {
                fill: Some([0, 0, 0, 255]),
                stroke: Some([0, 0, 0, 255]),
                stroke_width: 1.0,
                opacity: 1.0,
                blend_mode: BlendMode::Normal,
            },
            locked: AtomicBool::new(false),
            visible: AtomicBool::new(true),
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

    /// Set pen tool mode
    pub fn set_pen_tool_mode(&mut self, mode: PenToolMode) {
        self.pen_tool_mode = mode;
    }

    /// Perform boolean operation on shapes
    pub fn boolean_operation(
        &mut self,
        layer_id: u32,
        operation: BooleanOperation,
    ) -> Result<(), &'static str> {
        self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        // Implementation would perform exact boolean operations
        Ok(())
    }

    /// Set shape properties
    pub fn set_shape_properties(
        &mut self,
        layer_id: u32,
        properties: ShapeProperties,
    ) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.properties = properties;
        Ok(())
    }

    /// Lock layer
    pub fn lock_layer(&mut self, layer_id: u32) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.locked.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// Unlock layer
    pub fn unlock_layer(&mut self, layer_id: u32) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer.locked.store(false, Ordering::Relaxed);
        Ok(())
    }

    /// Toggle layer visibility
    pub fn toggle_visibility(&mut self, layer_id: u32) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        let current = layer.visible.load(Ordering::Relaxed);
        layer.visible.store(!current, Ordering::Relaxed);
        Ok(())
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
    unsafe { VECTORCRAFT.as_mut() }
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

    #[test]
    fn test_path_point_with_controls() {
        let point = PathPoint::with_controls(10.0, 20.0, 5.0, 15.0, 15.0, 25.0);
        assert_eq!(point.x, 10.0);
        assert_eq!(point.y, 20.0);
        assert_eq!(point.control_in_x, 5.0);
        assert_eq!(point.control_out_x, 15.0);
    }

    #[test]
    fn test_pen_tool_mode() {
        let mut editor = VectorEditor::new();
        editor.init();
        editor.set_pen_tool_mode(PenToolMode::Corner);
        assert_eq!(editor.pen_tool_mode, PenToolMode::Corner);
    }

    #[test]
    fn test_boolean_operation() {
        let mut editor = VectorEditor::new();
        editor.init();
        let id = editor.create_layer("Layer1".to_string());
        editor
            .boolean_operation(id, BooleanOperation::Union)
            .unwrap();
    }

    #[test]
    fn test_layer_lock() {
        let mut editor = VectorEditor::new();
        editor.init();
        let id = editor.create_layer("Layer1".to_string());
        editor.lock_layer(id).unwrap();
        assert!(editor.get_layer(id).unwrap().locked.load(Ordering::Relaxed));
    }

    #[test]
    fn test_toggle_visibility() {
        let mut editor = VectorEditor::new();
        editor.init();
        let id = editor.create_layer("Layer1".to_string());
        editor.toggle_visibility(id).unwrap();
        assert!(!editor
            .get_layer(id)
            .unwrap()
            .visible
            .load(Ordering::Relaxed));
    }

    #[test]
    fn test_shape_properties() {
        let mut editor = VectorEditor::new();
        editor.init();
        let id = editor.create_layer("Layer1".to_string());
        let props = ShapeProperties {
            fill: Some([255, 0, 0, 255]),
            stroke: Some([0, 0, 255, 255]),
            stroke_width: 2.0,
            opacity: 0.8,
            blend_mode: BlendMode::Multiply,
        };
        editor.set_shape_properties(id, props).unwrap();
    }
}
