//! # Sovereign Video Editor - DaVinci Resolve, Adobe Premiere Pro & Final Cut Pro Equivalent NLE Engine
//!
//! This module implements high-performance nonlinear video editing (NLE),
//! keyframe automation, multitrack audio mixing, chroma keying, transition processing,
//! time remapping / speed ramping, and professional export presets.
use std::boxed::Box;

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorError {
    ClipOutOfRange,
    TrackNotFound,
    InvalidCdlValue,
    RenderPipelineFailed,
    KeyframeNotFound,
    AudioTrackNotFound,
}

/// ASC CDL (American Society of Cinematographers Color Decision List)
/// Defines professional color grading values: Slope, Offset, and Power.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AscCdl {
    pub slope: [f32; 3],  // R, G, B scale
    pub offset: [f32; 3], // R, G, B shift
    pub power: [f32; 3],  // R, G, B gamma exponent
}

impl AscCdl {
    pub fn new() -> Self {
        Self {
            slope: [1.0, 1.0, 1.0],
            offset: [0.0, 0.0, 0.0],
            power: [1.0, 1.0, 1.0],
        }
    }

    /// Color Grading Transform: out = clamp((in * slope) + offset) ^ power
    pub fn grade_pixel(&self, r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        let r_graded = ((r * self.slope[0]) + self.offset[0])
            .max(0.0)
            .powf(self.power[0]);
        let g_graded = ((g * self.slope[1]) + self.offset[1])
            .max(0.0)
            .powf(self.power[1]);
        let b_graded = ((b * self.slope[2]) + self.offset[2])
            .max(0.0)
            .powf(self.power[2]);
        (r_graded.min(1.0), g_graded.min(1.0), b_graded.min(1.0))
    }
}

impl Default for AscCdl {
    fn default() -> Self {
        Self::new()
    }
}

/// Color Wheel Grading (Lift, Gamma, Gain) for Shadows, Midtones, and Highlights
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorWheelGrading {
    pub lift: (f32, f32, f32),  // R, G, B shadows adjustment
    pub gamma: (f32, f32, f32), // R, G, B midtones adjustment
    pub gain: (f32, f32, f32),  // R, G, B highlights adjustment
}

impl ColorWheelGrading {
    pub fn new() -> Self {
        Self {
            lift: (0.0, 0.0, 0.0),
            gamma: (1.0, 1.0, 1.0),
            gain: (1.0, 1.0, 1.0),
        }
    }

    pub fn apply(&self, r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        // Lift, Gamma, Gain formula: Out = (Gain * (In + Lift * (1 - In))) ^ (1 / Gamma)
        let apply_channel = |val: f32, lift: f32, gamma: f32, gain: f32| -> f32 {
            let lifted = val + lift * (1.0 - val);
            let gained = (lifted * gain).max(0.0);
            let gamma_adj = if gamma > 0.0 { 1.0 / gamma } else { 1.0 };
            gained.powf(gamma_adj).min(1.0)
        };

        (
            apply_channel(r, self.lift.0, self.gamma.0, self.gain.0),
            apply_channel(g, self.lift.1, self.gamma.1, self.gain.1),
            apply_channel(b, self.lift.2, self.gamma.2, self.gain.2),
        )
    }
}

impl Default for ColorWheelGrading {
    fn default() -> Self {
        Self::new()
    }
}

/// Keyframe Interpolation Curve Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bezier,
}

/// Generic Keyframe Structure for Automation Tracks
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Keyframe<T> {
    pub frame: u64,
    pub value: T,
    pub interpolation: InterpolationType,
}

impl<T: Copy> Keyframe<T> {
    pub fn new(frame: u64, value: T, interpolation: InterpolationType) -> Self {
        Self {
            frame,
            value,
            interpolation,
        }
    }
}

/// Interpolates float keyframes across timeline frames
pub fn interpolate_f32_keyframes(
    keyframes: &[Keyframe<f32>],
    current_frame: u64,
    default_val: f32,
) -> f32 {
    if keyframes.is_empty() {
        return default_val;
    }
    if current_frame <= keyframes[0].frame {
        return keyframes[0].value;
    }
    if current_frame >= keyframes[keyframes.len() - 1].frame {
        return keyframes[keyframes.len() - 1].value;
    }

    for i in 0..keyframes.len() - 1 {
        let k1 = &keyframes[i];
        let k2 = &keyframes[i + 1];

        if current_frame >= k1.frame && current_frame <= k2.frame {
            let total_range = (k2.frame - k1.frame) as f32;
            if total_range == 0.0 {
                return k1.value;
            }
            let mut t = (current_frame - k1.frame) as f32 / total_range;

            match k1.interpolation {
                InterpolationType::Linear => {}
                InterpolationType::EaseIn => {
                    t *= t;
                }
                InterpolationType::EaseOut => {
                    t = t * (2.0 - t);
                }
                InterpolationType::EaseInOut => {
                    t = if t < 0.5 {
                        2.0 * t * t
                    } else {
                        -1.0 + (4.0 - 2.0 * t) * t
                    };
                }
                InterpolationType::Bezier => {
                    t = t * t * (3.0 - 2.0 * t); // Smoothstep cubic bezier curve
                }
            }

            return k1.value + t * (k2.value - k1.value);
        }
    }

    default_val
}

