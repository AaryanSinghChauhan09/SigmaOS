//! Sigma3D - 3D Graphics and Rendering System
//!
//! Provides comprehensive 3D graphics management inspired by 3D Craft and Blender.
//! Supports 3D rendering, scene management, and model handling.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Render state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderState {
    Idle,
    Rendering,
    Completed,
    Error,
    Unknown,
}

/// 3D Model
#[derive(Debug, Clone)]
pub struct Model3D {
    pub id: String,
    pub name: String,
    pub format: String,
    pub vertex_count: u32,
    pub face_count: u32,
}

impl Model3D {
    pub fn new(id: String, name: String, format: String) -> Self {
        Self {
            id,
            name,
            format,
            vertex_count: 0,
            face_count: 0,
        }
    }

    pub fn with_geometry(mut self, vertex_count: u32, face_count: u32) -> Self {
        self.vertex_count = vertex_count;
        self.face_count = face_count;
        self
    }
}

/// Scene
#[derive(Debug, Clone)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub models: Vec<String>,
    pub render_state: RenderState,
}

impl Scene {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            models: Vec::new(),
            render_state: RenderState::Idle,
        }
    }

    pub fn with_model(mut self, model_id: String) -> Self {
        self.models.push(model_id);
        self
    }

    pub fn start_render(&mut self) {
        self.render_state = RenderState::Rendering;
    }

    pub fn complete_render(&mut self) {
        self.render_state = RenderState::Completed;
    }
}

/// 3D manager
pub struct Sigma3D {
    models: BTreeMap<String, Model3D>,
    scenes: BTreeMap<String, Scene>,
    next_model_id: u64,
    next_scene_id: u64,
}

impl Sigma3D {
    pub fn new() -> Self {
        Self {
            models: BTreeMap::new(),
            scenes: BTreeMap::new(),
            next_model_id: 1,
            next_scene_id: 1,
        }
    }

    pub fn add_model(&mut self, name: String, format: String) -> String {
        let id = format!("model-{}", self.next_model_id);
        self.next_model_id += 1;
        let model = Model3D::new(id.clone(), name, format);
        self.models.insert(id.clone(), model);
        id
    }

    pub fn get_model(&self, id: &str) -> Option<&Model3D> {
        self.models.get(id)
    }

    pub fn list_models(&self) -> Vec<&Model3D> {
        self.models.values().collect()
    }

    pub fn remove_model(&mut self, id: &str) -> bool {
        self.models.remove(id).is_some()
    }

    pub fn add_scene(&mut self, name: String) -> String {
        let id = format!("scene-{}", self.next_scene_id);
        self.next_scene_id += 1;
        let scene = Scene::new(id.clone(), name);
        self.scenes.insert(id.clone(), scene);
        id
    }

    pub fn get_scene(&self, id: &str) -> Option<&Scene> {
        self.scenes.get(id)
    }

    pub fn get_scene_mut(&mut self, id: &str) -> Option<&mut Scene> {
        self.scenes.get_mut(id)
    }

    pub fn list_scenes(&self) -> Vec<&Scene> {
        self.scenes.values().collect()
    }

    pub fn remove_scene(&mut self, id: &str) -> bool {
        self.scenes.remove(id).is_some()
    }

    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    pub fn scene_count(&self) -> usize {
        self.scenes.len()
    }
}

impl Default for Sigma3D {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMA3D_INSTANCE: Option<Sigma3D> = None;

pub fn init_sigma3d() {
    unsafe {
        SIGMA3D_INSTANCE = Some(Sigma3D::new());
    }
}

pub fn get_sigma3d() -> &'static Sigma3D {
    unsafe {
        SIGMA3D_INSTANCE
            .as_ref()
            .expect("Sigma3D not initialized")
    }
}

pub fn get_sigma3d_mut() -> &'static mut Sigma3D {
    unsafe {
        SIGMA3D_INSTANCE
            .as_mut()
            .expect("Sigma3D not initialized")
    }
}

pub fn add_model(name: String, format: String) -> String {
    let manager = get_sigma3d_mut();
    manager.add_model(name, format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model3d_creation() {
        let model = Model3D::new("model-1".to_string(), "Cube".to_string(), "OBJ".to_string());
        assert_eq!(model.name, "Cube");
        assert_eq!(model.format, "OBJ");
    }

    #[test]
    fn test_model3d_with_geometry() {
        let model = Model3D::new("model-1".to_string(), "Cube".to_string(), "OBJ".to_string())
            .with_geometry(1000, 2000);
        assert_eq!(model.vertex_count, 1000);
        assert_eq!(model.face_count, 2000);
    }

    #[test]
    fn test_scene_creation() {
        let scene = Scene::new("scene-1".to_string(), "Main Scene".to_string());
        assert_eq!(scene.name, "Main Scene");
        assert_eq!(scene.render_state, RenderState::Idle);
    }

    #[test]
    fn test_scene_render_state() {
        let mut scene = Scene::new("scene-1".to_string(), "Main Scene".to_string());
        scene.start_render();
        assert_eq!(scene.render_state, RenderState::Rendering);

        scene.complete_render();
        assert_eq!(scene.render_state, RenderState::Completed);
    }

    #[test]
    fn test_sigma3d_manager() {
        let manager = Sigma3D::new();
        assert_eq!(manager.model_count(), 0);
        assert_eq!(manager.scene_count(), 0);
    }

    #[test]
    fn test_add_model() {
        let mut manager = Sigma3D::new();
        let id = manager.add_model("Cube".to_string(), "OBJ".to_string());
        assert_eq!(id, "model-1");
        assert_eq!(manager.model_count(), 1);
    }

    #[test]
    fn test_add_scene() {
        let mut manager = Sigma3D::new();
        let id = manager.add_scene("Main Scene".to_string());
        assert_eq!(id, "scene-1");
        assert_eq!(manager.scene_count(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigma3d();
        let manager = get_sigma3d();
        assert_eq!(manager.model_count(), 0);

        let manager_mut = get_sigma3d_mut();
        manager_mut.add_model("Cube".to_string(), "OBJ".to_string());
        assert_eq!(manager.model_count(), 1);
    }

    #[test]
    fn test_add_model_convenience() {
        init_sigma3d();
        let id = add_model("Cube".to_string(), "OBJ".to_string());
        assert_eq!(id, "model-1");

        let manager = get_sigma3d();
        assert!(manager.get_model(&id).is_some());
    }
}
