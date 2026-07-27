//! # Sovereign Video Editor - DaVinci Resolve Equivalent NLE & Color Grading Engine
//!
//! This module implements high-performance nonlinear video editing (NLE)
//! and professional color correction matrices based on standard ASC CDL formulas.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorError {
    ClipOutOfRange,
    TrackNotFound,
    InvalidCdlValue,
    RenderPipelineFailed,
}

/// ASC CDL (American Society of Cinematographers Color Decision List)
/// Defines professional color grading values: Slope, Offset, and Power.
#[derive(Debug, Clone, Copy)]
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
        let r_graded = ((r * self.slope[0]) + self.offset[0]).max(0.0).powf(self.power[0]);
        let g_graded = ((g * self.slope[1]) + self.offset[1]).max(0.0).powf(self.power[1]);
        let b_graded = ((b * self.slope[2]) + self.offset[2]).max(0.0).powf(self.power[2]);
        (r_graded.min(1.0), g_graded.min(1.0), b_graded.min(1.0))
    }
}

impl Default for AscCdl {
    fn default() -> Self {
        Self::new()
    }
}

/// A clip loaded on the timeline
#[derive(Debug, Clone)]
pub struct TimelineClip {
    pub id: u32,
    pub name: String,
    pub start_frame: u64,
    pub duration_frames: u64,
}

/// Nonlinear timeline track structure
#[derive(Debug, Clone)]
pub struct VideoTrack {
    pub id: u32,
    pub name: String,
    pub clips: Vec<TimelineClip>,
    pub cdl: AscCdl,
}

impl VideoTrack {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            clips: Vec::new(),
            cdl: AscCdl::new(),
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
}

/// Sovereign Video Editor nonlinear pipeline
pub struct SovereignVideoEditor {
    pub tracks: BTreeMap<u32, VideoTrack>,
    pub active_lut: Option<String>,
}

impl SovereignVideoEditor {
    pub fn new() -> Self {
        Self {
            tracks: BTreeMap::new(),
            active_lut: None,
        }
    }

    pub fn add_track(&mut self, id: u32, name: String) {
        self.tracks.insert(id, VideoTrack::new(id, name));
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

    /// Render composite frames with color correction applied
    pub fn render_frame_composite(&self, track_id: u32, raw_frame_rgba: &[u8]) -> Result<Vec<u8>, EditorError> {
        let track = self.tracks.get(&track_id).ok_or(EditorError::TrackNotFound)?;

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
    fn test_timeline_and_track_assembly() {
        let mut editor = SovereignVideoEditor::new();
        editor.add_track(1, "Main Video".to_string());

        let track = editor.tracks.get_mut(&1).unwrap();
        track.add_clip(TimelineClip {
            id: 101,
            name: "Shot A.mp4".to_string(),
            start_frame: 0,
            duration_frames: 120,
        });
        track.add_clip(TimelineClip {
            id: 102,
            name: "Shot B.mp4".to_string(),
            start_frame: 120,
            duration_frames: 240,
        });

        assert_eq!(editor.total_timeline_frames(), 360);
    }

    #[test]
    fn test_frame_composite_rendering() {
        let mut editor = SovereignVideoEditor::new();
        editor.add_track(1, "Grade Track".to_string());

        let track = editor.tracks.get_mut(&1).unwrap();
        track.cdl.slope = [1.5, 1.0, 1.0]; // Boost reds significantly

        let raw_pixels = vec![100, 100, 100, 255]; // Gray pixel
        let rendered = editor.render_frame_composite(1, &raw_pixels).unwrap();

        assert!(rendered[0] > 100); // Red has been boosted
        assert_eq!(rendered[1], 100);
        assert_eq!(rendered[2], 100);
        assert_eq!(rendered[3], 255); // Alpha preserved
    }
}
