//! SigmaLight - Lighting and Color Management System
//!
//! Provides comprehensive lighting and color management inspired by Light Craft.
//! Supports light sources, color temperature, brightness control, and lighting scenes.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Light type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Ambient,
    Directional,
    Point,
    Spot,
    Area,
}

/// Color temperature (in Kelvin)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorTemperature {
    Warm(2700),
    Neutral(4000),
    Cool(5000),
    Daylight(6500),
    Custom(u32),
}

/// Light source
#[derive(Debug, Clone)]
pub struct LightSource {
    pub id: String,
    pub name: String,
    pub light_type: LightType,
    pub brightness: u8, // 0-255
    pub color_temperature: ColorTemperature,
    pub color_rgb: (u8, u8, u8),
    pub enabled: bool,
}

impl LightSource {
    pub fn new(id: String, name: String, light_type: LightType) -> Self {
        Self {
            id,
            name,
            light_type,
            brightness: 255,
            color_temperature: ColorTemperature::Neutral(4000),
            color_rgb: (255, 255, 255),
            enabled: true,
        }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness;
    }

    pub fn set_color_temperature(&mut self, temp: ColorTemperature) {
        self.color_temperature = temp;
    }

    pub fn set_color_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.color_rgb = (r, g, b);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn get_normalized_brightness(&self) -> f32 {
        self.brightness as f32 / 255.0
    }
}

/// Lighting scene
#[derive(Debug, Clone)]
pub struct LightingScene {
    pub id: String,
    pub name: String,
    pub light_states: BTreeMap<String, (u8, ColorTemperature, (u8, u8, u8))>,
}

impl LightingScene {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            light_states: BTreeMap::new(),
        }
    }

    pub fn add_light_state(&mut self, light_id: String, brightness: u8, temp: ColorTemperature, rgb: (u8, u8, u8)) {
        self.light_states.insert(light_id, (brightness, temp, rgb));
    }

    pub fn remove_light_state(&mut self, light_id: &str) -> bool {
        self.light_states.remove(light_id).is_some()
    }

    pub fn get_light_state(&self, light_id: &str) -> Option<&(u8, ColorTemperature, (u8, u8, u8))> {
        self.light_states.get(light_id)
    }

    pub fn light_count(&self) -> usize {
        self.light_states.len()
    }
}

/// SigmaLight - Lighting system manager
#[derive(Debug)]
pub struct SigmaLight {
    lights: BTreeMap<String, LightSource>,
    scenes: BTreeMap<String, LightingScene>,
    current_scene: Option<String>,
}

impl SigmaLight {
    pub fn new() -> Self {
        Self {
            lights: BTreeMap::new(),
            scenes: BTreeMap::new(),
            current_scene: None,
        }
    }

    pub fn add_light(&mut self, light: LightSource) {
        self.lights.insert(light.id.clone(), light);
    }

    pub fn remove_light(&mut self, id: &str) -> bool {
        self.lights.remove(id).is_some()
    }

    pub fn get_light(&self, id: &str) -> Option<&LightSource> {
        self.lights.get(id)
    }

    pub fn get_light_mut(&mut self, id: &str) -> Option<&mut LightSource> {
        self.lights.get_mut(id)
    }

    pub fn add_scene(&mut self, scene: LightingScene) {
        self.scenes.insert(scene.id.clone(), scene);
    }

    pub fn remove_scene(&mut self, id: &str) -> bool {
        if self.current_scene.as_ref() == Some(&id.to_string()) {
            self.current_scene = None;
        }
        self.scenes.remove(id).is_some()
    }

    pub fn get_scene(&self, id: &str) -> Option<&LightingScene> {
        self.scenes.get(id)
    }

