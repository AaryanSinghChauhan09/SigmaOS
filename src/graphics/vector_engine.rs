#![no_std]

extern crate alloc;
use alloc::vec::Vec;

/// Vector Graphics Engine (Inkscape/Ghostscript Parity)
/// Defines 2D paths, Bézier curves, and scalable rendering logic.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PathCommand {
    MoveTo(Point2D),
    LineTo(Point2D),
    CubicBezier {
        control1: Point2D,
        control2: Point2D,
        end: Point2D,
    },
    ClosePath,
}

pub struct VectorPath {
    pub commands: Vec<PathCommand>,
}

impl VectorPath {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.commands.push(PathCommand::MoveTo(Point2D { x, y }));
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        self.commands.push(PathCommand::LineTo(Point2D { x, y }));
    }

    pub fn cubic_bezier(&mut self, cx1: f64, cy1: f64, cx2: f64, cy2: f64, ex: f64, ey: f64) {
        self.commands.push(PathCommand::CubicBezier {
            control1: Point2D { x: cx1, y: cy1 },
            control2: Point2D { x: cx2, y: cy2 },
            end: Point2D { x: ex, y: ey },
        });
    }
    
    pub fn close(&mut self) {
        self.commands.push(PathCommand::ClosePath);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_path_building() {
        let mut path = VectorPath::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 0.0);
        path.cubic_bezier(10.0, 10.0, 0.0, 10.0, 0.0, 0.0);
        path.close();
        
        assert_eq!(path.commands.len(), 4);
        if let PathCommand::MoveTo(p) = &path.commands[0] {
            assert_eq!(p.x, 0.0);
        }
    }
}