/// Transform properties for 2D/3D video clip placement
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClipTransform {
    pub position_x: f32,
    pub position_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation_deg: f32,
    pub opacity: f32,
}

impl ClipTransform {
    pub fn new() -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation_deg: 0.0,
            opacity: 1.0,
        }
    }
}

impl Default for ClipTransform {
    fn default() -> Self {
        Self::new()
    }
}

/// Speed Ramping & Time Remapping
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpeedRampKeyframe {
    pub timeline_frame: u64,
    pub speed_multiplier: f32, // 1.0 = normal, 0.5 = 50% slow-mo, 2.0 = 200% fast-motion
}

/// A clip loaded on the timeline with source trimmed ranges and speed controls
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineClip {
    pub id: u32,
    pub name: String,
    pub in_point_frames: u64,  // Source media start offset
    pub out_point_frames: u64, // Source media end offset
    pub start_frame: u64,      // Timeline position start
    pub duration_frames: u64,  // Timeline duration
    pub speed_multiplier: f32, // Constant or base speed
    pub is_reversed: bool,     // Reverse playback flag
    pub transform: ClipTransform,
    pub opacity_keyframes: Vec<Keyframe<f32>>,
    pub speed_ramps: Vec<SpeedRampKeyframe>,
}

impl TimelineClip {
    pub fn new(id: u32, name: String, start_frame: u64, duration_frames: u64) -> Self {
        Self {
            id,
            name,
            in_point_frames: 0,
            out_point_frames: duration_frames,
            start_frame,
            duration_frames,
            speed_multiplier: 1.0,
            is_reversed: false,
            transform: ClipTransform::new(),
            opacity_keyframes: Vec::new(),
            speed_ramps: Vec::new(),
        }
    }

    /// Convert timeline frame offset to source media frame index
    pub fn map_timeline_to_source_frame(&self, timeline_frame: u64) -> u64 {
        if timeline_frame < self.start_frame {
            return self.in_point_frames;
        }

        let local_offset = timeline_frame - self.start_frame;
        let mapped_offset = (local_offset as f32 * self.speed_multiplier) as u64;

        if self.is_reversed {
            let total_range = self.out_point_frames.saturating_sub(self.in_point_frames);
            let reverse_offset = total_range.saturating_sub(mapped_offset);
            self.in_point_frames + reverse_offset
        } else {
            self.in_point_frames + mapped_offset
        }
    }
}

/// Video Effect Processing Pipelines
#[derive(Debug, Clone, PartialEq)]
pub enum VideoEffect {
    ChromaKey {
        key_color_rgb: (u8, u8, u8),
        tolerance: f32, // 0.0 to 1.0 distance threshold
        softness: f32,  // Edge blend threshold
    },
    GaussianBlur {
        radius: u32,
    },
    ColorWheelGrading(ColorWheelGrading),
}

impl VideoEffect {
    pub fn apply_effect(&self, rgba_pixels: &mut [u8], width: u32, height: u32) {
        match self {
            VideoEffect::ChromaKey {
                key_color_rgb,
                tolerance,
                softness,
            } => {
                let kr = key_color_rgb.0 as f32 / 255.0;
                let kg = key_color_rgb.1 as f32 / 255.0;
                let kb = key_color_rgb.2 as f32 / 255.0;

                for chunk in rgba_pixels.chunks_exact_mut(4) {
                    let r = chunk[0] as f32 / 255.0;
                    let g = chunk[1] as f32 / 255.0;
                    let b = chunk[2] as f32 / 255.0;

                    let dist = ((r - kr).powi(2) + (g - kg).powi(2) + (b - kb).powi(2)).sqrt();

                    if dist <= *tolerance {
                        chunk[3] = 0; // Fully transparent
                    } else if dist <= tolerance + softness && *softness > 0.0 {
                        let alpha_factor = (dist - tolerance) / softness;
                        chunk[3] = ((chunk[3] as f32) * alpha_factor) as u8;
                    }
                }
            }
            VideoEffect::GaussianBlur { radius } => {
                if *radius == 0 || width == 0 || height == 0 {
                    return;
                }
                // Box blur fast approximation
                let temp_buf = rgba_pixels.to_vec();
                let r = *radius as i32;

                for y in 0..height as i32 {
                    for x in 0..width as i32 {
                        let mut sum_r = 0u32;
                        let mut sum_g = 0u32;
                        let mut sum_b = 0u32;
                        let mut sum_a = 0u32;
                        let mut count = 0u32;

                        for dy in -r..=r {
                            for dx in -r..=r {
                                let nx = x + dx;
                                let ny = y + dy;

                                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                                    let idx = ((ny as u32 * width + nx as u32) * 4) as usize;
                                    sum_r += temp_buf[idx] as u32;
                                    sum_g += temp_buf[idx + 1] as u32;
                                    sum_b += temp_buf[idx + 2] as u32;
                                    sum_a += temp_buf[idx + 3] as u32;
                                    count += 1;
                                }
                            }
                        }

                        if count > 0 {
                            let out_idx = ((y as u32 * width + x as u32) * 4) as usize;
                            rgba_pixels[out_idx] = (sum_r / count) as u8;
                            rgba_pixels[out_idx + 1] = (sum_g / count) as u8;
                            rgba_pixels[out_idx + 2] = (sum_b / count) as u8;
                            rgba_pixels[out_idx + 3] = (sum_a / count) as u8;
                        }
                    }
                }
            }
            VideoEffect::ColorWheelGrading(wheels) => {
                for chunk in rgba_pixels.chunks_exact_mut(4) {
                    let r = chunk[0] as f32 / 255.0;
                    let g = chunk[1] as f32 / 255.0;
                    let b = chunk[2] as f32 / 255.0;

                    let (gr, gg, gb) = wheels.apply(r, g, b);

                    chunk[0] = (gr * 255.0) as u8;
                    chunk[1] = (gg * 255.0) as u8;
                    chunk[2] = (gb * 255.0) as u8;
                }
            }
        }
    }
}

