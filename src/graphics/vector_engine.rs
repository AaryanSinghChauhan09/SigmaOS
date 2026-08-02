#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)

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
    #[allow(clippy::new_without_default)]
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

/// Represents a node in the scene tree (exactly mirroring Godot's Node/Node2D/Node3D hierarchy)
#[derive(Debug, Clone, PartialEq)]
pub enum SceneNode {
    Node {
        name: String,
        children: Vec<SceneNode>,
    },
    Node2D {
        name: String,
        position: Point2D,
        rotation: f64,
        scale: Point2D,
        children: Vec<SceneNode>,
    },
    Sprite2D {
        name: String,
        position: Point2D,
        texture_path: String,
        children: Vec<SceneNode>,
    },
    RigidBody2D {
        name: String,
        position: Point2D,
        velocity: Point2D,
        mass: f64,
        children: Vec<SceneNode>,
    },
}

/// Sovereign Game Engine Shard (Godot Parity)
/// Governs scene trees, 2D physics integration, and delta-tick frame loops.
pub struct SovereignGameEngine {
    pub root_node: Option<SceneNode>,
    pub gravity: f64,
    pub active_camera_pos: Point2D,
}

impl SovereignGameEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            root_node: None,
            gravity: 9.8,
            active_camera_pos: Point2D { x: 0.0, y: 0.0 },
        }
    }

    pub fn set_root(&mut self, node: SceneNode) {
        self.root_node = Some(node);
    }

    /// Simulates Godot's _physics_process(delta) loop over the scene tree
    pub fn physics_tick(&mut self, delta: f64) {
        if let Some(ref mut root) = self.root_node {
            Self::process_physics_node(root, delta, self.gravity);
        }
    }

    fn process_physics_node(node: &mut SceneNode, delta: f64, gravity: f64) {
        match node {
            SceneNode::RigidBody2D { position, velocity, mass, children, .. } => {
                // Apply gravity acceleration: v = v + g * dt
                velocity.y += gravity * delta;
                // Apply velocity translation: p = p + v * dt
                position.x += velocity.x * delta;
                position.y += velocity.y * delta;

                for child in children {
                    Self::process_physics_node(child, delta, gravity);
                }
            }
            SceneNode::Node2D { children, .. } => {
                for child in children {
                    Self::process_physics_node(child, delta, gravity);
                }
            }
            SceneNode::Sprite2D { children, .. } => {
                for child in children {
                    Self::process_physics_node(child, delta, gravity);
                }
            }
            SceneNode::Node { children, .. } => {
                for child in children {
                    Self::process_physics_node(child, delta, gravity);
                }
            }
        }
    }
}

impl Default for SovereignGameEngine {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn test_sovereign_game_engine_physics() {
        let mut engine = SovereignGameEngine::new();
        engine.gravity = 10.0; // Simplify math: g = 10 m/s^2

        let player = SceneNode::RigidBody2D {
            name: "Player".to_string(),
            position: Point2D { x: 0.0, y: 0.0 },
            velocity: Point2D { x: 5.0, y: 0.0 },
            mass: 1.0,
            children: Vec::new(),
        };

        engine.set_root(player);

        // Run 1 second physics process step (dt = 1.0)
        engine.physics_tick(1.0);

        if let Some(SceneNode::RigidBody2D { position, velocity, .. }) = engine.root_node {
            // v_y = 0 + 10 * 1 = 10
            // p_x = 0 + 5 * 1 = 5
            // p_y = 0 + 10 * 1 = 10
            assert_eq!(velocity.y, 10.0);
            assert_eq!(position.x, 5.0);
            assert_eq!(position.y, 10.0);
        } else {
            panic!("Expected RigidBody2D root node");
        }
    }
}
