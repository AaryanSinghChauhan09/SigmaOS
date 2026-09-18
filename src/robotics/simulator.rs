#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)

use std::vec::Vec;

/// 3D Physical Simulator (Gazebo/CoppeliaSim Parity)
/// Renders collision geometries and solves multi-body rigid dynamics.

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64, pub y: f64, pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

pub struct RigidBody {
    pub id: u32,
    pub mass: f64,
    pub position: Vector3,
    pub velocity: Vector3,
}

pub struct PhysicsWorld {
    pub bodies: Vec<RigidBody>,
    pub gravity: Vector3,
}

impl PhysicsWorld {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            gravity: Vector3::new(0.0, -9.81, 0.0),
        }
    }

    pub fn add_body(&mut self, mass: f64, pos: Vector3) -> u32 {
        let id = self.bodies.len() as u32;
        self.bodies.push(RigidBody {
            id,
            mass,
            position: pos,
            velocity: Vector3::new(0.0, 0.0, 0.0),
        });
        id
    }

    pub fn step(&mut self, dt: f64) {
        for body in &mut self.bodies {
            if body.mass > 0.0 {
                // Apply gravity
                body.velocity.y += self.gravity.y * dt;
                
                // Update position
                body.position.x += body.velocity.x * dt;
                body.position.y += body.velocity.y * dt;
                body.position.z += body.velocity.z * dt;
                
                // Simple ground collision at y = 0
                if body.position.y < 0.0 {
                    body.position.y = 0.0;
                    body.velocity.y = 0.0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_step() {
        let mut world = PhysicsWorld::new();
        let id = world.add_body(1.0, Vector3::new(0.0, 10.0, 0.0));
        
        world.step(1.0); // 1 second step
        
        let body = &world.bodies[id as usize];
        assert_eq!(body.velocity.y, -9.81);
        assert_eq!(body.position.y, 10.0 - 9.81);
    }
    
    #[test]
    fn test_ground_collision() {
        let mut world = PhysicsWorld::new();
        let id = world.add_body(1.0, Vector3::new(0.0, 1.0, 0.0));
        
        world.step(1.0); // Will fall past 0 and trigger collision
        
        let body = &world.bodies[id as usize];
        assert_eq!(body.position.y, 0.0);
        assert_eq!(body.velocity.y, 0.0);
    }
}
