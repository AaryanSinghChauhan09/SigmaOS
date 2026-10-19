//! SigmaVector - Vector Graphics System
//!
//! Provides comprehensive vector graphics management inspired by Vector Craft.
//! Supports vector paths, shapes, bezier curves, and SVG-like elements.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
}

/// Vector element type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorElementType {
    Path,
    Rectangle,
    Circle,
    Ellipse,
    Line,
    Polygon,
    Text,
}

/// Line cap style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// Line join style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// Point
#[derive(Debug, Clone, Copy)]
pub struct VectorPoint {
    pub x: f32,
    pub y: f32,
}

impl VectorPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &VectorPoint) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Vector path segment
#[derive(Debug, Clone)]
pub enum PathSegment {
    MoveTo(VectorPoint),
    LineTo(VectorPoint),
    BezierCurveTo { control1: VectorPoint, control2: VectorPoint, end: VectorPoint },
    QuadraticCurveTo { control: VectorPoint, end: VectorPoint },
    ClosePath,
}

/// Vector element
#[derive(Debug, Clone)]
pub struct VectorElement {
    pub id: String,
    pub element_type: VectorElementType,
    pub segments: Vec<PathSegment>,
    pub fill_color: String,
    pub stroke_color: String,
    pub stroke_width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub visible: bool,
}

impl VectorElement {
    pub fn new(id: String, element_type: VectorElementType) -> Self {
        Self {
            id,
            element_type,
            segments: Vec::new(),
            fill_color: "#000000".to_string(),
            stroke_color: "#000000".to_string(),
            stroke_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            visible: true,
        }
    }

    pub fn add_segment(&mut self, segment: PathSegment) {
        self.segments.push(segment);
    }

    pub fn set_fill_color(&mut self, color: String) {
        self.fill_color = color;
    }

    pub fn set_stroke_color(&mut self, color: String) {
        self.stroke_color = color;
    }

    pub fn set_stroke_width(&mut self, width: f32) {
        self.stroke_width = width;
    }

    pub fn set_line_cap(&mut self, cap: LineCap) {
        self.line_cap = cap;
    }

    pub fn set_line_join(&mut self, join: LineJoin) {
        self.line_join = join;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }
}

/// Vector layer
#[derive(Debug, Clone)]
pub struct VectorLayer {
    pub id: String,
    pub name: String,
    pub elements: BTreeMap<String, VectorElement>,
    pub visible: bool,
    pub locked: bool,
}

impl VectorLayer {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            elements: BTreeMap::new(),
            visible: true,
            locked: false,
        }
    }

    pub fn add_element(&mut self, element: VectorElement) {
        self.elements.insert(element.id.clone(), element);
    }

    pub fn remove_element(&mut self, id: &str) -> bool {
        self.elements.remove(id).is_some()
    }

    pub fn get_element(&self, id: &str) -> Option<&VectorElement> {
        self.elements.get(id)
    }

    pub fn get_element_mut(&mut self, id: &str) -> Option<&mut VectorElement> {
        self.elements.get_mut(id)
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_locked(&mut self, locked: bool) {
        self.locked = locked;
    }

    pub fn element_count(&self) -> usize {
        self.elements.len()
    }
}

/// SigmaVector - Vector graphics system
#[derive(Debug)]
pub struct SigmaVector {
    layers: BTreeMap<String, VectorLayer>,
    current_layer: Option<String>,
}

impl SigmaVector {
    pub fn new() -> Self {
        Self {
            layers: BTreeMap::new(),
            current_layer: None,
        }
    }

    pub fn add_layer(&mut self, layer: VectorLayer) {
        self.layers.insert(layer.id.clone(), layer);
    }

    pub fn remove_layer(&mut self, id: &str) -> bool {
        if self.current_layer.as_ref() == Some(&id.to_string()) {
            self.current_layer = None;
        }
        self.layers.remove(id).is_some()
    }

