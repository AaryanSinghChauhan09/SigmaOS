// SigmaOS Sovereign AI-Native Video Editing Suite (SigmaCut)
// Designed for high-performance timeline composition, YUV translation, and overlay effects

/// Video processing error states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoError {
    Success = 0,
    InvalidFrame = 1,
    TimelineConflict = 2,
    NotSupported = 3,
    RenderFailed = 4,
}

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl PixelRgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        PixelRgba { r, g, b, a }
    }
}

/// Video frame representation
#[derive(Debug, Clone)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<PixelRgba>,
}

impl VideoFrame {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut pixels = Vec::new();
        for _ in 0..size {
            pixels.push(PixelRgba::new(0, 0, 0, 255));
        }
        VideoFrame {
            width,
            height,
            pixels,
        }
    }
}

/// Base OOP interface representing any video transition or filter effect
pub trait VideoEffect {
    fn process_frame(&self, frame: &mut VideoFrame) -> Result<(), VideoError>;
}

/// Base OOP interface representing a media clip on the timeline track
pub trait TimelineClip {
    fn name(&self) -> &str;
    fn start_frame(&self) -> u32;
    fn end_frame(&self) -> u32;
    fn get_frame(&self, offset_frame: u32) -> Result<VideoFrame, VideoError>;
}

// ==========================================
// 1. Concrete YUV-to-RGB Conversion Effect
// ==========================================

pub struct YuvToRgbEffect;

impl VideoEffect for YuvToRgbEffect {
    fn process_frame(&self, frame: &mut VideoFrame) -> Result<(), VideoError> {
        // Standard BT.601 fixed-point integer color space translation
        // For demonstration, simulate processing YUV inputs mapping directly into the RGBA frame.
        for pixel in frame.pixels.iter_mut() {
            let y: i32 = pixel.r as i32; // Map red field as Y channel for mock YUV inputs
            let u: i32 = pixel.g as i32 - 128; // Map green field as U channel
            let v: i32 = pixel.b as i32 - 128; // Map blue field as V channel

            // BT.601 integer coefficients
            let r = (y + ((91881 * v) >> 16)).clamp(0, 255);
            let g = (y - ((22554 * u + 46802 * v) >> 16)).clamp(0, 255);
            let b = (y + ((116130 * u) >> 16)).clamp(0, 255);

            pixel.r = r as u8;
            pixel.g = g as u8;
            pixel.b = b as u8;
            pixel.a = 255;
        }
        Ok(())
    }
}

// ==========================================
// 2. Concrete Text/Subtitle Overlay Effect
// ==========================================

pub struct SubtitleOverlayEffect {
    pub subtitle_text: String,
    pub font_size: u32,
    pub color: PixelRgba,
}

impl SubtitleOverlayEffect {
    pub fn new(text: String, size: u32, color: PixelRgba) -> Self {
        SubtitleOverlayEffect {
            subtitle_text: text,
            font_size: size,
            color,
        }
    }
}

impl VideoEffect for SubtitleOverlayEffect {
    fn process_frame(&self, frame: &mut VideoFrame) -> Result<(), VideoError> {
        if frame.width == 0 || frame.height == 0 {
            return Err(VideoError::InvalidFrame);
        }

        // Draw simple horizontal bar overlay matching the subtitle text area on lower third of frame
        let start_y = (frame.height * 4 / 5) as usize;
        let end_y = (start_y + self.font_size as usize).min(frame.height as usize);
        let start_x = (frame.width / 10) as usize;
        let end_x = (frame.width * 9 / 10) as usize;

        for y in start_y..end_y {
            for x in start_x..end_x {
                let idx = y * frame.width as usize + x;
                // Simple alpha blending overlay
                let bg = frame.pixels[idx];
                let alpha = self.color.a as f32 / 255.0;
                frame.pixels[idx] = PixelRgba::new(
                    ((self.color.r as f32 * alpha) + (bg.r as f32 * (1.0 - alpha))) as u8,
                    ((self.color.g as f32 * alpha) + (bg.g as f32 * (1.0 - alpha))) as u8,
                    ((self.color.b as f32 * alpha) + (bg.b as f32 * (1.0 - alpha))) as u8,
                    255,
                );
            }
        }
        Ok(())
    }
}

// ==========================================
// 3. Concrete Video Clip Timeline Element
// ==========================================

pub struct VideoClip {
    pub name: String,
    pub start: u32,
    pub duration: u32,
    pub width: u32,
    pub height: u32,
}

impl VideoClip {
    pub fn new(name: String, start: u32, duration: u32, w: u32, h: u32) -> Self {
        VideoClip {
            name,
            start,
            duration,
            width: w,
            height: h,
        }
    }
}

impl TimelineClip for VideoClip {
    fn name(&self) -> &str {
        &self.name
    }
    fn start_frame(&self) -> u32 {
        self.start
    }
    fn end_frame(&self) -> u32 {
        self.start + self.duration
    }
    fn get_frame(&self, _offset_frame: u32) -> Result<VideoFrame, VideoError> {
        Ok(VideoFrame::new(self.width, self.height))
    }
}

