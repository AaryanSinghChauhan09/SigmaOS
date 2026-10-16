// SPDX-License-Identifier: MIT
// SigmaOS - Designcraft (Design Tools Framework)
// Inspired by ArtCraft DesignCraft, Figma, Canva, Adobe XD, Sketch
// Provides professional design, layout, and prototyping capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Color model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorModel {
    RGB,
    HSL,
    CMYK,
    LAB,
}

/// Color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub model: ColorModel,
    pub c1: f32, // R or H or C or L
    pub c2: f32, // G or S or M or A
    pub c3: f32, // B or L or Y or B
    pub c4: f32, // A or (unused) or K or (unused)
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            model: ColorModel::RGB,
            c1: r as f32 / 255.0,
            c2: g as f32 / 255.0,
            c3: b as f32 / 255.0,
            c4: a as f32 / 255.0,
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self {
            model: ColorModel::HSL,
            c1: h,
            c2: s,
            c3: l,
            c4: a,
        }
    }
}

/// Font family
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    System,
    SansSerif,
    Serif,
    Monospace,
    Display,
    Handwriting,
}

/// Font weight
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

/// Text alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Element type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Rectangle,
    Ellipse,
    Line,
    Text,
    Image,
    Group,
    Frame,
    Component,
    Path,
    Vector,
}

/// Element
#[derive(Debug, Clone)]
pub struct Element {
    pub id: u32,
    pub element_type: ElementType,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
    pub opacity: f32,
    pub visible: bool,
    pub locked: bool,
    pub corner_radius: f32,
}

/// Text element
#[derive(Debug, Clone)]
pub struct TextElement {
    pub base: Element,
    pub content: String,
    pub font_family: FontFamily,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub alignment: TextAlignment,
    pub line_height: f32,
    pub letter_spacing: f32,
}

/// Layer
#[derive(Debug, Clone)]
pub struct Layer {
    pub id: u32,
    pub name: String,
    pub elements: Vec<u32>, // element IDs
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend_mode: DesignBlendMode,
}

/// Blend mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignBlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
}

/// Guide
#[derive(Debug, Clone)]
pub struct Guide {
    pub id: u32,
    pub orientation: GuideOrientation,
    pub position: f32,
    pub color: Color,
}

/// Guide orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuideOrientation {
    Horizontal,
    Vertical,
}

/// Grid
#[derive(Debug, Clone)]
pub struct Grid {
    pub id: u32,
    pub size: f32,
    pub color: Color,
    pub enabled: bool,
}

/// Component (reusable symbol)
#[derive(Debug, Clone)]
pub struct Component {
    pub id: u32,
    pub name: String,
    pub elements: Vec<Element>,
    pub master: bool,
}

/// Style (design token)
#[derive(Debug, Clone)]
pub struct Style {
    pub id: u32,
    pub name: String,
    pub style_type: StyleType,
    pub value: String,
}

/// Style type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleType {
    Color,
    Typography,
    Effect,
    Grid,
}

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    PNG,
    JPEG,
    SVG,
    PDF,
}

/// Artboard (canvas with fixed size)
#[derive(Debug, Clone)]
pub struct Artboard {
    pub id: u32,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background: Color,
    pub elements: Vec<u32>,
}

/// Canvas
#[derive(Debug, Clone)]
pub struct Canvas {
    pub id: u32,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub background: Color,
    pub artboards: BTreeMap<u32, Artboard>,
    pub layers: BTreeMap<u32, Layer>,
    pub elements: BTreeMap<u32, Element>,
    pub guides: Vec<Guide>,
    pub grid: Option<Grid>,
}

/// Design editor
pub struct DesignEditor {
    pub project: Option<String>,
    pub canvases: BTreeMap<u32, Canvas>,
    pub components: BTreeMap<u32, Component>,
    pub styles: BTreeMap<u32, Style>,
    pub next_canvas_id: AtomicU32,
    pub next_element_id: AtomicU32,
    pub next_layer_id: AtomicU32,
    pub next_component_id: AtomicU32,
    pub next_style_id: AtomicU32,
    pub active: AtomicBool,
    pub snap_to_grid: AtomicBool,
    pub snap_to_guides: AtomicBool,
}

impl DesignEditor {
    pub fn new() -> Self {
        Self {
            project: None,
            canvases: BTreeMap::new(),
            components: BTreeMap::new(),
            styles: BTreeMap::new(),
            next_canvas_id: AtomicU32::new(1),
            next_element_id: AtomicU32::new(1),
            next_layer_id: AtomicU32::new(1),
            next_component_id: AtomicU32::new(1),
            next_style_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
            snap_to_grid: AtomicBool::new(true),
            snap_to_guides: AtomicBool::new(true),
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
            artboards: BTreeMap::new(),
            layers: BTreeMap::new(),
            elements: BTreeMap::new(),
            guides: Vec::new(),
            grid: None,
        };
        self.canvases.insert(id, canvas);
        id
    }

