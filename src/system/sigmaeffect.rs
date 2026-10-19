//! SigmaEffect - Effects and Filters System
//!
//! Provides comprehensive effects and filters management inspired by Effect Craft.
//! Supports image filters, visual effects, and post-processing.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Effect type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    Blur,
    Sharpen,
    Brightness,
    Contrast,
    Saturation,
    HueRotate,
    Grayscale,
    Sepia,
    Invert,
    Noise,
    Emboss,
    EdgeDetect,
}

/// Effect parameter
#[derive(Debug, Clone)]
pub struct EffectParameter {
    pub name: String,
    pub value: f32,
}

impl EffectParameter {
    pub fn new(name: String, value: f32) -> Self {
        Self { name, value }
    }
}

/// Effect
#[derive(Debug, Clone)]
pub struct Effect {
    pub id: String,
    pub name: String,
    pub effect_type: EffectType,
    pub parameters: Vec<EffectParameter>,
    pub enabled: bool,
}

impl Effect {
    pub fn new(id: String, name: String, effect_type: EffectType) -> Self {
        Self {
            id,
            name,
            effect_type,
            parameters: Vec::new(),
            enabled: true,
        }
    }

    pub fn add_parameter(&mut self, param: EffectParameter) {
        self.parameters.push(param);
    }

    pub fn set_parameter(&mut self, name: &str, value: f32) -> bool {
        if let Some(param) = self.parameters.iter_mut().find(|p| p.name == name) {
            param.value = value;
            true
        } else {
            false
        }
    }

    pub fn get_parameter(&self, name: &str) -> Option<f32> {
        self.parameters
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.value)
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Effect chain
#[derive(Debug, Clone)]
pub struct EffectChain {
    pub id: String,
    pub name: String,
    pub effects: Vec<String>,
    pub blend_mode: String,
}

impl EffectChain {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            effects: Vec::new(),
            blend_mode: "normal".to_string(),
        }
    }

    pub fn add_effect(&mut self, effect_id: String) {
        self.effects.push(effect_id);
    }

    pub fn remove_effect(&mut self, index: usize) -> bool {
        if index < self.effects.len() {
            self.effects.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_blend_mode(&mut self, blend_mode: String) {
        self.blend_mode = blend_mode;
    }

    pub fn effect_count(&self) -> usize {
        self.effects.len()
    }
}

/// Effect preset
#[derive(Debug, Clone)]
pub struct EffectPreset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub effect_ids: Vec<String>,
}

impl EffectPreset {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            effect_ids: Vec::new(),
        }
    }

    pub fn add_effect(&mut self, effect_id: String) {
        self.effect_ids.push(effect_id);
    }

    pub fn effect_count(&self) -> usize {
        self.effect_ids.len()
    }
}

/// SigmaEffect - Effects system manager
#[derive(Debug)]
pub struct SigmaEffect {
    effects: BTreeMap<String, Effect>,
    chains: BTreeMap<String, EffectChain>,
    presets: BTreeMap<String, EffectPreset>,
}

impl SigmaEffect {
    pub fn new() -> Self {
        Self {
            effects: BTreeMap::new(),
            chains: BTreeMap::new(),
            presets: BTreeMap::new(),
        }
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.insert(effect.id.clone(), effect);
    }

    pub fn remove_effect(&mut self, id: &str) -> bool {
        self.effects.remove(id).is_some()
    }

    pub fn get_effect(&self, id: &str) -> Option<&Effect> {
        self.effects.get(id)
    }

    pub fn get_effect_mut(&mut self, id: &str) -> Option<&mut Effect> {
        self.effects.get_mut(id)
    }

    pub fn add_chain(&mut self, chain: EffectChain) {
        self.chains.insert(chain.id.clone(), chain);
    }

    pub fn remove_chain(&mut self, id: &str) -> bool {
        self.chains.remove(id).is_some()
    }

    pub fn get_chain(&self, id: &str) -> Option<&EffectChain> {
        self.chains.get(id)
    }

    pub fn get_chain_mut(&mut self, id: &str) -> Option<&mut EffectChain> {
        self.chains.get_mut(id)
    }

