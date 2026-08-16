// SigmaCut: Native Video Editor for SigmaOS
// Features GPU-accelerated raster timelines, multi-track mixing, and subtitle overlay rendering.

extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackType {
    Video,
    Audio,
    Subtitle,
}

#[derive(Debug, Clone)]
pub struct MediaClip {
    pub id: u32,
    pub title: String,
    pub start_frame: u64,
    pub duration_frames: u64,
    pub z_index: u32,
}

#[derive(Debug, Clone)]
pub struct SubtitleOverlay {
    pub start_frame: u64,
    pub end_frame: u64,
    pub text: String,
}

pub struct SigmaCutEditor {
    pub tracks: Vec<(TrackType, Vec<MediaClip>)>,
    pub subtitles: Vec<SubtitleOverlay>,
    pub frame_rate_fps: u32,
    pub current_scrub_frame: u64,
}

impl SigmaCutEditor {
    pub fn new(fps: u32) -> Self {
        Self {
            tracks: Vec::new(),
            subtitles: Vec::new(),
            frame_rate_fps: fps,
            current_scrub_frame: 0,
        }
    }

    pub fn add_track(&mut self, track_type: TrackType) -> usize {
        self.tracks.push((track_type, Vec::new()));
        self.tracks.len() - 1
    }

    pub fn insert_clip(&mut self, track_idx: usize, id: u32, title: &str, start: u64, duration: u64) -> Result<(), &'static str> {
        let track = self.tracks.get_mut(track_idx).ok_or("Invalid track index")?;
        track.1.push(MediaClip {
            id,
            title: title.to_string(),
            start_frame: start,
            duration_frames: duration,
            z_index: track_idx as u32,
        });
        Ok(())
    }

    pub fn add_subtitle_overlay(&mut self, start: u64, end: u64, text: &str) {
        self.subtitles.push(SubtitleOverlay {
            start_frame: start,
            end_frame: end,
            text: text.to_string(),
        });
    }

    pub fn render_frame_at(&mut self, frame_idx: u64) -> (usize, Option<String>) {
        self.current_scrub_frame = frame_idx;
        let mut active_clips_count = 0;

        for (_, clips) in &self.tracks {
            for clip in clips {
                if frame_idx >= clip.start_frame && frame_idx < clip.start_frame + clip.duration_frames {
                    active_clips_count += 1;
                }
            }
        }

        let mut active_sub = None;
        for sub in &self.subtitles {
            if frame_idx >= sub.start_frame && frame_idx <= sub.end_frame {
                active_sub = Some(sub.text.clone());
                break;
            }
        }

        (active_clips_count, active_sub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmacut_timeline_rendering() {
        let mut editor = SigmaCutEditor::new(60);
        let v_track = editor.add_track(TrackType::Video);
        let a_track = editor.add_track(TrackType::Audio);

        assert!(editor.insert_clip(v_track, 1, "intro.mp4", 0, 120).is_ok()); // 2 seconds
        assert!(editor.insert_clip(a_track, 2, "bgm.mp3", 0, 300).is_ok());   // 5 seconds

        editor.add_subtitle_overlay(30, 90, "Welcome to SigmaOS");

        // Scrub frame 60 (1 second in)
        let (active_clips, sub_text) = editor.render_frame_at(60);
        assert_eq!(active_clips, 2);
        assert_eq!(sub_text, Some("Welcome to SigmaOS".to_string()));

        // Scrub frame 200 (intro finished, audio still playing)
        let (active_clips2, sub_text2) = editor.render_frame_at(200);
        assert_eq!(active_clips2, 1);
        assert_eq!(sub_text2, None);
    }
}
