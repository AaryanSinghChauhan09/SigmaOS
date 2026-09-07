use std::vec;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
// SigmaOS Sovereign AI-Native 3D Graphics & Rendering Engine
// Designed for Blender-parity modeling, transforms, shaders, and raytracing

use crate::graphics::paint::ColorRgba;

/// 3D Vector Math Utility
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn add(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn mul(&self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len > 0.0 {
            self.mul(1.0 / len)
        } else {
            Vec3::zero()
        }
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        self.sub(normal.mul(2.0 * self.dot(normal)))
    }
}

/// Mesh vertex / geometry definitions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TriangleFace {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

impl TriangleFace {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        TriangleFace { v0, v1, v2 }
    }

    /// Möller-Trumbore ray-triangle intersection algorithm
    pub fn intersect_ray(&self, ray_orig: Vec3, ray_dir: Vec3) -> Option<f32> {
        let edge1 = self.v1.sub(self.v0);
        let edge2 = self.v2.sub(self.v0);
        let h = ray_dir.cross(edge2);
        let a = edge1.dot(h);

        if a > -1e-6 && a < 1e-6 {
            return None; // Parallel to triangle
        }

        let f = 1.0 / a;
        let s = ray_orig.sub(self.v0);
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray_dir.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);
        if t > 1e-6 {
            Some(t)
        } else {
            None
        }
    }

    pub fn normal(&self) -> Vec3 {
        let edge1 = self.v1.sub(self.v0);
        let edge2 = self.v2.sub(self.v0);
        edge1.cross(edge2).normalize()
    }
}

/// 3D Mesh modeling container representing a polygonal object (like Blender models)
pub struct MeshModel {
    pub name: String,
    pub faces: Vec<TriangleFace>,
}

impl MeshModel {
    pub fn new(name: &str) -> Self {
        MeshModel {
            name: name.to_string(),
            faces: Vec::new(),
        }
    }

    pub fn add_face(&mut self, face: TriangleFace) {
        self.faces.push(face);
    }

    /// Transforms mesh using translation offset
    pub fn translate(&mut self, offset: Vec3) {
        for face in &mut self.faces {
            face.v0 = face.v0.add(offset);
            face.v1 = face.v1.add(offset);
            face.v2 = face.v2.add(offset);
        }
    }

    /// Transforms mesh using scale factor
    pub fn scale(&mut self, factor: f32) {
        for face in &mut self.faces {
            face.v0 = face.v0.mul(factor);
            face.v1 = face.v1.mul(factor);
            face.v2 = face.v2.mul(factor);
        }
    }

    /// Rotates mesh around Y axis (simplistic matrix-less trig rotation)
    pub fn rotate_y(&mut self, radians: f32) {
        let cos_val = radians.cos();
        let sin_val = radians.sin();
        let rot_v = |v: Vec3| -> Vec3 {
            Vec3::new(
                v.x * cos_val + v.z * sin_val,
                v.y,
                -v.x * sin_val + v.z * cos_val,
            )
        };
        for face in &mut self.faces {
            face.v0 = rot_v(face.v0);
            face.v1 = rot_v(face.v1);
            face.v2 = rot_v(face.v2);
        }
    }
}

/// Material shader settings resembling Blender Principled BSDF / Phong shading properties
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialShader {
    pub base_color: ColorRgba,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub roughness: f32, // Phong exponent representation (higher roughness = lower exponent)
}

impl MaterialShader {
    pub fn new(base_color: ColorRgba) -> Self {
        MaterialShader {
            base_color,
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.3,
            roughness: 0.5,
        }
    }
}

/// Render Camera projection mappings
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderCamera {
    pub position: Vec3,
    pub target: Vec3,
    pub fov_degrees: f32,
    pub focal_length: f32,
}

impl RenderCamera {
    pub fn new(position: Vec3, target: Vec3) -> Self {
        RenderCamera {
            position,
            target,
            fov_degrees: 60.0,
            focal_length: 1.0,
        }
    }
}

/// BlenderRenderEngine is the core 3D scene engine.
/// It features raytracer rendering with shading, shadow test validation, and ray intersections.
pub struct BlenderRenderEngine {
    pub models: Vec<MeshModel>,
    pub camera: RenderCamera,
    pub light_source: Vec3,
    pub sky_color: ColorRgba,
}

impl BlenderRenderEngine {
    pub fn new(camera: RenderCamera) -> Self {
        BlenderRenderEngine {
            models: Vec::new(),
            camera,
            light_source: Vec3::new(10.0, 15.0, 10.0),
            sky_color: ColorRgba::new(20, 20, 30, 255),
        }
    }

    pub fn add_model(&mut self, model: MeshModel) {
        self.models.push(model);
    }