    pub fn add_preset(&mut self, preset: EffectPreset) {
        self.presets.insert(preset.id.clone(), preset);
    }

    pub fn remove_preset(&mut self, id: &str) -> bool {
        self.presets.remove(id).is_some()
    }

    pub fn get_preset(&self, id: &str) -> Option<&EffectPreset> {
        self.presets.get(id)
    }

    pub fn list_effects(&self) -> Vec<&Effect> {
        self.effects.values().collect()
    }

    pub fn list_chains(&self) -> Vec<&EffectChain> {
        self.chains.values().collect()
    }

    pub fn list_presets(&self) -> Vec<&EffectPreset> {
        self.presets.values().collect()
    }

    pub fn effect_count(&self) -> usize {
        self.effects.len()
    }

    pub fn chain_count(&self) -> usize {
        self.chains.len()
    }

    pub fn preset_count(&self) -> usize {
        self.presets.len()
    }
}

impl Default for SigmaEffect {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaEffect instance
static mut SIGMAEFFECT_INSTANCE: Option<SigmaEffect> = None;

/// Initialize the global SigmaEffect instance
pub fn init_sigmaeffect() {
    unsafe {
        SIGMAEFFECT_INSTANCE = Some(SigmaEffect::new());
    }
}

/// Get the global SigmaEffect instance
pub fn get_sigmaeffect() -> Option<&'static SigmaEffect> {
    unsafe { SIGMAEFFECT_INSTANCE.as_ref() }
}

/// Get the global SigmaEffect instance (mutable)
pub fn get_sigmaeffect_mut() -> Option<&'static mut SigmaEffect> {
    unsafe { SIGMAEFFECT_INSTANCE.as_mut() }
}

/// Convenience function to add an effect
pub fn add_effect(effect: Effect) -> bool {
    if let Some(fx) = get_sigmaeffect_mut() {
        fx.add_effect(effect);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_creation() {
        let effect = Effect::new("effect1".to_string(), "Blur".to_string(), EffectType::Blur);
        assert_eq!(effect.id, "effect1");
        assert_eq!(effect.effect_type, EffectType::Blur);
        assert!(effect.enabled);
    }

    #[test]
    fn test_effect_parameter_management() {
        let mut effect = Effect::new(
            "effect1".to_string(),
            "Brightness".to_string(),
            EffectType::Brightness,
        );
        effect.add_parameter(EffectParameter::new("intensity".to_string(), 0.5));
        assert_eq!(effect.parameters.len(), 1);
        assert!(effect.set_parameter("intensity", 0.8));
        assert_eq!(effect.get_parameter("intensity"), Some(0.8));
    }

    #[test]
    fn test_effect_chain_creation() {
        let chain = EffectChain::new("chain1".to_string(), "Post-processing".to_string());
        assert_eq!(chain.id, "chain1");
        assert_eq!(chain.blend_mode, "normal");
    }

    #[test]
    fn test_effect_chain_management() {
        let mut chain = EffectChain::new("chain1".to_string(), "Test".to_string());
        chain.add_effect("effect1".to_string());
        chain.add_effect("effect2".to_string());
        assert_eq!(chain.effect_count(), 2);
        assert!(chain.remove_effect(0));
        assert_eq!(chain.effect_count(), 1);
    }

    #[test]
    fn test_effect_preset_creation() {
        let preset = EffectPreset::new(
            "preset1".to_string(),
            "Vintage".to_string(),
            "Vintage film look".to_string(),
        );
        assert_eq!(preset.id, "preset1");
        assert_eq!(preset.name, "Vintage");
    }

    #[test]
    fn test_sigmaeffect_creation() {
        let fx = SigmaEffect::new();
        assert_eq!(fx.effect_count(), 0);
        assert_eq!(fx.chain_count(), 0);
        assert_eq!(fx.preset_count(), 0);
    }

    #[test]
    fn test_sigmaeffect_management() {
        let mut fx = SigmaEffect::new();
        let effect = Effect::new("effect1".to_string(), "Blur".to_string(), EffectType::Blur);
        fx.add_effect(effect);
        assert_eq!(fx.effect_count(), 1);
    }
}