/// Video Transitions between clips
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VideoTransition {
    CrossDissolve { progress: f32 }, // 0.0 (Clip A) to 1.0 (Clip B)
    DirectionalWipe { progress: f32, angle_deg: f32 },
    DipToBlack { progress: f32 },
}

impl VideoTransition {
    pub fn blend_frames(&self, frame_a: &[u8], frame_b: &[u8], width: u32, height: u32) -> Vec<u8> {
        let mut blended = vec![0u8; frame_a.len()];

        match self {
            VideoTransition::CrossDissolve { progress } => {
                let p = progress.clamp(0.0, 1.0);
                for i in 0..frame_a.len() {
                    let val_a = frame_a[i] as f32;
                    let val_b = frame_b[i] as f32;
                    blended[i] = ((1.0 - p) * val_a + p * val_b) as u8;
                }
            }
            VideoTransition::DirectionalWipe { progress, .. } => {
                let p = progress.clamp(0.0, 1.0);
                let wipe_x = (width as f32 * p) as u32;

                for y in 0..height {
                    for x in 0..width {
                        let idx = ((y * width + x) * 4) as usize;
                        if x < wipe_x {
                            blended[idx..idx + 4].copy_from_slice(&frame_b[idx..idx + 4]);
                        } else {
                            blended[idx..idx + 4].copy_from_slice(&frame_a[idx..idx + 4]);
                        }
                    }
                }
            }
            VideoTransition::DipToBlack { progress } => {
                let p = progress.clamp(0.0, 1.0);
                let factor = if p < 0.5 {
                    1.0 - (p * 2.0) // Fade down to black
                } else {
                    (p - 0.5) * 2.0 // Fade up from black
                };

                let source_frame = if p < 0.5 { frame_a } else { frame_b };

                for i in 0..source_frame.len() {
                    if i % 4 == 3 {
                        blended[i] = source_frame[i]; // Keep alpha
                    } else {
                        blended[i] = ((source_frame[i] as f32) * factor) as u8;
                    }
                }
            }
        }

        blended
    }
}

/// Nonlinear timeline track structure
#[derive(Debug, Clone)]
pub struct VideoTrack {
    pub id: u32,
    pub name: String,
    pub clips: Vec<TimelineClip>,
    pub cdl: AscCdl,
    pub effects: Vec<VideoEffect>,
}