    /// Add artboard to canvas
    pub fn add_artboard(
        &mut self,
        canvas_id: u32,
        name: String,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Result<u32, &'static str> {
        let canvas = self
            .canvases
            .get_mut(&canvas_id)
            .ok_or("Canvas not found")?;
        let id = canvas.artboards.len() as u32;
        let artboard = Artboard {
            id,
            name,
            x,
            y,
            width,
            height,
            background: Color::rgb(255, 255, 255),
            elements: Vec::new(),
        };
        canvas.artboards.insert(id, artboard);
        Ok(id)
    }

    /// Add layer
    pub fn add_layer(&mut self, canvas_id: u32, name: String) -> Result<u32, &'static str> {
        let canvas = self
            .canvases
            .get_mut(&canvas_id)
            .ok_or("Canvas not found")?;
        let id = self.next_layer_id.fetch_add(1, Ordering::Relaxed);
        let layer = Layer {
            id,
            name,
            elements: Vec::new(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: DesignBlendMode::Normal,
        };
        canvas.layers.insert(id, layer);
        Ok(id)
    }

    /// Add element
    pub fn add_element(
        &mut self,
        canvas_id: u32,
        layer_id: u32,
        element_type: ElementType,
        name: String,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Result<u32, &'static str> {
        let canvas = self
            .canvases
            .get_mut(&canvas_id)
            .ok_or("Canvas not found")?;
        let layer = canvas.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        let id = self.next_element_id.fetch_add(1, Ordering::Relaxed);
        let element = Element {
            id,
            element_type,
            name,
            x,
            y,
            width,
            height,
            rotation: 0.0,
            fill: None,
            stroke: None,
            stroke_width: 0.0,
            opacity: 1.0,
            visible: true,
            locked: false,
            corner_radius: 0.0,
        };
        layer.elements.push(id);
        canvas.elements.insert(id, element);
        Ok(id)
    }

