// Vector Graphics Engine (SigmaDraw)
// Defines 2D paths, Bézier curves, layers, groups, masks, SVG import/export, and real-time path manipulation.
// Inspiration: Inkscape, Blender's grease pencil.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;

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

#[derive(Debug, Clone, PartialEq)]
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

    /// Real-time manipulate a specific control point of the bezier curve
    pub fn manipulate_control_point(&mut self, command_idx: usize, control_num: u8, new_pos: Point2D) -> bool {
        if command_idx >= self.commands.len() {
            return false;
        }
        match &mut self.commands[command_idx] {
            PathCommand::MoveTo(p) => {
                if control_num == 0 {
                    *p = new_pos;
                    true
                } else {
                    false
                }
            }
            PathCommand::LineTo(p) => {
                if control_num == 0 {
                    *p = new_pos;
                    true
                } else {
                    false
                }
            }
            PathCommand::CubicBezier { control1, control2, end } => {
                match control_num {
                    1 => { *control1 = new_pos; true }
                    2 => { *control2 = new_pos; true }
                    3 => { *end = new_pos; true }
                    _ => false,
                }
            }
            PathCommand::ClosePath => false,
        }
    }
}

impl Default for VectorPath {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents vector layers
pub struct VectorLayer {
    pub name: String,
    pub visible: bool,
    pub paths: Vec<VectorPath>,
    pub mask: Option<VectorMask>,
}

impl VectorLayer {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            visible: true,
            paths: Vec::new(),
            mask: None,
        }
    }
}

/// Represents vector masks used to crop/restrict rendering region
pub struct VectorMask {
    pub clip_path: VectorPath,
}

/// Represents vector groups grouping multiple layers or paths together
pub struct VectorGroup {
    pub name: String,
    pub layers: Vec<VectorLayer>,
}

impl VectorGroup {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            layers: Vec::new(),
        }
    }
}

/// SVG import/export support parser
pub struct SvgConverter;

impl SvgConverter {
    /// Export list of paths into standard SVG XML string
    pub fn export_to_svg(paths: &[VectorPath]) -> String {
        let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">\n");
        for path in paths {
            svg.push_str("  <path d=\"");
            for (idx, cmd) in path.commands.iter().enumerate() {
                if idx > 0 {
                    svg.push(' ');
                }
                match cmd {
                    PathCommand::MoveTo(p) => {
                        svg.push_str(&format!("M {} {}", p.x, p.y));
                    }
                    PathCommand::LineTo(p) => {
                        svg.push_str(&format!("L {} {}", p.x, p.y));
                    }
                    PathCommand::CubicBezier { control1, control2, end } => {
                        svg.push_str(&format!("C {} {}, {} {}, {} {}", control1.x, control1.y, control2.x, control2.y, end.x, end.y));
                    }
                    PathCommand::ClosePath => {
                        svg.push('Z');
                    }
                }
            }
            svg.push_str("\" fill=\"none\" stroke=\"black\" />\n");
        }
        svg.push_str("</svg>");
        svg
    }

    /// Import/parse from a simple mock SVG syntax
    pub fn import_from_svg(svg_str: &str) -> Vec<VectorPath> {
        let mut paths = Vec::new();
        // Look for simple path d="..." attributes
        let mut start_idx = 0;
        while let Some(d_pos) = svg_str[start_idx..].find("d=\"") {
            let actual_pos = start_idx + d_pos + 3;
            if let Some(end_pos) = svg_str[actual_pos..].find('\"') {
                let d_content = &svg_str[actual_pos..actual_pos + end_pos];
                // Parse a mock path (e.g. "M 10 20 L 30 40 Z")
                let mut path = VectorPath::new();
                let parts: Vec<&str> = d_content.split_whitespace().collect();
                let mut i = 0;
                while i < parts.len() {
                    match parts[i] {
                        "M" => {
                            if i + 2 < parts.len() {
                                let x: f64 = parts[i+1].parse().unwrap_or(0.0);
                                let y: f64 = parts[i+2].parse().unwrap_or(0.0);
                                path.move_to(x, y);
                                i += 3;
                            } else { i += 1; }
                        }
                        "L" => {
                            if i + 2 < parts.len() {
                                let x: f64 = parts[i+1].parse().unwrap_or(0.0);
                                let y: f64 = parts[i+2].parse().unwrap_or(0.0);
                                path.line_to(x, y);
                                i += 3;
                            } else { i += 1; }
                        }
                        "Z" => {
                            path.close();
                            i += 1;
                        }
                        _ => i += 1,
                    }
                }
                paths.push(path);
                start_idx = actual_pos + end_pos + 1;
            } else {
                break;
            }
        }
        paths
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
            SceneNode::RigidBody2D { position, velocity, mass: _, children, .. } => {
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