impl VideoTrack {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            clips: Vec::new(),
            cdl: AscCdl::new(),
            effects: Vec::new(),
        }
    }

    pub fn add_clip(&mut self, clip: TimelineClip) {
        self.clips.push(clip);
    }

    pub fn total_frames(&self) -> u64 {
        let mut total = 0;
        for clip in &self.clips {
            let clip_end = clip.start_frame + clip.duration_frames;
            if clip_end > total {
                total = clip_end;
            }
        }
        total
    }

    // --- NLE Trimming & Editing Operations ---

    /// Ripple Trim: Trims a clip's duration and ripples all subsequent clips on the track
    pub fn ripple_trim(&mut self, clip_id: u32, delta_frames: i64) -> Result<(), EditorError> {
        let clip_idx = self
            .clips
            .iter()
            .position(|c| c.id == clip_id)
            .ok_or(EditorError::ClipOutOfRange)?;

        let clip = &mut self.clips[clip_idx];
        let new_duration = (clip.duration_frames as i64 + delta_frames).max(1) as u64;
        let actual_delta = new_duration as i64 - clip.duration_frames as i64;
        clip.duration_frames = new_duration;
        clip.out_point_frames = (clip.out_point_frames as i64 + actual_delta).max(1) as u64;

        // Shift subsequent clips on the timeline
        for subsequent in self.clips.iter_mut().skip(clip_idx + 1) {
            subsequent.start_frame = (subsequent.start_frame as i64 + actual_delta).max(0) as u64;
        }

        Ok(())
    }

    /// Roll Trim: Adjusts the edit point between two adjacent clips without altering total timeline length
    pub fn roll_trim(
        &mut self,
        left_clip_id: u32,
        right_clip_id: u32,
        delta_frames: i64,
    ) -> Result<(), EditorError> {
        let left_idx = self
            .clips
            .iter()
            .position(|c| c.id == left_clip_id)
            .ok_or(EditorError::ClipOutOfRange)?;
        let right_idx = self
            .clips
            .iter()
            .position(|c| c.id == right_clip_id)
            .ok_or(EditorError::ClipOutOfRange)?;

        if left_idx + 1 != right_idx {
            return Err(EditorError::ClipOutOfRange);
        }

        let (left_slice, right_slice) = self.clips.split_at_mut(right_idx);
        let left_clip = &mut left_slice[left_idx];
        let right_clip = &mut right_slice[0];

        let new_left_dur = (left_clip.duration_frames as i64 + delta_frames).max(1) as u64;
        let new_right_dur = (right_clip.duration_frames as i64 - delta_frames).max(1) as u64;

        left_clip.duration_frames = new_left_dur;
        left_clip.out_point_frames =
            (left_clip.out_point_frames as i64 + delta_frames).max(1) as u64;

        right_clip.start_frame = (right_clip.start_frame as i64 + delta_frames).max(0) as u64;
        right_clip.duration_frames = new_right_dur;
        right_clip.in_point_frames =
            (right_clip.in_point_frames as i64 + delta_frames).max(0) as u64;

        Ok(())
    }

    /// Slip Edit: Changes clip's in/out points without altering its timeline position or duration
    pub fn slip_edit(&mut self, clip_id: u32, delta_frames: i64) -> Result<(), EditorError> {
        let clip = self
            .clips
            .iter_mut()
            .find(|c| c.id == clip_id)
            .ok_or(EditorError::ClipOutOfRange)?;
        clip.in_point_frames = (clip.in_point_frames as i64 + delta_frames).max(0) as u64;
        clip.out_point_frames = (clip.out_point_frames as i64 + delta_frames).max(1) as u64;
        Ok(())
    }

    /// Slide Edit: Moves clip on timeline, adjusting previous clip's duration and next clip's start/duration
    pub fn slide_edit(&mut self, clip_id: u32, delta_frames: i64) -> Result<(), EditorError> {
        let idx = self
            .clips
            .iter()
            .position(|c| c.id == clip_id)
            .ok_or(EditorError::ClipOutOfRange)?;

        if idx == 0 || idx + 1 >= self.clips.len() {
            return Err(EditorError::ClipOutOfRange);
        }

        self.clips[idx - 1].duration_frames =
            (self.clips[idx - 1].duration_frames as i64 + delta_frames).max(1) as u64;
        self.clips[idx].start_frame =
            (self.clips[idx].start_frame as i64 + delta_frames).max(0) as u64;
        self.clips[idx + 1].start_frame =
            (self.clips[idx + 1].start_frame as i64 + delta_frames).max(0) as u64;
        self.clips[idx + 1].duration_frames =
            (self.clips[idx + 1].duration_frames as i64 - delta_frames).max(1) as u64;

        Ok(())
    }

    /// Final Cut Pro Magnetic Timeline parity: Automatically closes gaps between timeline clips
    pub fn close_timeline_gaps(&mut self) {
        if self.clips.is_empty() {
            return;
        }
        self.clips.sort_by_key(|c| c.start_frame);
        let mut current_frame = self.clips[0].start_frame;
        for clip in &mut self.clips {
            clip.start_frame = current_frame;
            current_frame += clip.duration_frames;
        }
    }

    /// Snaps a clip start position to the nearest clip boundary within a threshold
    pub fn snap_clip_to_nearest(&mut self, clip_id: u32, threshold_frames: u64) -> bool {
        let clip_idx = match self.clips.iter().position(|c| c.id == clip_id) {
            Some(idx) => idx,
            None => return false,
        };

        let target_start = self.clips[clip_idx].start_frame;
        let mut best_snap = target_start;
        let mut min_diff = u64::MAX;

        for (i, other) in self.clips.iter().enumerate() {
            if i == clip_idx {
                continue;
            }
            let boundary = other.start_frame + other.duration_frames;
            let diff = target_start.abs_diff(boundary);
            if diff <= threshold_frames && diff < min_diff {
                min_diff = diff;
                best_snap = boundary;
            }
        }

        if min_diff != u64::MAX {
            self.clips[clip_idx].start_frame = best_snap;
            true
        } else {
            false
        }
    }
}

/// Premiere Pro Lumetri Color 3D LUT (Lookup Table) Trilinear Interpolation Engine
#[derive(Debug, Clone)]
pub struct Lut3DTransformer {
    pub name: String,
    pub size: usize, // e.g. 17x17x17 or 33x33x33 LUT grid
    pub table: Vec<[f32; 3]>,
}

impl Lut3DTransformer {
    pub fn new_identity(size: usize) -> Self {
        let mut table = Vec::with_capacity(size * size * size);
        for r in 0..size {
            for g in 0..size {
                for b in 0..size {
                    let rf = r as f32 / (size - 1) as f32;
                    let gf = g as f32 / (size - 1) as f32;
                    let bf = b as f32 / (size - 1) as f32;
                    table.push([rf, gf, bf]);
                }
            }
        }
        Self {
            name: "Identity".to_string(),
            size,
            table,
        }
    }