    /// Set element fill
    pub fn set_element_fill(
        &mut self,
        canvas_id: u32,
        layer_id: u32,
        element_id: u32,
        color: Color,
    ) -> Result<(), &'static str> {
        let canvas = self
            .canvases
            .get_mut(&canvas_id)
            .ok_or("Canvas not found")?;
        let layer = canvas.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        if !layer.elements.contains(&element_id) {
            return Err("Element not in layer");
        }
        if let Some(elem) = canvas.elements.get_mut(&element_id) {
            elem.fill = Some(color);
        }
        Ok(())
    }

    /// Add guide
    pub fn add_guide(
        &mut self,
        canvas_id: u32,
        orientation: GuideOrientation,
        position: f32,
    ) -> Result<(), &'static str> {
        let canvas = self
            .canvases
            .get_mut(&canvas_id)
            .ok_or("Canvas not found")?;
        let id = canvas.guides.len() as u32;
        let guide = Guide {
            id,
            orientation,
            position,
            color: Color::rgba(0, 0, 255, 128),
        };
        canvas.guides.push(guide);
        Ok(())
    }

    /// Set grid
    pub fn set_grid(
        &mut self,
        canvas_id: u32,
        size: f32,
        enabled: bool,
    ) -> Result<(), &'static str> {
        let canvas = self
            .canvases
            .get_mut(&canvas_id)
            .ok_or("Canvas not found")?;
        canvas.grid = Some(Grid {
            id: 0,
            size,
            color: Color::rgba(200, 200, 200, 128),
            enabled,
        });
        Ok(())
    }

    /// Create component
    pub fn create_component(&mut self, name: String, elements: Vec<Element>) -> u32 {
        let id = self.next_component_id.fetch_add(1, Ordering::Relaxed);
        let component = Component {
            id,
            name,
            elements,
            master: true,
        };
        self.components.insert(id, component);
        id
    }

    /// Create style
    pub fn create_style(&mut self, name: String, style_type: StyleType, value: String) -> u32 {
        let id = self.next_style_id.fetch_add(1, Ordering::Relaxed);
        let style = Style {
            id,
            name,
            style_type,
            value,
        };
        self.styles.insert(id, style);
        id
    }

    /// Toggle snap to grid
    pub fn toggle_snap_to_grid(&mut self) {
        let current = self.snap_to_grid.load(Ordering::Relaxed);
        self.snap_to_grid.store(!current, Ordering::Relaxed);
    }

    /// Toggle snap to guides
    pub fn toggle_snap_to_guides(&mut self) {
        let current = self.snap_to_guides.load(Ordering::Relaxed);
        self.snap_to_guides.store(!current, Ordering::Relaxed);
    }

    /// Set canvas background
    pub fn set_canvas_background(
        &mut self,
        canvas_id: u32,
        color: Color,
    ) -> Result<(), &'static str> {
        let canvas = self
            .canvases
            .get_mut(&canvas_id)
            .ok_or("Canvas not found")?;
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
    unsafe { DESIGNCRAFT.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

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

    #[test]
    fn test_colors() {
        let rgb = Color::rgb(255, 0, 0);
        assert_eq!(rgb.model, ColorModel::RGB);

        let hsla = Color::hsla(0.0, 100.0, 50.0, 1.0);
        assert_eq!(hsla.model, ColorModel::HSL);
    }

    #[test]
    fn test_artboards() {
        let mut editor = DesignEditor::new();
        editor.init();
        let canvas_id = editor.create_canvas("Canvas".to_string(), 1920.0, 1080.0);
        let artboard_id = editor
            .add_artboard(canvas_id, "Artboard 1".to_string(), 0.0, 0.0, 375.0, 667.0)
            .unwrap();
        assert!(editor
            .get_canvas(canvas_id)
            .unwrap()
            .artboards
            .contains_key(&artboard_id));
    }

    #[test]
    fn test_layers() {
        let mut editor = DesignEditor::new();
        editor.init();
        let canvas_id = editor.create_canvas("Canvas".to_string(), 1920.0, 1080.0);
        let layer_id = editor.add_layer(canvas_id, "Layer 1".to_string()).unwrap();
        assert!(editor
            .get_canvas(canvas_id)
            .unwrap()
            .layers
            .contains_key(&layer_id));
    }

    #[test]
    fn test_elements() {
        let mut editor = DesignEditor::new();
        editor.init();
        let canvas_id = editor.create_canvas("Canvas".to_string(), 1920.0, 1080.0);
        let layer_id = editor.add_layer(canvas_id, "Layer 1".to_string()).unwrap();
        let element_id = editor
            .add_element(
                canvas_id,
                layer_id,
                ElementType::Rectangle,
                "Rect".to_string(),
                100.0,
                100.0,
                200.0,
                150.0,
            )
            .unwrap();
        assert!(editor.get_canvas(canvas_id).unwrap().layers[&layer_id]
            .elements
            .contains(&element_id));
    }

    #[test]
    fn test_guides() {
        let mut editor = DesignEditor::new();
        editor.init();
        let canvas_id = editor.create_canvas("Canvas".to_string(), 1920.0, 1080.0);
        editor
            .add_guide(canvas_id, GuideOrientation::Vertical, 960.0)
            .unwrap();
        assert_eq!(editor.get_canvas(canvas_id).unwrap().guides.len(), 1);
    }

    #[test]
    fn test_grid() {
        let mut editor = DesignEditor::new();
        editor.init();
        let canvas_id = editor.create_canvas("Canvas".to_string(), 1920.0, 1080.0);
        editor.set_grid(canvas_id, 10.0, true).unwrap();
        assert!(editor.get_canvas(canvas_id).unwrap().grid.is_some());
    }

    #[test]
    fn test_components() {
        let mut editor = DesignEditor::new();
        editor.init();
        let element = Element {
            id: 0,
            element_type: ElementType::Rectangle,
            name: "Button".to_string(),
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 40.0,
            rotation: 0.0,
            fill: Some(Color::rgb(0, 100, 255)),
            stroke: None,
            stroke_width: 0.0,
            opacity: 1.0,
            visible: true,
            locked: false,
            corner_radius: 5.0,
        };
        let component_id = editor.create_component("Button".to_string(), vec![element]);
        assert!(editor.components.contains_key(&component_id));
    }

    #[test]
    fn test_styles() {
        let mut editor = DesignEditor::new();
        editor.init();
        let style_id = editor.create_style(
            "Primary Color".to_string(),
            StyleType::Color,
            "#0066FF".to_string(),
        );
        assert!(editor.styles.contains_key(&style_id));
    }

    #[test]
    fn test_snap_toggle() {
        let mut editor = DesignEditor::new();
        editor.init();
        assert!(editor.snap_to_grid.load(Ordering::Relaxed));
        editor.toggle_snap_to_grid();
        assert!(!editor.snap_to_grid.load(Ordering::Relaxed));
    }
}
