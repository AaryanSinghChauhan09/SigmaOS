// SPDX-License-Identifier: MIT
// SigmaOS - Effectcraft (Effect Engine Framework)
// Inspired by ArtCraft EffectCraft, Blender Compositing, After Effects, OBS Studio
// Provides professional visual effects, compositing, and post-processing capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Blend mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    ColorDodge,
    ColorBurn,
    Darken,
    Lighten,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

/// Effect category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectCategory {
    Blur,
    Sharpen,
    Distortion,
    ColorCorrection,
    Light,
    Stylize,
    Generate,
    Transition,
    Transform,
    Keying,
}

/// Effect type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    // Blur effects
    GaussianBlur,
    MotionBlur,
    RadialBlur,
    BoxBlur,
    LensBlur,
    SurfaceBlur,
    SmartBlur,
    
    // Sharpen effects
    Sharpen,
    UnsharpMask,
    HighPass,
    
    // Distortion effects
    Ripple,
    Wave,
    Displacement,
    Spherize,
    Twirl,
    Liquify,
    PolarCoordinates,
    
    // Color correction
    BrightnessContrast,
    HueSaturation,
    ColorBalance,
    Levels,
    Curves,
    ColorLookup,
    ChannelMixer,
    GradientMap,
    
    // Light effects
    Glow,
    Bloom,
    LensFlare,
    Vignette,
    AmbientOcclusion,
    Shadow,
    Reflection,
    
    // Stylize effects
    Emboss,
    EdgeDetect,
    Posterize,
    Pixelate,
    Mosaic,
    Noise,
    Grain,
    FilmGrain,
    Halftone,
    
    // Generate effects
    Solid,
    Gradient,
    NoisePattern,
    Grid,
    Checkerboard,
    
    // Transition effects
    Dissolve,
    Fade,
    Wipe,
    Iris,
    Slide,
    Zoom,
    
    // Transform effects
    Scale,
    Rotate,
    Translate,
    Skew,
    Perspective,
    
    // Keying effects
    ChromaKey,
    LumaKey,
    DifferenceKey,
    ColorKey,
}

/// Parameter type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
    Float,
    Int,
    Bool,
    Color,
    Vector2,
    Vector3,
    Enum,
}

/// Effect parameter
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: ParameterType,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub default: f32,
}

/// Keyframe
#[derive(Debug, Clone)]
pub struct Keyframe {
    pub frame: u32,
    pub value: f32,
    pub interpolation: InterpolationType,
}

/// Interpolation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationType {
    Linear,
    Bezier,
    Constant,
    EaseIn,
    EaseOut,
    EaseInOut,
}

/// Animated parameter
#[derive(Debug, Clone)]
pub struct AnimatedParameter {
    pub parameter: Parameter,
    pub keyframes: Vec<Keyframe>,
    pub animated: bool,
}

/// Effect instance
pub struct EffectInstance {
    pub effect: Effect,
    pub category: EffectCategory,
    pub enabled: AtomicBool,
    pub blend_mode: BlendMode,
    pub opacity: f32,
    pub parameters: Vec<AnimatedParameter>,
    pub mask_enabled: bool,
}

/// Node type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Input,
    Output,
    Effect,
    Composite,
    Transform,
    ColorCorrect,
    Mask,
}

/// Compositing node
#[derive(Debug, Clone)]
pub struct CompositingNode {
    pub id: u32,
    pub node_type: NodeType,
    pub effect: Option<Effect>,
    pub inputs: Vec<u32>, // input node IDs
    pub outputs: Vec<u32>, // output node IDs
    pub position: (f32, f32),
    pub enabled: AtomicBool,
}

/// Node graph
#[derive(Debug, Clone)]
pub struct NodeGraph {
    pub nodes: BTreeMap<u32, CompositingNode>,
    pub connections: Vec<(u32, u32)>, // (from_node_id, to_node_id)
}

/// Effect engine
pub struct EffectEngine {
    pub effects: BTreeMap<u32, EffectInstance>,
    pub node_graph: NodeGraph,
    pub next_effect_id: AtomicU32,
    pub next_node_id: AtomicU32,
    pub active: AtomicBool,
    pub gpu_acceleration_enabled: AtomicBool,
}

impl EffectEngine {
    pub fn new() -> Self {
        Self {
            effects: BTreeMap::new(),
            node_graph: NodeGraph {
                nodes: BTreeMap::new(),
                connections: Vec::new(),
            },
            next_effect_id: AtomicU32::new(1),
            next_node_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
            gpu_acceleration_enabled: AtomicBool::new(true),
        }
    }

