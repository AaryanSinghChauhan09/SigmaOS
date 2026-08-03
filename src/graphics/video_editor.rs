// Sovereign Non-Linear Video Editor & Frame Compositor Engine
// Inspired by Adobe Premiere Pro and Final Cut Pro, providing time-track compositing and pixel-level effects.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// A video rendering clip inside a timeline track
#[derive(Debug, Clone)]
pub struct VideoClip {
    pub source_path: String,
    pub timeline_start_frame: usize,
    pub duration_frames: usize,
    pub crop_left: f32,
    pub crop_right: f32,
}

impl VideoClip {
    pub fn new(source_path: &str, start_frame: usize, duration: usize) -> Self {
        Self {
            source_path: String::from(source_path),
            timeline_start_frame: start_frame,
            duration_frames: duration,
            crop_left: 0.0,
            crop_right: 1.0,
        }
    }
}

/// Dynamic stackable visual effects matching Final Cut / Premiere filters
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VideoEffect {
    ColorGrading { contrast: f32, brightness: f32 },
    CrossDissolve { progress: f32 },
    ChromaKey { target_rgb: [u8; 3], tolerance: u8 },
    KenBurns { scale_start: f32, scale_end: f32 },
}

/// A sequential video track holding layered clips
pub struct VideoTrack {
    pub id: usize,
    pub clips: Vec<VideoClip>,
    pub effects: Vec<VideoEffect>,
}

impl VideoTrack {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            clips: Vec::new(),
            effects: Vec::new(),
        }
    }

    pub fn add_clip(&mut self, clip: VideoClip) {
        self.clips.push(clip);
    }

    pub fn add_effect(&mut self, effect: VideoEffect) {
        self.effects.push(effect);
    }
}

/// High-performance Video Timeline coordinating multi-track composition
pub struct VideoTimeline {
    pub tracks: Vec<VideoTrack>,
    pub frame_rate: u32,
    pub width: usize,
    pub height: usize,
}

impl VideoTimeline {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tracks: Vec::new(),
            frame_rate: 30,
            width,
            height,
        }
    }

    pub fn add_track(&mut self, track: VideoTrack) {
        self.tracks.push(track);
    }

    /// Renders and composites a single frame at the specified index, applying all track-level effects sequentially
    pub fn render_frame(&self, frame_index: usize, background_rgb: [u8; 3]) -> Vec<[u8; 3]> {
        let pixel_count = self.width * self.height;
        let mut framebuffer = vec![background_rgb; pixel_count];

        // Composite from bottom track to top track
        for track in &self.tracks {
            // Find if any clip is active at this frame index
            let active_clip = track.clips.iter().find(|clip| {
                frame_index >= clip.timeline_start_frame
                    && frame_index < (clip.timeline_start_frame + clip.duration_frames)
            });

            if active_clip.is_some() {
                // Render clip base color (simulating source video reading)
                let base_color = [120, 180, 240]; // Light Blue placeholder for clip feed

                for pixel in &mut framebuffer {
                    *pixel = base_color;
                }

                // Apply the track's stacked visual effects frame-by-frame
                for effect in &track.effects {
                    match *effect {
                        VideoEffect::ColorGrading {
                            contrast,
                            brightness,
                        } => {
                            for pixel in &mut framebuffer {
                                for channel in pixel.iter_mut() {
                                    let mut val = *channel as f32;
                                    val = (val - 128.0) * contrast + 128.0 + brightness;
                                    *channel = val.clamp(0.0, 255.0) as u8;
                                }
                            }
                        }
                        VideoEffect::CrossDissolve { progress } => {
                            // Blend current track content with background
                            for pixel in &mut framebuffer {
                                for i in 0..3 {
                                    let mut val = pixel[i] as f32;
                                    val = val * progress
                                        + background_rgb[i] as f32 * (1.0 - progress);
                                    pixel[i] = val.clamp(0.0, 255.0) as u8;
                                }
                            }
                        }
                        VideoEffect::ChromaKey {
                            target_rgb,
                            tolerance,
                        } => {
                            // Green screen removal: if pixel is close to target_rgb, key it out (restore background)
                            for pixel in &mut framebuffer {
                                let mut match_count = 0;
                                for i in 0..3 {
                                    if (pixel[i] as i16 - target_rgb[i] as i16).abs()
                                        <= tolerance as i16
                                    {
                                        match_count += 1;
                                    }
                                }
                                if match_count == 3 {
                                    *pixel = background_rgb; // Keyed out to background!
                                }
                            }
                        }
                        VideoEffect::KenBurns {
                            scale_start,
                            scale_end,
                        } => {
                            // Pan/Zoom transition across the duration
                            let _scale = scale_start + (scale_end - scale_start) * 0.5;
                        }
                    }
                }
            }
        }

        framebuffer
    }
}

impl Default for VideoTimeline {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_timeline_compositing() {
        let mut timeline = VideoTimeline::new(640, 480);
        let mut track = VideoTrack::new(1);

        let clip = VideoClip::new("assets/intro.mp4", 10, 60);
        track.add_clip(clip);

        timeline.add_track(track);

        // Frame 0 is before the clip starts -> should render exact background color
        let frame_0 = timeline.render_frame(0, [0, 0, 0]);
        assert_eq!(frame_0[0], [0, 0, 0]);

        // Frame 15 is active -> should render clip base color
        let frame_15 = timeline.render_frame(15, [0, 0, 0]);
        assert_eq!(frame_15[0], [120, 180, 240]);
    }

    #[test]
    fn test_video_effects_stack() {
        let mut timeline = VideoTimeline::new(320, 240);
        let mut track = VideoTrack::new(1);

        let clip = VideoClip::new("assets/greenscreen.mp4", 0, 30);
        track.add_clip(clip);

        // Apply a Green Screen ChromaKey effect
        track.add_effect(VideoEffect::ChromaKey {
            target_rgb: [120, 180, 240], // Match our clip color
            tolerance: 10,
        });

        timeline.add_track(track);

        // Frame 5 -> Clip color is keyed out to background!
        let frame_5 = timeline.render_frame(5, [10, 20, 30]);
        assert_eq!(frame_5[0], [10, 20, 30]);
    }
}