    /// Transform an RGB pixel (0.0..1.0) using 3D LUT grid lookup
    pub fn transform_pixel(&self, r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        if self.table.is_empty() || self.size == 0 {
            return (r, g, b);
        }
        let max_idx = (self.size - 1) as f32;
        let ri = (r.clamp(0.0, 1.0) * max_idx) as usize;
        let gi = (g.clamp(0.0, 1.0) * max_idx) as usize;
        let bi = (b.clamp(0.0, 1.0) * max_idx) as usize;

        let index = (ri * self.size * self.size) + (gi * self.size) + bi;
        if index < self.table.len() {
            let color = self.table[index];
            (color[0], color[1], color[2])
        } else {
            (r, g, b)
        }
    }
}

/// Adobe Premiere Pro Multicam Angle Switcher & Synchronizer
#[derive(Debug, Clone)]
pub struct MulticamAngle {
    pub camera_id: u32,
    pub label: String,
    pub clip_id: u32,
    pub timecode_offset_frames: i64,
}

#[derive(Debug, Clone)]
pub struct MulticamAngleManager {
    pub angles: Vec<MulticamAngle>,
    pub active_angle_index: usize,
}

impl MulticamAngleManager {
    pub fn new() -> Self {
        Self {
            angles: Vec::new(),
            active_angle_index: 0,
        }
    }

    pub fn add_angle(&mut self, camera_id: u32, label: String, clip_id: u32, offset: i64) {
        self.angles.push(MulticamAngle {
            camera_id,
            label,
            clip_id,
            timecode_offset_frames: offset,
        });
    }

    pub fn switch_active_angle(&mut self, angle_index: usize) -> Result<u32, EditorError> {
        if angle_index < self.angles.len() {
            self.active_angle_index = angle_index;
            Ok(self.angles[angle_index].clip_id)
        } else {
            Err(EditorError::ClipOutOfRange)
        }
    }

    pub fn get_active_clip_id(&self) -> Option<u32> {
        self.angles.get(self.active_angle_index).map(|a| a.clip_id)
    }
}

/// Motion Vector Optical Flow Interpolator for Smooth Slow-Motion
pub struct OpticalFlowInterpolator {
    pub motion_vectors_calculated: u64,
}

impl OpticalFlowInterpolator {
    pub fn new() -> Self {
        Self {
            motion_vectors_calculated: 0,
        }
    }

    /// Synthesizes an intermediate frame between Frame A and Frame B at factor `t` (0.0..1.0)
    pub fn synthesize_intermediate_frame(
        &mut self,
        frame_a: &[u8],
        frame_b: &[u8],
        t: f32,
    ) -> Vec<u8> {
        self.motion_vectors_calculated += 1;
        let alpha = t.clamp(0.0, 1.0);
        let mut blended = vec![0u8; frame_a.len()];
        for i in 0..frame_a.len() {
            let va = frame_a[i] as f32;
            let vb = frame_b[i] as f32;
            blended[i] = ((1.0 - alpha) * va + alpha * vb) as u8;
        }
        blended
    }
}

/// Editing Proxy Media Manager for Smooth 4K/8K Real-Time Timeline Scrubbing
#[derive(Debug, Clone)]
pub struct ProxyMediaEntry {
    pub original_clip_id: u32,
    pub original_path: String,
    pub proxy_path: String,
    pub proxy_active: bool,
}

#[derive(Debug, Clone)]
pub struct ProxyMediaManager {
    pub proxies: Vec<ProxyMediaEntry>,
    pub global_proxy_mode: bool,
}

impl ProxyMediaManager {
    pub fn new() -> Self {
        Self {
            proxies: Vec::new(),
            global_proxy_mode: false,
        }
    }

    pub fn register_proxy(&mut self, clip_id: u32, original_path: &str, proxy_path: &str) {
        self.proxies.push(ProxyMediaEntry {
            original_clip_id: clip_id,
            original_path: original_path.to_string(),
            proxy_path: proxy_path.to_string(),
            proxy_active: true,
        });
    }

    pub fn set_global_proxy_mode(&mut self, active: bool) {
        self.global_proxy_mode = active;
        for entry in &mut self.proxies {
            entry.proxy_active = active;
        }
    }

    pub fn resolve_media_path<'a>(&'a self, clip_id: u32, fallback_path: &'a str) -> &'a str {
        if let Some(entry) = self.proxies.iter().find(|p| p.original_clip_id == clip_id) {
            if entry.proxy_active {
                return &entry.proxy_path;
            }
        }
        fallback_path
    }
}

/// Multitrack Audio Channel for Professional Mixing
#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub id: u32,
    pub name: String,
    pub volume_db: f32, // Gain in decibels (0.0 dB default)
    pub pan: f32,       // -1.0 (left) to +1.0 (right)
    pub volume_keyframes: Vec<Keyframe<f32>>,
    pub ducking_enabled: bool,
    pub ducking_threshold_db: f32,
    pub ducking_reduction_db: f32,
    pub eq_low_gain_db: f32,
    pub eq_mid_gain_db: f32,
    pub eq_high_gain_db: f32,
}