    /// Initialize effect engine
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Add effect
    pub fn add_effect(&mut self, effect: Effect, category: EffectCategory) -> u32 {
        let id = self.next_effect_id.fetch_add(1, Ordering::Relaxed);
        let instance = EffectInstance {
            effect,
            category,
            enabled: AtomicBool::new(true),
            blend_mode: BlendMode::Normal,
            opacity: 1.0,
            parameters: Vec::new(),
            mask_enabled: false,
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

    /// Set blend mode
    pub fn set_blend_mode(&mut self, effect_id: u32, blend_mode: BlendMode) -> Result<(), &'static str> {
        let effect = self.effects.get_mut(&effect_id).ok_or("Effect not found")?;
        effect.blend_mode = blend_mode;
        Ok(())
    }

    /// Set opacity
    pub fn set_opacity(&mut self, effect_id: u32, opacity: f32) -> Result<(), &'static str> {
        let effect = self.effects.get_mut(&effect_id).ok_or("Effect not found")?;
        effect.opacity = opacity.clamp(0.0, 1.0);
        Ok(())
    }

    /// Add parameter
    pub fn add_parameter(&mut self, effect_id: u32, parameter: Parameter) -> Result<(), &'static str> {
        let effect = self.effects.get_mut(&effect_id).ok_or("Effect not found")?;
        effect.parameters.push(AnimatedParameter {
            parameter,
            keyframes: Vec::new(),
            animated: false,
        });
        Ok(())
    }

    /// Set parameter value
    pub fn set_parameter(&mut self, effect_id: u32, name: String, value: f32) -> Result<(), &'static str> {
        let effect = self.effects.get_mut(&effect_id).ok_or("Effect not found")?;
        for param in effect.parameters.iter_mut() {
            if param.parameter.name == name {
                param.parameter.value = value.clamp(param.parameter.min, param.parameter.max);
                return Ok(());
            }
        }
        Err("Parameter not found")
    }

    /// Add keyframe
    pub fn add_keyframe(&mut self, effect_id: u32, param_name: String, frame: u32, value: f32, interpolation: InterpolationType) -> Result<(), &'static str> {
        let effect = self.effects.get_mut(&effect_id).ok_or("Effect not found")?;
        for param in effect.parameters.iter_mut() {
            if param.parameter.name == param_name {
                param.keyframes.push(Keyframe { frame, value, interpolation });
                param.animated = true;
                return Ok(());
            }
        }
        Err("Parameter not found")
    }

    /// Add node
    pub fn add_node(&mut self, node_type: NodeType, effect: Option<Effect>, x: f32, y: f32) -> u32 {
        let id = self.next_node_id.fetch_add(1, Ordering::Relaxed);
        let node = CompositingNode {
            id,
            node_type,
            effect,
            inputs: Vec::new(),
            outputs: Vec::new(),
            position: (x, y),
            enabled: AtomicBool::new(true),
        };
        self.node_graph.nodes.insert(id, node);
        id
    }

    /// Connect nodes
    pub fn connect_nodes(&mut self, from_id: u32, to_id: u32) -> Result<(), &'static str> {
        let from_node = self.node_graph.nodes.get_mut(&from_id).ok_or("Source node not found")?;
        let to_node = self.node_graph.nodes.get_mut(&to_id).ok_or("Target node not found")?;
        
        from_node.outputs.push(to_id);
        to_node.inputs.push(from_id);
        self.node_graph.connections.push((from_id, to_id));
        Ok(())
    }

    /// Enable mask
    pub fn enable_mask(&mut self, effect_id: u32, enabled: bool) -> Result<(), &'static str> {
        let effect = self.effects.get_mut(&effect_id).ok_or("Effect not found")?;
        effect.mask_enabled = enabled;
        Ok(())
    }

    /// Toggle GPU acceleration
    pub fn toggle_gpu_acceleration(&mut self) {
        let current = self.gpu_acceleration_enabled.load(Ordering::Relaxed);
        self.gpu_acceleration_enabled.store(!current, Ordering::Relaxed);
    }

    /// Remove effect
    pub fn remove_effect(&mut self, effect_id: u32) -> Result<(), &'static str> {
        self.effects.remove(&effect_id).ok_or("Effect not found")?;
        Ok(())
    }

    /// Remove node
    pub fn remove_node(&mut self, node_id: u32) -> Result<(), &'static str> {
        self.node_graph.nodes.remove(&node_id).ok_or("Node not found")?;
        self.node_graph.connections.retain(|(from, to)| *from != node_id && *to != node_id);
        Ok(())
    }

    /// Get effect
    pub fn get_effect(&self, id: u32) -> Option<&EffectInstance> {
        self.effects.get(&id)
    }

    /// Get node
    pub fn get_node(&self, id: u32) -> Option<&CompositingNode> {
        self.node_graph.nodes.get(&id)
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
        let id = engine.add_effect(Effect::GaussianBlur, EffectCategory::Blur);
        assert!(engine.get_effect(id).is_some());
        assert_eq!(engine.get_effect(id).unwrap().category, EffectCategory::Blur);
    }

    #[test]
    fn test_enable_disable_effect() {
        let mut engine = EffectEngine::new();
        engine.init();
        let id = engine.add_effect(Effect::GaussianBlur, EffectCategory::Blur);
        engine.disable_effect(id).unwrap();
        assert!(!engine.get_effect(id).unwrap().enabled.load(Ordering::Relaxed));
        engine.enable_effect(id).unwrap();
        assert!(engine.get_effect(id).unwrap().enabled.load(Ordering::Relaxed));
    }

    #[test]
    fn test_blend_mode() {
        let mut engine = EffectEngine::new();
        engine.init();
        let id = engine.add_effect(Effect::Glow, EffectCategory::Light);
        engine.set_blend_mode(id, BlendMode::Screen).unwrap();
        assert_eq!(engine.get_effect(id).unwrap().blend_mode, BlendMode::Screen);
    }

    #[test]
    fn test_opacity() {
        let mut engine = EffectEngine::new();
        engine.init();
        let id = engine.add_effect(Effect::Vignette, EffectCategory::Light);
        engine.set_opacity(id, 0.5).unwrap();
        assert_eq!(engine.get_effect(id).unwrap().opacity, 0.5);
    }

    #[test]
    fn test_parameters() {
        let mut engine = EffectEngine::new();
        engine.init();
        let id = engine.add_effect(Effect::GaussianBlur, EffectCategory::Blur);
        let param = Parameter {
            name: "radius".to_string(),
            param_type: ParameterType::Float,
            value: 5.0,
            min: 0.0,
            max: 100.0,
            default: 5.0,
        };
        engine.add_parameter(id, param).unwrap();
        engine.set_parameter(id, "radius".to_string(), 10.0).unwrap();
        assert_eq!(engine.get_effect(id).unwrap().parameters[0].parameter.value, 10.0);
    }

    #[test]
    fn test_keyframes() {
        let mut engine = EffectEngine::new();
        engine.init();
        let id = engine.add_effect(Effect::BrightnessContrast, EffectCategory::ColorCorrection);
        let param = Parameter {
            name: "brightness".to_string(),
            param_type: ParameterType::Float,
            value: 0.0,
            min: -100.0,
            max: 100.0,
            default: 0.0,
        };
        engine.add_parameter(id, param).unwrap();
        engine.add_keyframe(id, "brightness".to_string(), 0, 0.0, InterpolationType::Linear).unwrap();
        engine.add_keyframe(id, "brightness".to_string(), 30, 50.0, InterpolationType::EaseInOut).unwrap();
        assert_eq!(engine.get_effect(id).unwrap().parameters[0].keyframes.len(), 2);
        assert!(engine.get_effect(id).unwrap().parameters[0].animated);
    }

    #[test]
    fn test_node_graph() {
        let mut engine = EffectEngine::new();
        engine.init();
        let input_id = engine.add_node(NodeType::Input, None, 0.0, 0.0);
        let effect_id = engine.add_node(NodeType::Effect, Some(Effect::GaussianBlur), 100.0, 0.0);
        let output_id = engine.add_node(NodeType::Output, None, 200.0, 0.0);
        
        engine.connect_nodes(input_id, effect_id).unwrap();
        engine.connect_nodes(effect_id, output_id).unwrap();
        
        assert_eq!(engine.node_graph.nodes.len(), 3);
        assert_eq!(engine.node_graph.connections.len(), 2);
    }

    #[test]
    fn test_mask() {
        let mut engine = EffectEngine::new();
        engine.init();
        let id = engine.add_effect(Effect::ChromaKey, EffectCategory::Keying);
        engine.enable_mask(id, true).unwrap();
        assert!(engine.get_effect(id).unwrap().mask_enabled);
    }

    #[test]
    fn test_gpu_acceleration() {
        let mut engine = EffectEngine::new();
        engine.init();
        assert!(engine.gpu_acceleration_enabled.load(Ordering::Relaxed));
        engine.toggle_gpu_acceleration();
        assert!(!engine.gpu_acceleration_enabled.load(Ordering::Relaxed));
    }

    #[test]
    fn test_remove_effect() {
        let mut engine = EffectEngine::new();
        engine.init();
        let id = engine.add_effect(Effect::Noise, EffectCategory::Stylize);
        engine.remove_effect(id).unwrap();
        assert!(engine.get_effect(id).is_none());
    }

    #[test]
    fn test_remove_node() {
        let mut engine = EffectEngine::new();
        engine.init();
        let id = engine.add_node(NodeType::Effect, Some(Effect::Glow), 0.0, 0.0);
        engine.remove_node(id).unwrap();
        assert!(engine.get_node(id).is_none());
    }
}
