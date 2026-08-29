#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::vec;

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;

/// Video Timeline Editor (Shotcut Parity)
/// Multi-track sequencer for arranging and transitioning video clips.

pub struct VideoClip {
    pub id: u32,
    pub start_time_ms: u64,
    pub duration_ms: u64,
}

pub struct VideoTrack {
    pub clips: Vec<VideoClip>,
    pub z_index: u32,
}

pub struct VideoTimeline {
    pub tracks: Vec<VideoTrack>,
    pub frame_rate: u32,
}

impl VideoTimeline {
    pub fn new(frame_rate: u32) -> Self {
        Self {
            tracks: Vec::new(),
            frame_rate,
        }
    }

    pub fn add_track(&mut self, track: VideoTrack) {
        self.tracks.push(track);
        self.tracks.sort_by_key(|t| t.z_index);
    }

    /// Calculate total duration of the timeline based on all clips
    pub fn get_total_duration_ms(&self) -> u64 {
        let mut max_duration = 0;
        for track in &self.tracks {
            for clip in &track.clips {
                let end = clip.start_time_ms + clip.duration_ms;
                if end > max_duration {
                    max_duration = end;
                }
            }
        }
        max_duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_timeline_duration() {
        let mut timeline = VideoTimeline::new(60);
        let track = VideoTrack {
            clips: alloc::vec![
                VideoClip {
                    id: 1,
                    start_time_ms: 0,
                    duration_ms: 5000
                },
                VideoClip {
                    id: 2,
                    start_time_ms: 4000,
                    duration_ms: 3000
                },
            ],
            z_index: 0,
        };
        timeline.add_track(track);
        assert_eq!(timeline.get_total_duration_ms(), 7000);
    }
}
