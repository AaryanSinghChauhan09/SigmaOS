//! SigmaAni - Animation and Graphics System
//!
//! Provides comprehensive animation and graphics management inspired by Ani Craft.
//! Supports keyframe animation, tweening, and graphics primitives.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Animation state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Stopped,
    Playing,
    Paused,
    Completed,
}

/// Tweening function type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TweenType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

/// Animation track type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackType {
    Position,
    Rotation,
    Scale,
    Color,
    Opacity,
}

/// Keyframe for animation
#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: u64,
    pub value: f32,
    pub tween: TweenType,
}

impl Keyframe {
    pub fn new(time: u64, value: f32, tween: TweenType) -> Self {
        Self { time, value, tween }
    }
}

/// Animation track
#[derive(Debug, Clone)]
pub struct AnimationTrack {
    pub id: String,
    pub track_type: TrackType,
    pub keyframes: Vec<Keyframe>,
    pub duration: u64,
}

impl AnimationTrack {
    pub fn new(id: String, track_type: TrackType) -> Self {
        Self {
            id,
            track_type,
            keyframes: Vec::new(),
            duration: 0,
        }
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        if keyframe.time > self.duration {
            self.duration = keyframe.time;
        }
        self.keyframes.push(keyframe);
        self.keyframes.sort_by_key(|k| k.time);
    }

    pub fn evaluate(&self, time: u64) -> Option<f32> {
        if self.keyframes.is_empty() {
            return None;
        }

        if time >= self.duration {
            return self.keyframes.last().map(|k| k.value);
        }

        let window = self
            .keyframes
            .windows(2)
            .find(|w| time >= w[0].time && time < w[1].time)?;
        let prev = &window[0];
        let next = &window[1];

        let t = (time - prev.time) as f32 / (next.time - prev.time) as f32;
        let eased = match prev.tween {
            TweenType::Linear => t,
            TweenType::EaseIn => t * t,
            TweenType::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            TweenType::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            }
            TweenType::Bounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
            TweenType::Elastic => {
                let c4 = (2.0 * core::f32::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    -(2.0_f32).powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin()
                }
            }
        };

        Some(prev.value + (next.value - prev.value) * eased)
    }
}

/// Animation
#[derive(Debug, Clone)]
pub struct Animation {
    pub id: String,
    pub name: String,
    pub state: AnimationState,
    pub tracks: BTreeMap<String, AnimationTrack>,
    pub current_time: u64,
    pub loop_animation: bool,
}

impl Animation {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            state: AnimationState::Stopped,
            tracks: BTreeMap::new(),
            current_time: 0,
            loop_animation: false,
        }
    }

    pub fn add_track(&mut self, track: AnimationTrack) {
        self.tracks.insert(track.id.clone(), track);
    }

    pub fn play(&mut self) {
        self.state = AnimationState::Playing;
    }

    pub fn pause(&mut self) {
        self.state = AnimationState::Paused;
    }

    pub fn stop(&mut self) {
        self.state = AnimationState::Stopped;
        self.current_time = 0;
    }

    pub fn update(&mut self, delta_time: u64) {
        if self.state != AnimationState::Playing {
            return;
        }

        let max_duration = self.tracks.values().map(|t| t.duration).max().unwrap_or(0);

        self.current_time += delta_time;

        if self.current_time >= max_duration {
            if self.loop_animation {
                self.current_time = 0;
            } else {
                self.state = AnimationState::Completed;
                self.current_time = max_duration;
            }
        }
    }

    pub fn get_track_value(&self, track_id: &str) -> Option<f32> {
        self.tracks.get(track_id)?.evaluate(self.current_time)
    }

    pub fn get_duration(&self) -> u64 {
        self.tracks.values().map(|t| t.duration).max().unwrap_or(0)
    }
}

/// SigmaAni - Animation system manager
#[derive(Debug)]
pub struct SigmaAni {
    animations: BTreeMap<String, Animation>,
    current_animation: Option<String>,
}