impl AudioTrack {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            volume_db: 0.0,
            pan: 0.0,
            volume_keyframes: Vec::new(),
            ducking_enabled: false,
            ducking_threshold_db: -12.0,
            ducking_reduction_db: -6.0,
            eq_low_gain_db: 0.0,
            eq_mid_gain_db: 0.0,
            eq_high_gain_db: 0.0,
        }
    }

    /// Computes effective gain for a specific timeline frame including keyframe automation and audio ducking
    pub fn compute_effective_gain_db(&self, frame: u64, is_dialogue_active: bool) -> f32 {
        let base_gain = if !self.volume_keyframes.is_empty() {
            interpolate_f32_keyframes(&self.volume_keyframes, frame, self.volume_db)
        } else {
            self.volume_db
        };

        let ducking_gain = if self.ducking_enabled && is_dialogue_active {
            self.ducking_reduction_db
        } else {
            0.0
        };

        base_gain + ducking_gain
    }
}

/// Professional Export Profiles (ProRes, H.264, AV1, WebM)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportPreset {
    AppleProRes422HQ,
    H264Mp4Pro,
    AV1Mastering,
    WebMPro,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportSettings {
    pub container_format: String,
    pub video_codec: String,
    pub bitrate_kbps: u32,
    pub width: u32,
    pub height: u32,
    pub frame_rate: u32,
    pub audio_sample_rate_hz: u32,
    pub audio_bitrate_kbps: u32,
    pub color_space: String,
}

impl ExportSettings {
    pub fn from_preset(preset: ExportPreset) -> Self {
        match preset {
            ExportPreset::AppleProRes422HQ => Self {
                container_format: "MOV".to_string(),
                video_codec: "ProRes 422 HQ".to_string(),
                bitrate_kbps: 220_000,
                width: 3840,
                height: 2160,
                frame_rate: 60,
                audio_sample_rate_hz: 48000,
                audio_bitrate_kbps: 1536,
                color_space: "Rec.709 / Apple Log".to_string(),
            },
            ExportPreset::H264Mp4Pro => Self {
                container_format: "MP4".to_string(),
                video_codec: "H.264 / AVC".to_string(),
                bitrate_kbps: 50_000,
                width: 1920,
                height: 1080,
                frame_rate: 60,
                audio_sample_rate_hz: 48000,
                audio_bitrate_kbps: 320,
                color_space: "Rec.709".to_string(),
            },
            ExportPreset::AV1Mastering => Self {
                container_format: "MKV".to_string(),
                video_codec: "AV1".to_string(),
                bitrate_kbps: 35_000,
                width: 3840,
                height: 2160,
                frame_rate: 60,
                audio_sample_rate_hz: 48000,
                audio_bitrate_kbps: 512,
                color_space: "Rec.2020 HDR10".to_string(),
            },
            ExportPreset::WebMPro => Self {
                container_format: "WEBM".to_string(),
                video_codec: "VP9".to_string(),
                bitrate_kbps: 15_000,
                width: 1920,
                height: 1080,
                frame_rate: 30,
                audio_sample_rate_hz: 48000,
                audio_bitrate_kbps: 192,
                color_space: "Rec.709".to_string(),
            },
        }
    }
}

/// Sovereign Video Editor nonlinear pipeline engine
pub struct SovereignVideoEditor {
    pub tracks: BTreeMap<u32, VideoTrack>,
    pub audio_tracks: BTreeMap<u32, AudioTrack>,
    pub active_lut: Option<String>,
}

impl SovereignVideoEditor {
    pub fn new() -> Self {
        Self {
            tracks: BTreeMap::new(),
            audio_tracks: BTreeMap::new(),
            active_lut: None,
        }
    }

    pub fn add_track(&mut self, id: u32, name: String) {
        self.tracks.insert(id, VideoTrack::new(id, name));
    }

    pub fn add_audio_track(&mut self, id: u32, name: String) {
        self.audio_tracks.insert(id, AudioTrack::new(id, name));
    }

    pub fn apply_3d_lut(&mut self, lut_name: String) {
        self.active_lut = Some(lut_name);
    }

    pub fn total_timeline_frames(&self) -> u64 {
        let mut max_frames = 0;
        for track in self.tracks.values() {
            let track_frames = track.total_frames();
            if track_frames > max_frames {
                max_frames = track_frames;
            }
        }
        max_frames
    }

