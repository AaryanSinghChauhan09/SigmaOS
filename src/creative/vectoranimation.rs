// SPDX-License-Identifier: MIT
// SigmaOS - VectorAnimation (Vector Animation Framework)
// Inspired by Blender, Synfig Studio, OpenToonz, Moho
// Provides professional vector animation, rigging, and motion graphics capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Animation layer type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationLayerType {
    Vector,
    Bitmap,
    Camera,
    Audio,
    Effect,
}

/// Interpolation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationMode {
    Constant,
    Linear,
    Bezier,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

/// Keyframe
#[derive(Debug, Clone)]
pub struct Keyframe {
    pub frame: u32,
    pub value: f32,
    pub interpolation: InterpolationMode,
    pub handle_left: (f32, f32),
    pub handle_right: (f32, f32),
}

/// Bone
#[derive(Debug, Clone)]
pub struct Bone {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
    pub length: f32,
    pub angle: f32,
    pub position: (f32, f32),
    pub ik_chain: bool,
}

/// Rig
#[derive(Debug, Clone)]
pub struct Rig {
    pub id: u32,
    pub name: String,
    pub bones: BTreeMap<u32, Bone>,
    pub ik_chains: Vec<Vec<u32>>,
}

/// Animation layer
#[derive(Debug, Clone)]
pub struct AnimationLayer {
    pub id: u32,
    pub name: String,
    pub layer_type: AnimationLayerType,
    pub visible: bool,
    pub locked: bool,
    pub keyframes: BTreeMap<String, Vec<Keyframe>>,
    pub rig: Option<Rig>,
}

/// Timeline
#[derive(Debug, Clone)]
pub struct Timeline {
    pub fps: u32,
    pub start_frame: u32,
    pub end_frame: u32,
    pub current_frame: u32,
    pub looping: bool,
}

/// Onion skin
#[derive(Debug, Clone)]
pub struct OnionSkin {
    pub enabled: bool,
    pub frames_before: u32,
    pub frames_after: u32,
    pub opacity: f32,
}

/// Animation project
pub struct AnimationProject {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub layers: BTreeMap<u32, AnimationLayer>,
    pub timeline: Timeline,
    pub onion_skin: OnionSkin,
    pub next_layer_id: AtomicU32,
    pub next_bone_id: AtomicU32,
}

impl AnimationProject {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Self {
            name,
            width,
            height,
            layers: BTreeMap::new(),
            timeline: Timeline {
                fps: 24,
                start_frame: 0,
                end_frame: 240,
                current_frame: 0,
                looping: false,
            },
            onion_skin: OnionSkin {
                enabled: true,
                frames_before: 1,
                frames_after: 1,
                opacity: 0.5,
            },
            next_layer_id: AtomicU32::new(1),
            next_bone_id: AtomicU32::new(1),
        }
    }

    /// Add layer
    pub fn add_layer(&mut self, name: String, layer_type: AnimationLayerType) -> u32 {
        let id = self.next_layer_id.fetch_add(1, Ordering::Relaxed);
        let layer = AnimationLayer {
            id,
            name,
            layer_type,
            visible: true,
            locked: false,
            keyframes: BTreeMap::new(),
            rig: None,
        };
        self.layers.insert(id, layer);
        id
    }

    /// Add keyframe
    pub fn add_keyframe(
        &mut self,
        layer_id: u32,
        property: String,
        keyframe: Keyframe,
    ) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        layer
            .keyframes
            .entry(property)
            .or_insert_with(Vec::new)
            .push(keyframe);
        Ok(())
    }

    /// Create rig
    pub fn create_rig(&mut self, layer_id: u32, name: String) -> Result<u32, &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        let rig = Rig {
            id: self.next_bone_id.fetch_add(1, Ordering::Relaxed),
            name,
            bones: BTreeMap::new(),
            ik_chains: Vec::new(),
        };
        let rig_id = rig.id;
        layer.rig = Some(rig);
        Ok(rig_id)
    }

    /// Add bone to rig
    pub fn add_bone(&mut self, layer_id: u32, bone: Bone) -> Result<(), &'static str> {
        let layer = self.layers.get_mut(&layer_id).ok_or("Layer not found")?;
        let rig = layer.rig.as_mut().ok_or("Layer has no rig")?;
        rig.bones.insert(bone.id, bone);
        Ok(())
    }

    /// Set current frame
    pub fn set_frame(&mut self, frame: u32) {
        self.timeline.current_frame =
            frame.clamp(self.timeline.start_frame, self.timeline.end_frame);
    }

    /// Set fps
    pub fn set_fps(&mut self, fps: u32) {
        self.timeline.fps = fps.clamp(1, 120);
    }

    /// Toggle onion skin
    pub fn toggle_onion_skin(&mut self) {
        self.onion_skin.enabled = !self.onion_skin.enabled;
    }

    /// Get interpolated value at frame
    pub fn get_interpolated_value(
        &self,
        layer_id: u32,
        property: String,
        frame: u32,
    ) -> Option<f32> {
        let layer = self.layers.get(&layer_id)?;
        let keyframes = layer.keyframes.get(&property)?;

        if keyframes.is_empty() {
            return None;
        }

        // Find surrounding keyframes
        let mut before = None;
        let mut after = None;

        for kf in keyframes {
            if kf.frame <= frame {
                before = Some(kf);
            } else if after.is_none() {
                after = Some(kf);
            }
        }

        match (before, after) {
            (Some(kf), None) => Some(kf.value),
            (None, Some(kf)) => Some(kf.value),
            (Some(b), Some(a)) => {
                if b.frame == a.frame {
                    Some(b.value)
                } else {
                    let t = (frame - b.frame) as f32 / (a.frame - b.frame) as f32;
                    let value = match b.interpolation {
                        InterpolationMode::Linear => b.value + (a.value - b.value) * t,
                        InterpolationMode::EaseIn => b.value + (a.value - b.value) * t * t,
                        InterpolationMode::EaseOut => {
                            b.value + (a.value - b.value) * (1.0 - (1.0 - t) * (1.0 - t))
                        }
                        InterpolationMode::EaseInOut => {
                            let t2 = t * 2.0;
                            if t2 < 1.0 {
                                b.value + (a.value - b.value) * 0.5 * t2 * t2
                            } else {
                                b.value
                                    + (a.value - b.value) * (1.0 - 0.5 * (2.0 - t2) * (2.0 - t2))
                            }
                        }
                        _ => b.value + (a.value - b.value) * t,
                    };
                    Some(value)
                }
            }
            _ => None,
        }
    }

    /// Export animation
    pub fn export(&self, format: AnimationExportFormat) -> Result<Vec<u8>, &'static str> {
        // Simplified export - in real implementation would render frames
        Ok(Vec::new())
    }
}