impl SigmaAni {
    pub fn new() -> Self {
        Self {
            animations: BTreeMap::new(),
            current_animation: None,
        }
    }

    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.insert(animation.id.clone(), animation);
    }

    pub fn remove_animation(&mut self, id: &str) -> bool {
        self.animations.remove(id).is_some()
    }

    pub fn get_animation(&self, id: &str) -> Option<&Animation> {
        self.animations.get(id)
    }

    pub fn get_animation_mut(&mut self, id: &str) -> Option<&mut Animation> {
        self.animations.get_mut(id)
    }

    pub fn set_current_animation(&mut self, id: &str) -> bool {
        if self.animations.contains_key(id) {
            self.current_animation = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_animation(&self) -> Option<&Animation> {
        self.current_animation
            .as_ref()
            .and_then(|id| self.animations.get(id))
    }

    pub fn get_current_animation_mut(&mut self) -> Option<&mut Animation> {
        self.current_animation
            .as_ref()
            .and_then(|id| self.animations.get_mut(id.as_str()))
    }

    pub fn play_current(&mut self) -> bool {
        if let Some(anim) = self.get_current_animation_mut() {
            anim.play();
            true
        } else {
            false
        }
    }

    pub fn pause_current(&mut self) -> bool {
        if let Some(anim) = self.get_current_animation_mut() {
            anim.pause();
            true
        } else {
            false
        }
    }

    pub fn stop_current(&mut self) -> bool {
        if let Some(anim) = self.get_current_animation_mut() {
            anim.stop();
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta_time: u64) {
        for animation in self.animations.values_mut() {
            animation.update(delta_time);
        }
    }

    pub fn list_animations(&self) -> Vec<&Animation> {
        self.animations.values().collect()
    }

    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }
}

impl Default for SigmaAni {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaAni instance
static mut SIGMAANI_INSTANCE: Option<SigmaAni> = None;

/// Initialize the global SigmaAni instance
pub fn init_sigmaani() {
    unsafe {
        SIGMAANI_INSTANCE = Some(SigmaAni::new());
    }
}

/// Get the global SigmaAni instance
pub fn get_sigmaani() -> Option<&'static SigmaAni> {
    unsafe { SIGMAANI_INSTANCE.as_ref() }
}

/// Get the global SigmaAni instance (mutable)
pub fn get_sigmaani_mut() -> Option<&'static mut SigmaAni> {
    unsafe { SIGMAANI_INSTANCE.as_mut() }
}

/// Convenience function to add an animation
pub fn add_animation(animation: Animation) -> bool {
    if let Some(ani) = get_sigmaani_mut() {
        ani.add_animation(animation);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyframe_creation() {
        let keyframe = Keyframe::new(100, 0.5, TweenType::Linear);
        assert_eq!(keyframe.time, 100);
        assert_eq!(keyframe.value, 0.5);
    }

    #[test]
    fn test_animation_track_add_keyframe() {
        let mut track = AnimationTrack::new("track1".to_string(), TrackType::Position);
        track.add_keyframe(Keyframe::new(0, 0.0, TweenType::Linear));
        track.add_keyframe(Keyframe::new(100, 1.0, TweenType::Linear));
        assert_eq!(track.keyframes.len(), 2);
        assert_eq!(track.duration, 100);
    }

    #[test]
    fn test_animation_track_evaluate() {
        let mut track = AnimationTrack::new("track1".to_string(), TrackType::Position);
        track.add_keyframe(Keyframe::new(0, 0.0, TweenType::Linear));
        track.add_keyframe(Keyframe::new(100, 1.0, TweenType::Linear));

        let value = track.evaluate(50);
        assert!(value.is_some());
        assert!((value.unwrap() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_animation_creation() {
        let animation = Animation::new("anim1".to_string(), "Test Animation".to_string());
        assert_eq!(animation.id, "anim1");
        assert_eq!(animation.name, "Test Animation");
        assert_eq!(animation.state, AnimationState::Stopped);
    }

    #[test]
    fn test_animation_playback() {
        let mut animation = Animation::new("anim1".to_string(), "Test".to_string());
        animation.play();
        assert_eq!(animation.state, AnimationState::Playing);

        animation.pause();
        assert_eq!(animation.state, AnimationState::Paused);

        animation.stop();
        assert_eq!(animation.state, AnimationState::Stopped);
        assert_eq!(animation.current_time, 0);
    }

    #[test]
    fn test_sigmaani_creation() {
        let ani = SigmaAni::new();
        assert_eq!(ani.animation_count(), 0);
    }

    #[test]
    fn test_sigmaani_add_animation() {
        let mut ani = SigmaAni::new();
        let animation = Animation::new("anim1".to_string(), "Test".to_string());
        ani.add_animation(animation);
        assert_eq!(ani.animation_count(), 1);
    }

    #[test]
    fn test_sigmaani_remove_animation() {
        let mut ani = SigmaAni::new();
        let animation = Animation::new("anim1".to_string(), "Test".to_string());
        ani.add_animation(animation);
        assert!(ani.remove_animation("anim1"));
        assert_eq!(ani.animation_count(), 0);
    }

    #[test]
    fn test_tween_types() {
        let mut track = AnimationTrack::new("track1".to_string(), TrackType::Position);
        track.add_keyframe(Keyframe::new(0, 0.0, TweenType::EaseIn));
        track.add_keyframe(Keyframe::new(100, 1.0, TweenType::EaseIn));

        let value = track.evaluate(50);
        assert!(value.is_some());
    }
}
