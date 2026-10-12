// SPDX-License-Identifier: MIT
// SigmaOS - Lightcraft (Lighting Engine Framework)
// Inspired by Blender Cycles, Unreal Engine, Unity
// Provides 3D lighting and rendering capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Light type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Point,
    Directional,
    Spot,
    Area,
    Ambient,
}

/// Light
pub struct Light {
    pub id: u32,
    pub light_type: LightType,
    pub intensity: f32,
    pub color: (f32, f32, f32), // RGB
    pub position: (f32, f32, f32),
    pub enabled: AtomicBool,
}

/// Material
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Material {
    Lambert,
    Phong,
    Blinn,
    PBR,
    Emissive,
}

/// Scene
pub struct Scene {
    pub id: u32,
    pub name: String,
    pub lights: Vec<Light>,
    pub materials: Vec<Material>,
}

/// Light engine
pub struct LightEngine {
    pub scenes: BTreeMap<u32, Scene>,
    pub next_scene_id: AtomicU32,
    pub active: AtomicBool,
}

impl LightEngine {
    pub fn new() -> Self {
        Self {
            scenes: BTreeMap::new(),
            next_scene_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize light engine
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }
    
    /// Create scene
    pub fn create_scene(&mut self, name: String) -> u32 {
        let id = self.next_scene_id.fetch_add(1, Ordering::Relaxed);
        let scene = Scene {
            id,
            name,
            lights: Vec::new(),
            materials: Vec::new(),
        };
        self.scenes.insert(id, scene);
        id
    }
    
    /// Add light to scene
    pub fn add_light(&mut self, scene_id: u32, light_type: LightType, intensity: f32) -> Result<u32, &'static str> {
        let scene = self.scenes.get_mut(&scene_id).ok_or("Scene not found")?;
        let light_id = scene.lights.len() as u32;
        let light = Light {
            id: light_id,
            light_type,
            intensity,
            color: (1.0, 1.0, 1.0),
            position: (0.0, 0.0, 0.0),
            enabled: AtomicBool::new(true),
        };
        scene.lights.push(light);
        Ok(light_id)
    }
    
    /// Set light intensity
    pub fn set_light_intensity(&mut self, scene_id: u32, light_id: u32, intensity: f32) -> Result<(), &'static str> {
        let scene = self.scenes.get_mut(&scene_id).ok_or("Scene not found")?;
        let light = scene.lights.get_mut(light_id as usize).ok_or("Light not found")?;
        light.intensity = intensity;
        Ok(())
    }
    
    /// Set light color
    pub fn set_light_color(&mut self, scene_id: u32, light_id: u32, r: f32, g: f32, b: f32) -> Result<(), &'static str> {
        let scene = self.scenes.get_mut(&scene_id).ok_or("Scene not found")?;
        let light = scene.lights.get_mut(light_id as usize).ok_or("Light not found")?;
        light.color = (r, g, b);
        Ok(())
    }
    
    /// Enable light
    pub fn enable_light(&mut self, scene_id: u32, light_id: u32) -> Result<(), &'static str> {
        let scene = self.scenes.get_mut(&scene_id).ok_or("Scene not found")?;
        let light = scene.lights.get_mut(light_id as usize).ok_or("Light not found")?;
        light.enabled.store(true, Ordering::Relaxed);
        Ok(())
    }
    
    /// Disable light
    pub fn disable_light(&mut self, scene_id: u32, light_id: u32) -> Result<(), &'static str> {
        let scene = self.scenes.get_mut(&scene_id).ok_or("Scene not found")?;
        let light = scene.lights.get_mut(light_id as usize).ok_or("Light not found")?;
        light.enabled.store(false, Ordering::Relaxed);
        Ok(())
    }
    
    /// Get scene
    pub fn get_scene(&self, id: u32) -> Option<&Scene> {
        self.scenes.get(&id)
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for LightEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Global lightcraft
static mut LIGHTCRAFT: Option<LightEngine> = None;

/// Initialize lightcraft
pub fn init_lightcraft() {
    unsafe {
        LIGHTCRAFT = Some(LightEngine::new());
    }
}

/// Get global lightcraft
pub fn get_lightcraft() -> Option<&'static mut LightEngine> {
    unsafe {
        LIGHTCRAFT.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lightcraft_init() {
        let mut engine = LightEngine::new();
        engine.init();
        assert!(engine.is_active());
    }
    
    #[test]
    fn test_create_scene() {
        let mut engine = LightEngine::new();
        engine.init();
        let id = engine.create_scene("Scene1".to_string());
        assert!(engine.get_scene(id).is_some());
    }
    
    #[test]
    fn test_add_light() {
        let mut engine = LightEngine::new();
        engine.init();
        let scene_id = engine.create_scene("Scene1".to_string());
        engine.add_light(scene_id, LightType::Point, 1.0).unwrap();
        assert_eq!(engine.get_scene(scene_id).unwrap().lights.len(), 1);
    }
}
