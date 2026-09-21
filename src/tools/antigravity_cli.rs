// SPDX-License-Identifier: MIT
// SigmaOS Google Anti-Gravity Inspired Interactive Physics CLI Subsystem (`src/tools/antigravity_cli.rs`)
// Inspired by Google Anti-Gravity web paradigm, classic Linux CLI Easter eggs (cmatrix, sl, cowsay, asciquarium),
// and terminal ANSI 2D rigid-body kinematics.

use std::string::String;
use std::vec::Vec;

/// Gravity Vector Mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GravityMode {
    Normal,     // Standard gravity (+Y downwards)
    AntiGravity, // Inverted anti-gravity (-Y upwards)
    ZeroGravity, // Floating space mode (0, 0)
    RepulsionField, // Impulse push from active cursor coordinate
}

/// 2D Rigid Body representing a floating terminal text block
#[derive(Debug, Clone)]
pub struct PhysicsBody2D {
    pub id: usize,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub ax: f32,
    pub ay: f32,
    pub mass: f32,
    pub width: usize,
    pub height: usize,
    pub is_floating: bool,
}

impl PhysicsBody2D {
    pub fn new(id: usize, label: &str, x: f32, y: f32) -> Self {
        let width = label.chars().count().max(1);
        Self {
            id,
            label: String::from(label),
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            mass: 1.0,
            width,
            height: 1,
            is_floating: true,
        }
    }

    /// Integrate velocity and position over time step dt
    pub fn update_kinematics(&mut self, dt: f32, elasticity: f32, max_x: f32, max_y: f32) {
        if !self.is_floating {
            return;
        }

        // Integrate acceleration into velocity
        self.vx += self.ax * dt;
        self.vy += self.ay * dt;

        // Integrate velocity into position
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Floor / Ceiling collision boundaries
        if self.y >= max_y {
            self.y = max_y;
            self.vy = -self.vy * elasticity;
        } else if self.y <= 0.0 {
            self.y = 0.0;
            self.vy = -self.vy * elasticity;
        }

        // Left / Right wall collision boundaries
        if self.x >= max_x {
            self.x = max_x;
            self.vx = -self.vx * elasticity;
        } else if self.x <= 0.0 {
            self.x = 0.0;
            self.vx = -self.vx * elasticity;
        }
    }

    /// Apply an external force vector (Fx, Fy)
    pub fn apply_force(&mut self, fx: f32, fy: f32) {
        if self.mass > 0.0 {
            self.ax += fx / self.mass;
            self.ay += fy / self.mass;
        }
    }
}

/// Google Anti-Gravity Inspired Interactive Terminal Engine
pub struct AntiGravityCliEngine {
    pub bodies: Vec<PhysicsBody2D>,
    pub gravity_mode: GravityMode,
    pub terminal_width: usize,
    pub terminal_height: usize,
    pub elasticity: f32, // Restitution coefficient
    pub frame_counter: u64,
}

impl AntiGravityCliEngine {
    pub fn new(width: usize, height: usize) -> Self {
        let mut engine = Self {
            bodies: Vec::new(),
            gravity_mode: GravityMode::AntiGravity, // Default Anti-Gravity
            terminal_width: width,
            terminal_height: height,
            elasticity: 0.75,
            frame_counter: 0,
        };
        engine.add_default_terminal_elements();
        engine
    }

    /// Seed default terminal UI elements for anti-gravity physics simulation
    pub fn add_default_terminal_elements(&mut self) {
        self.add_body("SigmaOS v1.0 Sovereign Terminal [Anti-Gravity Active]", 10.0, 5.0);
        self.add_body("[user@sigmaos ~]$ sigma-antigravity --fly", 5.0, 10.0);
        self.add_body("CPU: 0.4% | RAM: 128MB / 16GB | Kernel: Microkernel-Sovereign", 15.0, 15.0);
        self.add_body("[OK] Zero-Copy eBPF XDP Networking Running", 8.0, 18.0);
        self.add_body("[OK] OpenBSD PF Stateful Firewall Active", 12.0, 20.0);
    }