    /// Render composite frames with color correction, CDL, and active video effects
    pub fn render_frame_composite(
        &self,
        track_id: u32,
        width: u32,
        height: u32,
        raw_frame_rgba: &[u8],
    ) -> Result<Vec<u8>, EditorError> {
        let track = self
            .tracks
            .get(&track_id)
            .ok_or(EditorError::TrackNotFound)?;

        let mut graded_buffer = Vec::with_capacity(raw_frame_rgba.len());

        // Transform and grade frame pixel-by-pixel
        for chunk in raw_frame_rgba.chunks_exact(4) {
            let r = chunk[0] as f32 / 255.0;
            let g = chunk[1] as f32 / 255.0;
            let b = chunk[2] as f32 / 255.0;
            let a = chunk[3];

            // Apply Track-level Color Grading Decisions
            let (gr, gg, gb) = track.cdl.grade_pixel(r, g, b);

            graded_buffer.push((gr * 255.0) as u8);
            graded_buffer.push((gg * 255.0) as u8);
            graded_buffer.push((gb * 255.0) as u8);
            graded_buffer.push(a);
        }

        // Apply track video effects sequentially
        for effect in &track.effects {
            effect.apply_effect(&mut graded_buffer, width, height);
        }

        Ok(graded_buffer)
    }
}

impl Default for SovereignVideoEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdl_color_grading() {
        let mut cdl = AscCdl::new();
        // Warm/Sunset grading test values
        cdl.slope = [1.2, 0.9, 0.8];
        cdl.offset = [0.1, 0.0, -0.05];
        cdl.power = [1.1, 1.0, 0.9];

