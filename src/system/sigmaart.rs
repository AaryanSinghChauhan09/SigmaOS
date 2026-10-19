//! SigmaArt - Art and Graphics System
//!
//! Provides comprehensive art and graphics management inspired by Art Craft.
//! Supports canvas drawing, shape primitives, and color management.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Color representation (RGBA)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
;

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    pub fn black() -> Self {
        Self::rgb(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }

    pub fn red() -> Self {
        Self::rgb(255, 0, 0)
    }

    pub fn green() -> Self {
        Self::rgb(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self::rgb(0, 0, 255)
    }
;

/// Point in 2D space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
;

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
;

/// Size in 2D space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
;

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
;

/// Rectangle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
;

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x < (self.x + self.width as i32)
            && point.y >= self.y
            && point.y < (self.y + self.height as i32)
    }
;

/// Shape type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    Rectangle,
    Circle,
    Line,
    Triangle,
    Polygon,
;

/// Shape primitive
#[derive(Debug, Clone)]
pub struct Shape {
    pub id: String,
    pub shape_type: ShapeType,
    pub position: Point,
    pub size: Size,
    pub color: Color,
    pub rotation: f32,
;

impl Shape {
    pub fn new(
        id: String,
        shape_type: ShapeType,
        position: Point,
        size: Size,
        color: Color,
    ) -> Self {
        Self {
            id,
            shape_type,
            position,
            size,
            color,
            rotation: 0.0,
        }
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }
;

/// Layer for organizing shapes
#[derive(Debug, Clone)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub shapes: BTreeMap<String, Shape>,
    pub visible: bool,
    pub opacity: f32,
;

impl Layer {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            shapes: BTreeMap::new(),
            visible: true,
            opacity: 1.0,
        }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.insert(shape.id.clone(), shape);
    }

    pub fn remove_shape(&mut self, id: &str) -> bool {
        self.shapes.remove(id).is_some()
    }

    pub fn get_shape(&self, id: &str) -> Option<&Shape> {
        self.shapes.get(id)
    }

    pub fn get_shape_mut(&mut self, id: &str) -> Option<&mut Shape> {
        self.shapes.get_mut(id)
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.max(0.0).min(1.0);
    }

    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }
;

/// Canvas for drawing
#[derive(Debug, Clone)]
pub struct Canvas {
    pub id: String,
    pub name: String,
    pub size: Size,
    pub background_color: Color,
    pub layers: BTreeMap<String, Layer>,
    pub active_layer: Option<String>,
;

impl Canvas {
    pub fn new(id: String, name: String, size: Size, background_color: Color) -> Self {
        Self {
            id,
            name,
            size,
            background_color,
            layers: BTreeMap::new(),
            active_layer: None,
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.insert(layer.id.clone(), layer);
    }

    pub fn remove_layer(&mut self, id: &str) -> bool {
        if self.active_layer.as_ref() == Some(&id.to_string()) {
            self.active_layer = None;
        }
        self.layers.remove(id).is_some()
    }

    pub fn get_layer(&self, id: &str) -> Option<&Layer> {
        self.layers.get(id)
    }

    pub fn get_layer_mut(&mut self, id: &str) -> Option<&mut Layer> {
        self.layers.get_mut(id)
    }

    pub fn set_active_layer(&mut self, id: &str) -> bool {
        if self.layers.contains_key(id) {
            self.active_layer = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_active_layer(&self) -> Option<&Layer> {
        self.active_layer
            .as_ref()
            .and_then(|id| self.layers.get(id))
    }

    pub fn get_active_layer_mut(&mut self) -> Option<&mut Layer> {
        self.active_layer
            .as_ref()
            .and_then(|id| self.layers.get_mut(id.as_str()))
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn total_shape_count(&self) -> usize {
        self.layers.values().map(|l| l.shape_count()).sum()
    }
;

/// SigmaArt - Art system manager
#[derive(Debug)]
pub struct SigmaArt {
    canvases: BTreeMap<String, Canvas>,
    current_canvas: Option<String>,
;

impl SigmaArt {
    pub fn new() -> Self {
        Self {
            canvases: BTreeMap::new(),
            current_canvas: None,
        }
    }

    pub fn add_canvas(&mut self, canvas: Canvas) {
        self.canvases.insert(canvas.id.clone(), canvas);
    }

    pub fn remove_canvas(&mut self, id: &str) -> bool {
        if self.current_canvas.as_ref() == Some(&id.to_string()) {
            self.current_canvas = None;
        }
        self.canvases.remove(id).is_some()
    }

    pub fn get_canvas(&self, id: &str) -> Option<&Canvas> {
        self.canvases.get(id)
    }

    pub fn get_canvas_mut(&mut self, id: &str) -> Option<&mut Canvas> {
        self.canvases.get_mut(id)
    }

    pub fn set_current_canvas(&mut self, id: &str) -> bool {
        if self.canvases.contains_key(id) {
            self.current_canvas = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_canvas(&self) -> Option<&Canvas> {
        self.current_canvas
            .as_ref()
            .and_then(|id| self.canvases.get(id))
    }

    pub fn get_current_canvas_mut(&mut self) -> Option<&mut Canvas> {
        self.current_canvas
            .as_ref()
            .and_then(|id| self.canvases.get_mut(id.as_str()))
    }

    pub fn list_canvases(&self) -> Vec<&Canvas> {
        self.canvases.values().collect()
    }

    pub fn canvas_count(&self) -> usize {
        self.canvases.len()
    }
;

impl Default for SigmaArt {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaArt instance
static mut SIGMAART_INSTANCE: Option<SigmaArt> = None;

/// Initialize the global SigmaArt instance
pub fn init_sigmaart() {
    unsafe {
        SIGMAART_INSTANCE = Some(SigmaArt::new());
    }
;

/// Get the global SigmaArt instance
pub fn get_sigmaart() -> Option<&'static SigmaArt> {
    unsafe { SIGMAART_INSTANCE.as_ref() }
;

/// Get the global SigmaArt instance (mutable)
pub fn get_sigmaart_mut() -> Option<&'static mut SigmaArt> {
    unsafe { SIGMAART_INSTANCE.as_mut() }
;

/// Convenience function to add a canvas
pub fn add_canvas(canvas: Canvas) -> bool {
    if let Some(art) = get_sigmaart_mut() {
        art.add_canvas(canvas);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::new(255, 128, 64, 255);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_color_presets() {
        assert_eq!(Color::black(), Color::rgb(0, 0, 0));
        assert_eq!(Color::white(), Color::rgb(255, 255, 255));
        assert_eq!(Color::red(), Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_point_creation() {
        let point = Point::new(10, 20);
        assert_eq!(point.x, 10);
        assert_eq!(point.y, 20);
    }

    #[test]
    fn test_size_creation() {
        let size = Size::new(100, 200);
        assert_eq!(size.width, 100);
        assert_eq!(size.height, 200);
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(10, 10, 50, 50);
        assert!(rect.contains(Point::new(20, 20)));
        assert!(rect.contains(Point::new(10, 10)));
        assert!(!rect.contains(Point::new(5, 5)));
        assert!(!rect.contains(Point::new(70, 70)));
    }

    #[test]
    fn test_shape_creation() {
        let shape = Shape::new(
            "shape1".to_string(),
            ShapeType::Rectangle,
            Point::new(0, 0),
            Size::new(100, 100),
            Color::red(),
        );
        assert_eq!(shape.id, "shape1");
        assert_eq!(shape.shape_type, ShapeType::Rectangle);
    }

    #[test]
    fn test_shape_modification() {
        let mut shape = Shape::new(
            "shape1".to_string(),
            ShapeType::Circle,
            Point::new(0, 0),
            Size::new(50, 50),
            Color::blue(),
        );
        shape.set_rotation(45.0);
        shape.set_color(Color::green());
        shape.set_position(Point::new(10, 10));
        assert_eq!(shape.rotation, 45.0);
    }

    #[test]
    fn test_layer_creation() {
        let layer = Layer::new("layer1".to_string(), "Test Layer".to_string());
        assert_eq!(layer.id, "layer1");
        assert!(layer.visible);
        assert_eq!(layer.opacity, 1.0);
    }

    #[test]
    fn test_layer_shape_management() {
        let mut layer = Layer::new("layer1".to_string(), "Test".to_string());
        let shape = Shape::new(
            "shape1".to_string(),
            ShapeType::Rectangle,
            Point::new(0, 0),
            Size::new(100, 100),
            Color::red(),
        );
        layer.add_shape(shape);
        assert_eq!(layer.shape_count(), 1);
        assert!(layer.remove_shape("shape1"));
        assert_eq!(layer.shape_count(), 0);
    }

    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::new(
            "canvas1".to_string(),
            "Test Canvas".to_string(),
            Size::new(800, 600),
            Color::white(),
        );
        assert_eq!(canvas.id, "canvas1");
        assert_eq!(canvas.size.width, 800);
    }

    #[test]
    fn test_canvas_layer_management() {
        let mut canvas = Canvas::new(
            "canvas1".to_string(),
            "Test".to_string(),
            Size::new(800, 600),
            Color::white(),
        );
        let layer = Layer::new("layer1".to_string(), "Layer 1".to_string());
        canvas.add_layer(layer);
        assert_eq!(canvas.layer_count(), 1);
    }

    #[test]
    fn test_sigmaart_creation() {
        let art = SigmaArt::new();
        assert_eq!(art.canvas_count(), 0);
    }

    #[test]
    fn test_sigmaart_canvas_management() {
        let mut art = SigmaArt::new();
        let canvas = Canvas::new(
            "canvas1".to_string(),
            "Test".to_string(),
            Size::new(800, 600),
            Color::white(),
        );
        art.add_canvas(canvas);
        assert_eq!(art.canvas_count(), 1);
    }
;