// ==========================================
// 4. Concrete Streaming Overlay Manager (Streamlabs & XSplit)
// ==========================================

use std::collections::HashMap;

/// Type of overlay source (Webcam, Game Capture, Chat Box, Alerts, Labels)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlaySourceType {
    Webcam,
    GameCapture,
    ChatBox,
    AlertBox,
    StreamLabel,
}

/// Represents an individual overlay item positioned in a scene
#[derive(Debug, Clone)]
pub struct OverlayItem {
    pub id: String,
    pub source_type: OverlaySourceType,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub opacity: f32, // 0.0 to 1.0
    pub z_index: u32,  // Render order layer
}

/// Represents a configured scene consisting of prioritized overlay items
#[derive(Debug, Clone)]
pub struct StreamScene {
    pub name: String,
    pub overlays: Vec<OverlayItem>,
}

impl StreamScene {
    pub fn new(name: &str) -> Self {
        StreamScene {
            name: name.to_string(),
            overlays: Vec::new(),
        }
    }

    pub fn add_overlay(&mut self, item: OverlayItem) {
        self.overlays.push(item);
        // Sort overlays by z_index ascending so higher z-index gets drawn last (on top)
        self.overlays.sort_by_key(|o| o.z_index);
    }
}

/// Dynamic alert trigger state
#[derive(Debug, Clone)]
pub struct ActiveAlert {
    pub message: String,
    pub duration_frames: u32,
    pub frames_remaining: u32,
}

/// High-Performance Streaming Overlay Manager matching Streamlabs & XSplit capabilities
pub struct StreamingOverlayManager {
    pub scenes: HashMap<String, StreamScene>,
    pub active_scene_name: String,
    pub transition_type: String, // "cut", "fade"
    pub transition_frames: u32,
    pub active_alert: Option<ActiveAlert>,
}

impl StreamingOverlayManager {
    pub fn new() -> Self {
        StreamingOverlayManager {
            scenes: HashMap::new(),
            active_scene_name: String::new(),
            transition_type: "cut".to_string(),
            transition_frames: 0,
            active_alert: None,
        }
    }

    pub fn register_scene(&mut self, scene: StreamScene) {
        if self.active_scene_name.is_empty() {
            self.active_scene_name = scene.name.clone();
        }
        self.scenes.insert(scene.name.clone(), scene);
    }

    pub fn switch_scene(&mut self, scene_name: &str, transition: &str, duration_frames: u32) -> Result<(), &'static str> {
        if !self.scenes.contains_key(scene_name) {
            return Err("Scene not registered in overlay manager");
        }
        self.active_scene_name = scene_name.to_string();
        self.transition_type = transition.to_string();
        self.transition_frames = duration_frames;
        Ok(())
    }

    pub fn trigger_alert(&mut self, message: &str, duration_frames: u32) {
        self.active_alert = Some(ActiveAlert {
            message: message.to_string(),
            duration_frames,
            frames_remaining: duration_frames,
        });
    }

    /// Renders all active layered scene overlays onto the base video frame
    pub fn render_stream_frame(&mut self, frame: &mut VideoFrame) -> Result<(), VideoError> {
        if frame.width == 0 || frame.height == 0 {
            return Err(VideoError::InvalidFrame);
        }

        // 1. Render active scene layers
        if let Some(scene) = self.scenes.get(&self.active_scene_name) {
            for overlay in &scene.overlays {
                // Determine bounds
                let start_y = overlay.y as usize;
                let end_y = (start_y + overlay.height as usize).min(frame.height as usize);
                let start_x = overlay.x as usize;
                let end_x = (start_x + overlay.width as usize).min(frame.width as usize);

                for y in start_y..end_y {
                    for x in start_x..end_x {
                        let idx = y * frame.width as usize + x;
                        let bg = frame.pixels[idx];

                        // Generate mock source pixel based on type
                        let src_color = match overlay.source_type {
                            OverlaySourceType::Webcam => PixelRgba::new(0, 0, 200, 255),       // Blue-tinted webcam
                            OverlaySourceType::GameCapture => PixelRgba::new(10, 10, 10, 255), // Dark game capture
                            OverlaySourceType::ChatBox => PixelRgba::new(50, 50, 50, 200),    // Dark-gray semi-transparent chat
                            OverlaySourceType::AlertBox => PixelRgba::new(255, 165, 0, 255),   // Orange alert
                            OverlaySourceType::StreamLabel => PixelRgba::new(0, 255, 0, 255),  // Green label
                        };

                        let alpha = overlay.opacity;
                        frame.pixels[idx] = PixelRgba::new(
                            ((src_color.r as f32 * alpha) + (bg.r as f32 * (1.0 - alpha))) as u8,
                            ((src_color.g as f32 * alpha) + (bg.g as f32 * (1.0 - alpha))) as u8,
                            ((src_color.b as f32 * alpha) + (bg.b as f32 * (1.0 - alpha))) as u8,
                            255,
                        );
                    }
                }
            }
        }

        // 2. Render real-time alert overlay if triggered and active
        if let Some(ref mut alert) = self.active_alert {
            if alert.frames_remaining > 0 {
                // Draw alert banner across the top-center of the stream
                let start_y = (frame.height / 10) as usize;
                let end_y = (start_y + 15).min(frame.height as usize);
                let start_x = (frame.width / 4) as usize;
                let end_x = (frame.width * 3 / 4) as usize;

                for y in start_y..end_y {
                    for x in start_x..end_x {
                        let idx = y * frame.width as usize + x;
                        // Bright orange overlay representing Streamlabs / XSplit styled Alert Notification
                        frame.pixels[idx] = PixelRgba::new(255, 69, 0, 255);
                    }
                }
                alert.frames_remaining -= 1;
            }
        }

        // Clean up alert if duration expired
        if let Some(ref alert) = self.active_alert {
            if alert.frames_remaining == 0 {
                self.active_alert = None;
            }
        }

        Ok(())
    }
}