/// Animation export format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationExportFormat {
    PNG,
    GIF,
    MP4,
    WebM,
    SVG,
    Lottie,
}

/// Global animation project
static mut ANIMATION_PROJECT: Option<AnimationProject> = None;

/// Initialize animation project
pub fn init_vectoranimation(name: String, width: u32, height: u32) {
    unsafe {
        ANIMATION_PROJECT = Some(AnimationProject::new(name, width, height));
    }
}

/// Get animation project
pub fn get_vectoranimation() -> &'static mut AnimationProject {
    unsafe {
        ANIMATION_PROJECT
            .as_mut()
            .expect("VectorAnimation not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_project_creation() {
        let project = AnimationProject::new("Test".to_string(), 1920, 1080);
        assert_eq!(project.name, "Test");
        assert_eq!(project.width, 1920);
        assert_eq!(project.height, 1080);
    }

    #[test]
    fn test_add_layer() {
        let mut project = AnimationProject::new("Test".to_string(), 1920, 1080);
        let layer_id = project.add_layer("Layer 1".to_string(), AnimationLayerType::Vector);
        assert!(project.layers.contains_key(&layer_id));
    }

    #[test]
    fn test_add_keyframe() {
        let mut project = AnimationProject::new("Test".to_string(), 1920, 1080);
        let layer_id = project.add_layer("Layer 1".to_string(), AnimationLayerType::Vector);
        let keyframe = Keyframe {
            frame: 0,
            value: 0.0,
            interpolation: InterpolationMode::Linear,
            handle_left: (0.0, 0.0),
            handle_right: (0.0, 0.0),
        };
        project
            .add_keyframe(layer_id, "position_x".to_string(), keyframe)
            .unwrap();
        assert_eq!(
            project.layers.get(&layer_id).unwrap().keyframes["position_x"].len(),
            1
        );
    }

    #[test]
    fn test_set_frame() {
        let mut project = AnimationProject::new("Test".to_string(), 1920, 1080);
        project.set_frame(100);
        assert_eq!(project.timeline.current_frame, 100);
    }

    #[test]
    fn test_interpolation() {
        let mut project = AnimationProject::new("Test".to_string(), 1920, 1080);
        let layer_id = project.add_layer("Layer 1".to_string(), AnimationLayerType::Vector);

        let kf1 = Keyframe {
            frame: 0,
            value: 0.0,
            interpolation: InterpolationMode::Linear,
            handle_left: (0.0, 0.0),
            handle_right: (0.0, 0.0),
        };

        let kf2 = Keyframe {
            frame: 10,
            value: 100.0,
            interpolation: InterpolationMode::Linear,
            handle_left: (0.0, 0.0),
            handle_right: (0.0, 0.0),
        };

        project
            .add_keyframe(layer_id, "test".to_string(), kf1)
            .unwrap();
        project
            .add_keyframe(layer_id, "test".to_string(), kf2)
            .unwrap();

        let value = project.get_interpolated_value(layer_id, "test".to_string(), 5);
        assert_eq!(value, Some(50.0));
    }
}
