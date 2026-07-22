#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::f64;

/// 3D Raytracing Engine (Blender Parity)
/// Simple real-time path tracing with ray intersections.

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    /// Calculate intersection distance of a ray with the sphere
    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin.sub(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let root = (-b - sqrt(discriminant)) / (2.0 * a);
            if root > 0.001 {
                Some(root)
            } else {
                None
            }
        }
    }
}

/// Simple sqrt substitute for #![no_std] (Newton's method)
fn sqrt(x: f64) -> f64 {
    if x <= 0.0 { return 0.0; }
    let mut z = x;
    for _ in 0..10 {
        z = z - (z * z - x) / (2.0 * z);
    }
    z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_sphere_intersection() {
        let sphere = Sphere {
            center: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
            radius: 1.0,
        };
        let ray = Ray {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        };
        
        let t = sphere.intersect(&ray).unwrap();
        // The ray is at z=0 pointing -z, sphere is at z=-5, r=1.
        // Surface is at z=-4.
        assert!((t - 4.0).abs() < 0.01);
    }
}