impl Default for StreamingOverlayManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_clip_creation() {
        let clip = VideoClip::new("ClipA.mp4".to_string(), 0, 120, 1920, 1080);
        assert_eq!(clip.name(), "ClipA.mp4");
        assert_eq!(clip.start_frame(), 0);
        assert_eq!(clip.end_frame(), 120);
    }

    #[test]
    fn test_subtitle_overlay() {
        let mut frame = VideoFrame::new(100, 100);
        let overlay = SubtitleOverlayEffect::new(
            "Hello World".to_string(),
            10,
            PixelRgba::new(255, 0, 0, 128),
        );
        overlay.process_frame(&mut frame).unwrap();
        // Check that pixels in lower fifth of frame have been blended
        let idx = 85 * 100 + 50;
        let r_val = frame.pixels[idx].r;
        assert!(r_val == 127 || r_val == 128); // blended default 0 and overlay 255 at ~0.5 opacity
    }

    #[test]
    fn test_streaming_overlay_manager() {
        let mut manager = StreamingOverlayManager::new();
        assert_eq!(manager.active_scene_name, "");

        // 1. Create a scene
        let mut scene = StreamScene::new("In-Game Layout");
        scene.add_overlay(OverlayItem {
            id: "webcam_feed".to_string(),
            source_type: OverlaySourceType::Webcam,
            x: 10,
            y: 10,
            width: 30,
            height: 30,
            opacity: 0.8,
            z_index: 2,
        });
        scene.add_overlay(OverlayItem {
            id: "game_feed".to_string(),
            source_type: OverlaySourceType::GameCapture,
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            opacity: 1.0,
            z_index: 1, // Layer 1 (background)
        });

        // 2. Register scene and check active
        manager.register_scene(scene);
        assert_eq!(manager.active_scene_name, "In-Game Layout");

        // 3. Switch scene
        let mut starting_soon = StreamScene::new("Starting Soon");
        starting_soon.add_overlay(OverlayItem {
            id: "waiting_label".to_string(),
            source_type: OverlaySourceType::StreamLabel,
            x: 20,
            y: 40,
            width: 60,
            height: 20,
            opacity: 0.9,
            z_index: 1,
        });
        manager.register_scene(starting_soon);

        assert!(manager.switch_scene("Starting Soon", "fade", 30).is_ok());
        assert_eq!(manager.active_scene_name, "Starting Soon");
        assert_eq!(manager.transition_type, "fade");
        assert_eq!(manager.transition_frames, 30);

        // 4. Test rendering frame
        let mut frame = VideoFrame::new(100, 100);
        assert!(manager.render_stream_frame(&mut frame).is_ok());

        // StreamLabel is green (0, 255, 0).
        // Let's verify that a pixel in the label region (x=50, y=50) has a strong green component
        let idx = 50 * 100 + 50;
        assert!(frame.pixels[idx].g > 200);

        // 5. Test alerts triggers
        manager.trigger_alert("New Subscriber!", 3);
        assert!(manager.active_alert.is_some());

        // Process first frame with alert
        assert!(manager.render_stream_frame(&mut frame).is_ok());
        // Alert box is bright orange (255, 69, 0) on the top (y=12)
        let alert_idx = 12 * 100 + 50;
        assert_eq!(frame.pixels[alert_idx].r, 255);
        assert_eq!(frame.pixels[alert_idx].g, 69);
        assert_eq!(manager.active_alert.as_ref().unwrap().frames_remaining, 2);

        // Process remaining frames
        assert!(manager.render_stream_frame(&mut frame).is_ok());
        assert!(manager.render_stream_frame(&mut frame).is_ok());

        // Alert should expire and auto-clean
        assert!(manager.active_alert.is_none());
    }
}
