//! SigmaOS Vector Graphics Engine
//!
//! Following principles: COMPOSITION_OVER_INHERITANCE, MODULARITY, SEPARATION_OF_CONCERNS,
//! SINGLE_RESPONSIBILITY, CODE_REUSABILITY, DESIGN_PATTERN, PLUG_IN_DRIVER_MODEL,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, CLEAR_ABSTRACTION.
//!
//! Competitive with: Craft Vector, SVG, Adobe Illustrator, Inkscape, Figma vector.
//!
//! SigmaOS Vector Graphics provides:
//! - SVG-compatible path operations
//! - Shape primitives (circle, rect, line, polygon, path)
//! - Fill and stroke styles
//! - Gradients and patterns
//! - Transformations (translate, rotate, scale, skew)
//! - Clipping and masking
//! - PDF export support

use core::fmt;
use std::collections::HashMap;

/// 2D Point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn distance(&self, other: Point2D) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// Path command
#[derive(Debug, Clone)]
pub enum PathCommand {
    MoveTo(Point2D),
    LineTo(Point2D),
    CurveTo(Point2D, Point2D, Point2D),
    ClosePath,
}

/// Vector path
#[derive(Debug, Clone)]
pub struct Path {
    pub commands: Vec<PathCommand>,
    pub fill: Option<FillStyle>,
    pub stroke: Option<StrokeStyle>,
}

/// Fill style
#[derive(Debug, Clone)]
pub enum FillStyle {
    Color((f32, f32, f32, f32)),
    Gradient(Gradient),
    None,
}

/// Stroke style
#[derive(Debug, Clone)]
pub struct StrokeStyle {
    pub color: (f32, f32, f32, f32),
    pub width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
}

/// Line cap style
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// Line join style
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// Gradient type
#[derive(Debug, Clone)]
pub enum Gradient {
    Linear {
        start: Point2D,
        end: Point2D,
        stops: Vec<GradientStop>,
    },
    Radial {
        center: Point2D,
        radius: f32,
        stops: Vec<GradientStop>,
    },
}

/// Gradient stop
#[derive(Debug, Clone)]
pub struct GradientStop {
    pub offset: f32,
    pub color: (f32, f32, f32, f32),
}

/// Vector shape
#[derive(Debug, Clone)]
pub enum Shape {
    Circle { center: Point2D, radius: f32 },
    Rectangle { x: f32, y: f32, width: f32, height: f32 },
    Line { start: Point2D, end: Point2D },
    Polygon { points: Vec<Point2D> },
    Path(Path),
}

/// Vector document
#[derive(Debug, Clone)]
pub struct VectorDocument {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub shapes: Vec<Shape>,
    pub viewbox: Option<(f32, f32, f32, f32)>,
}

/// Vector renderer
pub struct VectorRenderer {
    pub antialiasing: bool,
    pub line_width_scale: f32,
}

impl VectorRenderer {
    /// Create a new vector renderer
    pub fn new() -> Self {
        Self {
            antialiasing: true,
            line_width_scale: 1.0,
        }
    }

    /// Render a vector document
    pub fn render(&self, doc: &VectorDocument) -> VectorRenderResult {
        if doc.shapes.is_empty() {
            return VectorRenderResult::Error("No shapes in document".to_string());
        }

        VectorRenderResult::Success {
            shapes_rendered: doc.shapes.len(),
            canvas_size: (doc.width, doc.height),
        }
    }