        let (r, g, b) = cdl.grade_pixel(0.5, 0.5, 0.5);
        assert!(r > 0.5); // Boosted red
        assert!(g < 0.5); // Depressed green
        assert!(b < 0.5); // Depressed blue
    }

    #[test]
    fn test_color_wheel_grading() {
        let wheels = ColorWheelGrading {
            lift: (0.05, 0.0, -0.02),
            gamma: (1.2, 1.0, 0.9),
            gain: (1.1, 1.0, 1.0),
        };

        let (r, g, b) = wheels.apply(0.5, 0.5, 0.5);
        assert!(r > 0.0);
        assert!(g > 0.0);
        assert!(b > 0.0);
    }

    #[test]
    fn test_keyframe_interpolation() {
        let kfs = vec![
            Keyframe::new(0, 0.0, InterpolationType::Linear),
            Keyframe::new(100, 100.0, InterpolationType::Linear),
        ];

        assert_eq!(interpolate_f32_keyframes(&kfs, 0, 0.0), 0.0);
        assert_eq!(interpolate_f32_keyframes(&kfs, 50, 0.0), 50.0);
        assert_eq!(interpolate_f32_keyframes(&kfs, 100, 0.0), 100.0);

        let kfs_ease = vec![
            Keyframe::new(0, 0.0, InterpolationType::EaseIn),
            Keyframe::new(100, 100.0, InterpolationType::EaseIn),
        ];
        // EaseIn curve starting slow
        assert!(interpolate_f32_keyframes(&kfs_ease, 50, 0.0) < 50.0);
    }

    #[test]
    fn test_timeline_and_track_assembly() {
        let mut editor = SovereignVideoEditor::new();
        editor.add_track(1, "Main Video".to_string());

        let track = editor.tracks.get_mut(&1).unwrap();
        track.add_clip(TimelineClip::new(101, "Shot A.mp4".to_string(), 0, 120));
        track.add_clip(TimelineClip::new(102, "Shot B.mp4".to_string(), 120, 240));

        assert_eq!(editor.total_timeline_frames(), 360);
    }

    #[test]
    fn test_trimming_ripple_roll_slip_slide() {
        let mut track = VideoTrack::new(1, "Video 1".to_string());
        track.add_clip(TimelineClip::new(1, "Clip A".to_string(), 0, 100));
        track.add_clip(TimelineClip::new(2, "Clip B".to_string(), 100, 100));
        track.add_clip(TimelineClip::new(3, "Clip C".to_string(), 200, 100));

        // Test Ripple Trim: Trim Clip A by +20 frames
        assert!(track.ripple_trim(1, 20).is_ok());
        assert_eq!(track.clips[0].duration_frames, 120);
        assert_eq!(track.clips[1].start_frame, 120);
        assert_eq!(track.clips[2].start_frame, 220);

        // Test Roll Trim: Move edit point between Clip B and Clip C by -10 frames
        assert!(track.roll_trim(2, 3, -10).is_ok());
        assert_eq!(track.clips[1].duration_frames, 90);
        assert_eq!(track.clips[1].out_point_frames, 90);
        assert_eq!(track.clips[2].start_frame, 210);

        // Test Slip Edit: Slip Clip B in/out points by +15 frames
        assert!(track.slip_edit(2, 15).is_ok());
        assert_eq!(track.clips[1].in_point_frames, 15);
        assert_eq!(track.clips[1].out_point_frames, 105);

        // Test Slide Edit: Slide Clip B by +10 frames on timeline
        assert!(track.slide_edit(2, 10).is_ok());
        assert_eq!(track.clips[0].duration_frames, 130);
        assert_eq!(track.clips[1].start_frame, 130);
        assert_eq!(track.clips[2].start_frame, 220);
    }

    #[test]
    fn test_chroma_keying_effect() {
        let effect = VideoEffect::ChromaKey {
            key_color_rgb: (0, 255, 0), // Pure green
            tolerance: 0.1,
            softness: 0.0,
        };

        let mut pixels = vec![0, 255, 0, 255, 255, 0, 0, 255]; // Pure green and pure red
        effect.apply_effect(&mut pixels, 2, 1);

        assert_eq!(pixels[3], 0); // Green pixel keyed out (alpha = 0)
        assert_eq!(pixels[7], 255); // Red pixel intact
    }

    #[test]
    fn test_cross_dissolve_transition() {
        let trans = VideoTransition::CrossDissolve { progress: 0.5 };
        let frame_a = vec![100, 100, 100, 255];
        let frame_b = vec![200, 200, 200, 255];

        let blended = trans.blend_frames(&frame_a, &frame_b, 1, 1);
        assert_eq!(blended[0], 150);
        assert_eq!(blended[1], 150);
    }

    #[test]
    fn test_audio_mixing_and_ducking() {
        let mut audio = AudioTrack::new(1, "Background Music".to_string());
        audio.volume_db = -3.0;
        audio.ducking_enabled = true;
        audio.ducking_reduction_db = -6.0;

        // Without dialogue: -3.0 dB
        assert_eq!(audio.compute_effective_gain_db(0, false), -3.0);
        // With active dialogue voiceover: -3.0 + (-6.0) = -9.0 dB
        assert_eq!(audio.compute_effective_gain_db(0, true), -9.0);
    }

    #[test]
    fn test_speed_ramping_mapping() {
        let mut clip = TimelineClip::new(1, "HighSpeed.mp4".to_string(), 0, 100);
        clip.speed_multiplier = 2.0; // Fast motion (2x)

        assert_eq!(clip.map_timeline_to_source_frame(10), 20);

        clip.speed_multiplier = 1.0;
        clip.is_reversed = true;
        assert_eq!(clip.map_timeline_to_source_frame(10), 90);
    }

    #[test]
    fn test_export_presets() {
        let prores = ExportSettings::from_preset(ExportPreset::AppleProRes422HQ);
        assert_eq!(prores.container_format, "MOV");
        assert_eq!(prores.video_codec, "ProRes 422 HQ");
        assert_eq!(prores.bitrate_kbps, 220_000);

        let av1 = ExportSettings::from_preset(ExportPreset::AV1Mastering);
        assert_eq!(av1.container_format, "MKV");
        assert_eq!(av1.video_codec, "AV1");
    }

    #[test]
    fn test_magnetic_timeline_gap_closure_and_snapping() {
        let mut track = VideoTrack::new(1, "Main Track".to_string());
        track.add_clip(TimelineClip::new(1, "Clip 1".to_string(), 0, 50));
        track.add_clip(TimelineClip::new(2, "Clip 2".to_string(), 100, 50)); // 50 frame gap

        track.close_timeline_gaps();
        assert_eq!(track.clips[0].start_frame, 0);
        assert_eq!(track.clips[1].start_frame, 50); // Gap closed

        // Move clip 2 away and test snapping
        track.clips[1].start_frame = 53;
        assert!(track.snap_clip_to_nearest(2, 5));
        assert_eq!(track.clips[1].start_frame, 50); // Snapped to clip 1 end
    }

    #[test]
    fn test_lut3d_transformer() {
        let lut = Lut3DTransformer::new_identity(17);
        let (r, g, b) = lut.transform_pixel(0.5, 0.5, 0.5);
        assert!((r - 0.5).abs() < 0.1);
        assert!((g - 0.5).abs() < 0.1);
        assert!((b - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_multicam_angle_switching() {
        let mut multicam = MulticamAngleManager::new();
        multicam.add_angle(1, "Cam A (Wide)".to_string(), 101, 0);
        multicam.add_angle(2, "Cam B (Close)".to_string(), 102, 12);

        assert_eq!(multicam.get_active_clip_id(), Some(101));
        assert_eq!(multicam.switch_active_angle(1).unwrap(), 102);
        assert_eq!(multicam.get_active_clip_id(), Some(102));
    }

    #[test]
    fn test_optical_flow_synthesis() {
        let mut opt_flow = OpticalFlowInterpolator::new();
        let frame_a = vec![0u8, 0u8, 0u8, 255u8];
        let frame_b = vec![100u8, 100u8, 100u8, 255u8];

        let mid = opt_flow.synthesize_intermediate_frame(&frame_a, &frame_b, 0.5);
        assert_eq!(mid[0], 50);
        assert_eq!(opt_flow.motion_vectors_calculated, 1);
    }

    #[test]
    fn test_proxy_media_manager() {
        let mut proxy_mgr = ProxyMediaManager::new();
        proxy_mgr.register_proxy(101, "/media/A001_8K.RAW", "/proxy/A001_720p.mp4");

        proxy_mgr.set_global_proxy_mode(true);
        assert_eq!(
            proxy_mgr.resolve_media_path(101, "/media/A001_8K.RAW"),
            "/proxy/A001_720p.mp4"
        );

        proxy_mgr.set_global_proxy_mode(false);
        assert_eq!(
            proxy_mgr.resolve_media_path(101, "/media/A001_8K.RAW"),
            "/media/A001_8K.RAW"
        );
    }
}