    pub fn set_current_scene(&mut self, id: &str) -> bool {
        if self.scenes.contains_key(id) {
            self.current_scene = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_scene(&self) -> Option<&LightingScene> {
        self.current_scene.as_ref()
            .and_then(|id| self.scenes.get(id))
    }

    pub fn apply_scene(&mut self, scene_id: &str) -> bool {
        if let Some(scene) = self.scenes.get(scene_id) {
            for (light_id, (brightness, temp, rgb)) in &scene.light_states {
                if let Some(light) = self.lights.get_mut(light_id) {
                    light.brightness = *brightness;
                    light.color_temperature = *temp;
                    light.color_rgb = *rgb;
                }
            }
            self.current_scene = Some(scene_id.to_string());
            true
        } else {
            false
        }
    }

    pub fn list_lights(&self) -> Vec<&LightSource> {
        self.lights.values().collect()
    }

    pub fn list_scenes(&self) -> Vec<&LightingScene> {
        self.scenes.values().collect()
    }

    pub fn light_count(&self) -> usize {
        self.lights.len()
    }

    pub fn scene_count(&self) -> usize {
        self.scenes.len()
    }
}

impl Default for SigmaLight {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaLight instance
static mut SIGMALIGHT_INSTANCE: Option<SigmaLight> = None;

/// Initialize the global SigmaLight instance
pub fn init_sigmalight() {
    unsafe {
        SIGMALIGHT_INSTANCE = Some(SigmaLight::new());
    }
}

/// Get the global SigmaLight instance
pub fn get_sigmalight() -> Option<&'static SigmaLight> {
    unsafe {
        SIGMALIGHT_INSTANCE.as_ref()
    }
}

/// Get the global SigmaLight instance (mutable)
pub fn get_sigmalight_mut() -> Option<&'static mut SigmaLight> {
    unsafe {
        SIGMALIGHT_INSTANCE.as_mut()
    }
}

/// Convenience function to add a light
pub fn add_light(light: LightSource) -> bool {
    if let Some(light_sys) = get_sigmalight_mut() {
        light_sys.add_light(light);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_source_creation() {
        let light = LightSource::new("light1".to_string(), "Living Room".to_string(), LightType::Point);
        assert_eq!(light.id, "light1");
        assert_eq!(light.light_type, LightType::Point);
        assert_eq!(light.brightness, 255);
        assert!(light.enabled);
    }

    #[test]
    fn test_light_source_modification() {
        let mut light = LightSource::new("light1".to_string(), "Test".to_string(), LightType::Point);
        light.set_brightness(128);
        light.set_color_temperature(ColorTemperature::Warm(2700));
        light.set_color_rgb(255, 200, 150);
        assert_eq!(light.brightness, 128);
        assert_eq!(light.get_normalized_brightness(), 128.0 / 255.0);
    }

    #[test]
    fn test_lighting_scene_creation() {
        let scene = LightingScene::new("scene1".to_string(), "Evening".to_string());
        assert_eq!(scene.id, "scene1");
        assert_eq!(scene.light_count(), 0);
    }

    #[test]
    fn test_lighting_scene_management() {
        let mut scene = LightingScene::new("scene1".to_string(), "Test".to_string());
        scene.add_light_state("light1".to_string(), 200, ColorTemperature::Warm(2700), (255, 200, 150));
        assert_eq!(scene.light_count(), 1);
        assert!(scene.remove_light_state("light1"));
        assert_eq!(scene.light_count(), 0);
    }

    #[test]
    fn test_sigmalight_creation() {
        let light_sys = SigmaLight::new();
        assert_eq!(light_sys.light_count(), 0);
        assert_eq!(light_sys.scene_count(), 0);
    }

    #[test]
    fn test_sigmalight_management() {
        let mut light_sys = SigmaLight::new();
        let light = LightSource::new("light1".to_string(), "Test".to_string(), LightType::Point);
        light_sys.add_light(light);
        assert_eq!(light_sys.light_count(), 1);
    }

    #[test]
    fn test_scene_application() {
        let mut light_sys = SigmaLight::new();
        let mut light = LightSource::new("light1".to_string(), "Test".to_string(), LightType::Point);
        light.brightness = 255;
        light_sys.add_light(light);

        let mut scene = LightingScene::new("scene1".to_string(), "Dim".to_string());
        scene.add_light_state("light1".to_string(), 50, ColorTemperature::Warm(2700), (255, 200, 150));
        light_sys.add_scene(scene);

        assert!(light_sys.apply_scene("scene1"));
        assert_eq!(light_sys.get_light("light1").unwrap().brightness, 50);
    }
}