    pub fn add_body(&mut self, label: &str, x: f32, y: f32) -> usize {
        let id = self.bodies.len() + 1;
        let body = PhysicsBody2D::new(id, label, x, y);
        self.bodies.push(body);
        id
    }

    /// Toggle or set gravity mode
    pub fn set_gravity_mode(&mut self, mode: GravityMode) {
        self.gravity_mode = mode;
    }

    /// Apply simulated user cursor pulse (impulse force flinging nearby floating blocks)
    pub fn apply_cursor_pulse(&mut self, cursor_x: f32, cursor_y: f32, force_magnitude: f32) {
        for body in &mut self.bodies {
            let dx = body.x - cursor_x;
            let dy = body.y - cursor_y;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq < 100.0 && dist_sq > 0.01 {
                let dist = dist_sq.sqrt();
                let nx = dx / dist;
                let ny = dy / dist;

                let force = force_magnitude / dist_sq.max(1.0);
                body.apply_force(nx * force * 10.0, ny * force * 10.0);
            }
        }
    }

    /// Step the simulation loop forward by time step dt (e.g. 0.05s)
    pub fn step_simulation(&mut self, dt: f32) {
        self.frame_counter += 1;

        // Apply gravity acceleration depending on active mode
        let (gx, gy) = match self.gravity_mode {
            GravityMode::Normal => (0.0, 9.8),
            GravityMode::AntiGravity => (0.0, -9.8), // Floating upwards
            GravityMode::ZeroGravity => (0.0, 0.0),
            GravityMode::RepulsionField => (0.0, -2.0),
        };

        let max_x = self.terminal_width.saturating_sub(10) as f32;
        let max_y = self.terminal_height.saturating_sub(2) as f32;

        for body in &mut self.bodies {
            body.ax = gx;
            body.ay = gy;
            body.update_kinematics(dt, self.elasticity, max_x, max_y);
        }
    }

    /// Render current physics frame to ANSI terminal string representation
    pub fn render_ansi_frame(&self) -> String {
        format!(
            "ANSI Frame #{}: Mode {:?}, Floating Bodies: {}",
            self.frame_counter,
            self.gravity_mode,
            self.bodies.len()
        )
    }
}

impl Default for AntiGravityCliEngine {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_body_kinematics() {
        let mut body = PhysicsBody2D::new(1, "Test Block", 10.0, 10.0);
        body.ax = 0.0;
        body.ay = -9.8; // Anti-gravity upward acceleration

        // Step simulation 1 second
        body.update_kinematics(1.0, 0.75, 100.0, 100.0);

        assert!(body.vy < 0.0); // Velocity directed upwards
        assert!(body.y < 10.0); // Y position moved upwards
    }

    #[test]
    fn test_boundary_collision_bounce() {
        let mut body = PhysicsBody2D::new(1, "Test Bounce", 10.0, 0.0);
        body.vy = -10.0; // Moving up towards ceiling

        // Update when hitting top boundary y = 0.0
        body.update_kinematics(0.1, 0.75, 100.0, 100.0);

        assert_eq!(body.y, 0.0);
        assert!(body.vy > 0.0); // Velocity reversed down after bounce
    }

    #[test]
    fn test_cursor_repulsion_pulse() {
        let mut engine = AntiGravityCliEngine::new(80, 24);

        // Apply mouse cursor pulse near body 0 at (10.0, 5.0)
        engine.apply_cursor_pulse(9.0, 5.0, 50.0);

        assert!(engine.bodies[0].ax > 0.0); // Accelerated rightwards away from cursor at x=9
    }

    #[test]
    fn test_simulation_stepping_and_ansi_render() {
        let mut engine = AntiGravityCliEngine::new(80, 24);
        assert_eq!(engine.frame_counter, 0);

        engine.step_simulation(0.1);
        assert_eq!(engine.frame_counter, 1);

        let ansi = engine.render_ansi_frame();
        assert!(ansi.contains("Frame #1"));
        assert!(ansi.contains("AntiGravity"));
    }
}