    pub fn get_layer(&self, id: &str) -> Option<&VectorLayer> {
        self.layers.get(id)
    }

    pub fn get_layer_mut(&mut self, id: &str) -> Option<&mut VectorLayer> {
        self.layers.get_mut(id)
    }

    pub fn set_current_layer(&mut self, id: &str) -> bool {
        if self.layers.contains_key(id) {
            self.current_layer = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_layer(&self) -> Option<&VectorLayer> {
        self.current_layer.as_ref()
            .and_then(|id| self.layers.get(id))
    }

    pub fn get_current_layer_mut(&mut self) -> Option<&mut VectorLayer> {
        self.current_layer.as_ref()
            .and_then(|id| self.layers.get_mut(id.as_str()))
    }

    pub fn list_layers(&self) -> Vec<&VectorLayer> {
        self.layers.values().collect()
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}

impl Default for SigmaVector {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaVector instance
static mut SIGMAVECTOR_INSTANCE: Option<SigmaVector> = None;

/// Initialize the global SigmaVector instance
pub fn init_sigmavector() {
    unsafe {
        SIGMAVECTOR_INSTANCE = Some(SigmaVector::new());
    }
}

/// Get the global SigmaVector instance
pub fn get_sigmavector() -> Option<&'static SigmaVector> {
    unsafe {
        SIGMAVECTOR_INSTANCE.as_ref()
    }
}

/// Get the global SigmaVector instance (mutable)
pub fn get_sigmavector_mut() -> Option<&'static mut SigmaVector> {
    unsafe {
        SIGMAVECTOR_INSTANCE.as_mut()
    }
}

/// Convenience function to add a layer
pub fn add_layer(layer: VectorLayer) -> bool {
    if let Some(vector) = get_sigmavector_mut() {
        vector.add_layer(layer);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_point_creation() {
        let point = VectorPoint::new(10.0, 20.0);
        assert_eq!(point.x, 10.0);
        assert_eq!(point.y, 20.0);
    }

    #[test]
    fn test_vector_point_distance() {
        let p1 = VectorPoint::new(0.0, 0.0);
        let p2 = VectorPoint::new(3.0, 4.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_vector_element_creation() {
        let element = VectorElement::new("element1".to_string(), VectorElementType::Path);
        assert_eq!(element.id, "element1");
        assert_eq!(element.element_type, VectorElementType::Path);
        assert!(element.visible);
    }

    #[test]
    fn test_vector_element_modification() {
        let mut element = VectorElement::new("element1".to_string(), VectorElementType::Path);
        element.add_segment(PathSegment::MoveTo(VectorPoint::new(10.0, 10.0)));
        element.set_fill_color("#FF0000".to_string());
        assert_eq!(element.segment_count(), 1);
        assert_eq!(element.fill_color, "#FF0000");
    }

    #[test]
    fn test_vector_layer_creation() {
        let layer = VectorLayer::new("layer1".to_string(), "Background".to_string());
        assert_eq!(layer.id, "layer1");
        assert!(layer.visible);
        assert!(!layer.locked);
    }

    #[test]
    fn test_vector_layer_management() {
        let mut layer = VectorLayer::new("layer1".to_string(), "Test".to_string());
        let element = VectorElement::new("element1".to_string(), VectorElementType::Path);
        layer.add_element(element);
        assert_eq!(layer.element_count(), 1);
        assert!(layer.remove_element("element1"));
        assert_eq!(layer.element_count(), 0);
    }

    #[test]
    fn test_sigmavector_creation() {
        let vector = SigmaVector::new();
        assert_eq!(vector.layer_count(), 0);
    }

    #[test]
    fn test_sigmavector_management() {
        let mut vector = SigmaVector::new();
        let layer = VectorLayer::new("layer1".to_string(), "Test".to_string());
        vector.add_layer(layer);
        assert_eq!(vector.layer_count(), 1);
    }
}
