// SPDX-License-Identifier: MIT
// SigmaOS - Effectcraft (Effect Engine Framework)
// Inspired by Blender compositing, After Effects, OBS Studio
// Provides visual effects and compositing capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Effect type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Blur,
    Sharpen,
    Distortion,
    ColorCorrection,
    Glow,
    Bloom,
    Vignette,
    ChromaticAberration,
    Noise,
    Grain,
}

/// Effect parameter
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

/// Effect instance
pub struct EffectInstance {
    pub effect: Effect,
    pub enabled: AtomicBool,
    pub parameters: Vec<Parameter>,
}

/// Effect engine
pub struct EffectEngine {
    pub effects: BTreeMap<u32, EffectInstance>,
    pub next_effect_id: AtomicU32,
    pub active: AtomicBool,
}

impl EffectEngine {
    pub fn new() -> Self {
        Self {
            effects: BTreeMap::new(),
            next_effect_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize effect engine
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }
    
    /// Add effect
    pub fn add_effect(&mut self, effect: Effect) -> u32 {
        let id = self.next_effect_id.fetch_add(1, Ordering::Relaxed);
        let instance = EffectInstance {
            effect,
            enabled: AtomicBool::new(true),
            parameters: Vec::new(),
        };
        self.effects.insert(id, instance);
        id
    }
    
    /// Enable effect
    pub fn enable_effect(&mut self, effect_id: u32) -> Result<(), &'static str> {
        let effect = self.effects.get_mut(&effect_id).ok_or("Effect not found")?;
        effect.enabled.store(true, Ordering::Relaxed);
        Ok(())
    }
    
    /// Disable effect
    pub fn disable_effect(&mut self, effect_id: u32) -> Result<(), &'static str> {
        let effect = self.effects.get_mut(&effect_id).ok_or("Effect not found")?;
        effect.enabled.store(false, Ordering::Relaxed);
        Ok(())
    }
    
    /// Set parameter
    pub fn set_parameter(&mut self, effect_id: u32, name: String, value: f32) -> Result<(), &'static str> {
        let effect = self.effects.get_mut(&effect_id).ok_or("Effect not found")?;
        // Find parameter by name
        for param in effect.parameters.iter_mut() {
            if param.name == name {
                param.value = value;
                return Ok(());
            }
        }
        // Add new parameter
        effect.parameters.push(Parameter {
            name,
            value,
            min: 0.0,
            max: 100.0,
        });
        Ok(())
    }
    
    /// Remove effect
    pub fn remove_effect(&mut self, effect_id: u32) -> Result<(), &'static str> {
        self.effects.remove(&effect_id).ok_or("Effect not found")?;
        Ok(())
    }
    
    /// Get effect
    pub fn get_effect(&self, id: u32) -> Option<&EffectInstance> {
        self.effects.get(&id)
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for EffectEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Global effectcraft
static mut EFFECTCRAFT: Option<EffectEngine> = None;

/// Initialize effectcraft
pub fn init_effectcraft() {
    unsafe {
        EFFECTCRAFT = Some(EffectEngine::new());
    }
}

/// Get global effectcraft
pub fn get_effectcraft() -> Option<&'static mut EffectEngine> {
    unsafe {
        EFFECTCRAFT.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_effectcraft_init() {
        let mut engine = EffectEngine::new();
        engine.init();
        assert!(engine.is_active());
    }
    
    #[test]
    fn test_add_effect() {
        let mut engine = EffectEngine::new();
        engine.init();
        let id = engine.add_effect(Effect::Blur);
        assert!(engine.get_effect(id).is_some());
    }
    
    #[test]
    fn test_enable_disable_effect() {
        let mut engine = EffectEngine::new();
        engine.init();
        let id = engine.add_effect(Effect::Blur);
        engine.disable_effect(id).unwrap();
        assert!(!engine.get_effect(id).unwrap().enabled.load(Ordering::Relaxed));
        engine.enable_effect(id).unwrap();
        assert!(engine.get_effect(id).unwrap().enabled.load(Ordering::Relaxed));
    }
}