    /// Renders the 3D scene onto a 2D viewport
    pub fn render_viewport(&self, width: u32, height: u32, shader: MaterialShader) -> Vec<ColorRgba> {
        let size = (width * height) as usize;
        let mut viewport = vec![self.sky_color; size];

        let cam_dir = self.camera.target.sub(self.camera.position).normalize();
        let up = Vec3::new(0.0, 1.0, 0.0);
        let cam_right = cam_dir.cross(up).normalize();
        let cam_up = cam_right.cross(cam_dir).normalize();

        let aspect_ratio = width as f32 / height as f32;
        let fov_rad = (self.camera.fov_degrees * std::f32::consts::PI / 180.0) * 0.5;
        let half_h = fov_rad.tan();
        let half_w = aspect_ratio * half_h;

        for y in 0..height {
            for x in 0..width {
                // Normalize screen space
                let px = (2.0 * ((x as f32 + 0.5) / width as f32) - 1.0) * half_w;
                let py = (1.0 - 2.0 * ((y as f32 + 0.5) / height as f32)) * half_h;

                let ray_dir = cam_right.mul(px)
                    .add(cam_up.mul(py))
                    .add(cam_dir)
                    .normalize();

                if let Some((t, face, normal)) = self.trace_ray(self.camera.position, ray_dir) {
                    let hit_point = self.camera.position.add(ray_dir.mul(t));
                    let pixel_color = self.shade_pixel(hit_point, normal, shader);
                    viewport[(y * width + x) as usize] = pixel_color;
                }
            }
        }

        viewport
    }

    fn trace_ray(&self, origin: Vec3, dir: Vec3) -> Option<(f32, &TriangleFace, Vec3)> {
        let mut min_t = f32::MAX;
        let mut closest_face = None;

        for model in &self.models {
            for face in &model.faces {
                if let Some(t) = face.intersect_ray(origin, dir) {
                    if t < min_t {
                        min_t = t;
                        closest_face = Some(face);
                    }
                }
            }
        }

        closest_face.map(|face| (min_t, face, face.normal()))
    }

    fn shade_pixel(&self, hit_point: Vec3, normal: Vec3, shader: MaterialShader) -> ColorRgba {
        let light_dir = self.light_source.sub(hit_point).normalize();

        // Shadow ray casting test
        let mut is_shadowed = false;
        let shadow_orig = hit_point.add(normal.mul(1e-3)); // avoid acne
        for model in &self.models {
            for face in &model.faces {
                if face.intersect_ray(shadow_orig, light_dir).is_some() {
                    is_shadowed = true;
                    break;
                }
            }
        }

        if is_shadowed {
            // Ambient factor shading only
            let r = (shader.base_color.r as f32 * shader.ambient) as u8;
            let g = (shader.base_color.g as f32 * shader.ambient) as u8;
            let b = (shader.base_color.b as f32 * shader.ambient) as u8;
            return ColorRgba::new(r, g, b, 255);
        }

        // Diffuse (Lambertian reflection)
        let diffuse_intensity = normal.dot(light_dir).max(0.0) * shader.diffuse;

        // Specular (Phong shading approximation)
        let view_dir = self.camera.position.sub(hit_point).normalize();
        let reflect_dir = light_dir.mul(-1.0).reflect(normal).normalize();
        let specular_intensity = view_dir.dot(reflect_dir).max(0.0)
            .powf(1.0 / shader.roughness.max(0.01)) * shader.specular;

        let total_light = shader.ambient + diffuse_intensity + specular_intensity;

        let r = ((shader.base_color.r as f32 * total_light).min(255.0)) as u8;
        let g = ((shader.base_color.g as f32 * total_light).min(255.0)) as u8;
        let b = ((shader.base_color.b as f32 * total_light).min(255.0)) as u8;

        ColorRgba::new(r, g, b, 255)
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_operations() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v1.add(v2), Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(v1.sub(v2), Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(v1.dot(v2), 32.0);
        assert!((v1.length() - 14.0f32.sqrt()).abs() < 1e-5);
    }

    #[test]
    fn test_triangle_intersection() {
        // Front facing triangle
        let tri = TriangleFace::new(
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(1.0, -1.0, 5.0),
            Vec3::new(0.0, 1.0, 5.0),
        );

        let ray_orig = Vec3::new(0.0, 0.0, 0.0);
        let ray_dir = Vec3::new(0.0, 0.0, 1.0);

        let hit = tri.intersect_ray(ray_orig, ray_dir);
        assert!(hit.is_some());
        assert_eq!(hit.unwrap(), 5.0);
    }

    #[test]
    fn test_mesh_transforms() {
        let mut mesh = MeshModel::new("CubePart");
        mesh.add_face(TriangleFace::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ));

        mesh.translate(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(mesh.faces[0].v0, Vec3::new(1.0, 2.0, 3.0));

        mesh.scale(2.0);
        assert_eq!(mesh.faces[0].v0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_blender_render_viewport() {
        let camera = RenderCamera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 5.0));
        let mut engine = BlenderRenderEngine::new(camera);

        let mut cube = MeshModel::new("MockCube");
        cube.add_face(TriangleFace::new(
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(1.0, -1.0, 5.0),
            Vec3::new(0.0, 1.0, 5.0),
        ));
        engine.add_model(cube);

        let shader = MaterialShader::new(ColorRgba::new(255, 0, 0, 255));
        let pixels = engine.render_viewport(10, 10, shader);
        assert_eq!(pixels.len(), 100);

        // Center pixel should hit the triangle and be shaded red (not sky_color)
        let center_color = pixels[55];
        assert_ne!(center_color, engine.sky_color);
        assert!(center_color.r > 0);
    }
}