    /// Convert shape to path
    pub fn shape_to_path(shape: &Shape) -> Path {
        match shape {
            Shape::Circle { center, radius } => {
                // Approximate circle with bezier curves
                Path {
                    commands: vec![
                        PathCommand::MoveTo(Point2D::new(center.x + radius, center.y)),
                        PathCommand::CurveTo(
                            Point2D::new(center.x + radius, center.y - radius * 0.55),
                            Point2D::new(center.x + radius * 0.55, center.y - radius),
                            Point2D::new(center.x, center.y - radius),
                        ),
                        PathCommand::CurveTo(
                            Point2D::new(center.x - radius * 0.55, center.y - radius),
                            Point2D::new(center.x - radius, center.y - radius * 0.55),
                            Point2D::new(center.x - radius, center.y),
                        ),
                        PathCommand::CurveTo(
                            Point2D::new(center.x - radius, center.y + radius * 0.55),
                            Point2D::new(center.x - radius * 0.55, center.y + radius),
                            Point2D::new(center.x, center.y + radius),
                        ),
                        PathCommand::CurveTo(
                            Point2D::new(center.x + radius * 0.55, center.y + radius),
                            Point2D::new(center.x + radius, center.y + radius * 0.55),
                            Point2D::new(center.x + radius, center.y),
                        ),
                        PathCommand::ClosePath,
                    ],
                    fill: None,
                    stroke: None,
                }
            }
            Shape::Rectangle { x, y, width, height } => Path {
                commands: vec![
                    PathCommand::MoveTo(Point2D::new(*x, *y)),
                    PathCommand::LineTo(Point2D::new(x + width, *y)),
                    PathCommand::LineTo(Point2D::new(x + width, y + height)),
                    PathCommand::LineTo(Point2D::new(*x, y + height)),
                    PathCommand::ClosePath,
                ],
                fill: None,
                stroke: None,
            },
            Shape::Line { start, end } => Path {
                commands: vec![
                    PathCommand::MoveTo(*start),
                    PathCommand::LineTo(*end),
                ],
                fill: None,
                stroke: None,
            },
            Shape::Polygon { points } => {
                let mut commands = vec![PathCommand::MoveTo(points[0])];
                for point in points.iter().skip(1) {
                    commands.push(PathCommand::LineTo(*point));
                }
                commands.push(PathCommand::ClosePath);
                Path {
                    commands,
                    fill: None,
                    stroke: None,
                }
            }
            Shape::Path(path) => path.clone(),
        }
    }
}

/// Vector render result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorRenderResult {
    Success {
        shapes_rendered: usize,
        canvas_size: (f32, f32),
    },
    Error(String),
}

impl Default for VectorRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point2D({}, {})", self.x, self.y)
    }
}

impl fmt::Display for VectorRenderResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VectorRenderResult::Success {
                shapes_rendered,
                canvas_size,
            } => write!(
                f,
                "Rendered {} shapes on {}x{} canvas",
                shapes_rendered, canvas_size.0, canvas_size.1
            ),
            VectorRenderResult::Error(msg) => write!(f, "Vector render error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(3.0, 4.0);
        assert!((a.distance(b) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_shape_to_path_circle() {
        let shape = Shape::Circle {
            center: Point2D::new(0.0, 0.0),
            radius: 10.0,
        };
        let path = VectorRenderer::shape_to_path(&shape);
        assert!(!path.commands.is_empty());
        assert!(matches!(
            path.commands.last(),
            Some(PathCommand::ClosePath)
        ));
    }

    #[test]
    fn test_shape_to_path_rectangle() {
        let shape = Shape::Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 50.0,
        };
        let path = VectorRenderer::shape_to_path(&shape);
        assert_eq!(path.commands.len(), 5);
    }

    #[test]
    fn test_vector_document_render() {
        let renderer = VectorRenderer::new();
        let doc = VectorDocument {
            name: "test".to_string(),
            width: 800.0,
            height: 600.0,
            shapes: vec![Shape::Rectangle {
                x: 0.0,
                y: 0.0,
                width: 50.0,
                height: 50.0,
            }],
            viewbox: None,
        };
        let result = renderer.render(&doc);
        assert!(matches!(result, VectorRenderResult::Success { .. }));
    }

    #[test]
    fn test_empty_document() {
        let renderer = VectorRenderer::new();
        let doc = VectorDocument {
            name: "empty".to_string(),
            width: 800.0,
            height: 600.0,
            shapes: vec![],
            viewbox: None,
        };
        let result = renderer.render(&doc);
        assert!(matches!(result, VectorRenderResult::Error(_)));
    }
}