// SigmaOS Polish-Parity Out-of-the-Box Codecs & Multimedia Engine (SigmaMedia)
// Designed for chiptune synthesizers, audio playing, and decoders with zero dependencies

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFormat {
    Mp3,
    Wav,
    Pcm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

pub struct AudioTrack {
    pub name: String,
    pub format: MediaFormat,
    pub duration_secs: u32,
    pub volume: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleFormat {
    Srt,
    Ass, // Advanced Substation Alpha (Aegisub standard)
    WebVtt,
}

#[derive(Debug, Clone)]
pub struct SubtitleEntry {
    pub id: u32,
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
    pub style: String, // ASS style definition (e.g. Default)
}

pub struct SubtitleSyncManager {
    pub entries: Vec<SubtitleEntry>,
}

impl SubtitleSyncManager {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: SubtitleEntry) {
        self.entries.push(entry);
    }

    /// Subtitle Edit-style positive/negative timing delays shift (offset milliseconds)
    pub fn shift_timings(&mut self, offset_ms: i64) {
        for entry in &mut self.entries {
            let new_start = (entry.start_ms as i64).saturating_add(offset_ms);
            let new_end = (entry.end_ms as i64).saturating_add(offset_ms);

            entry.start_ms = new_start.max(0) as u64;
            entry.end_ms = new_end.max(0) as u64;
        }
    }

    /// Subtitle Edit-style framerate scale conversion (e.g. from 23.976 to 29.97 fps)
    pub fn scale_timings(&mut self, from_fps: f64, to_fps: f64) {
        if from_fps == 0.0 || to_fps == 0.0 {
            return;
        }
        let factor = to_fps / from_fps;
        for entry in &mut self.entries {
            entry.start_ms = ((entry.start_ms as f64) * factor) as u64;
            entry.end_ms = ((entry.end_ms as f64) * factor) as u64;
        }
    }
}

pub struct AegisubStyleEngine;

impl AegisubStyleEngine {
    /// Parses Aegisub ASS style overrides (like {\fnArial\fs24\b1\c&H0000FF&} Red Text)
    pub fn parse_ass_overrides(&self, text: &str) -> (HashMap<String, String>, String) {
        let mut props = HashMap::new();
        let mut clean_text = String::new();

        let mut in_override = false;
        let mut override_block = String::new();

        for ch in text.chars() {
            if ch == '{' {
                in_override = true;
                override_block.clear();
            } else if ch == '}' {
                in_override = false;
                // Parse properties out of the override block (e.g., \fnArial\fs24\b1)
                self.extract_properties(&override_block, &mut props);
            } else if in_override {
                override_block.push(ch);
            } else {
                clean_text.push(ch);
            }
        }

        (props, clean_text)
    }

    /// Emulates Karaoke centisecond offset parsing inside tags (e.g., {\k50}Ka{\k40}ra{\k60}oke)
    /// Returns total centisecond duration
    pub fn calculate_karaoke_duration(&self, text: &str) -> u32 {
        let mut total_duration = 0;
        let mut in_tag = false;
        let mut tag_content = String::new();

        for ch in text.chars() {
            if ch == '{' {
                in_tag = true;
                tag_content.clear();
            } else if ch == '}' {
                in_tag = false;
                if tag_content.starts_with("\\k") {
                    if let Ok(duration) = tag_content[2..].parse::<u32>() {
                        total_duration += duration;
                    }
                }
            } else if in_tag {
                tag_content.push(ch);
            }
        }

        total_duration
    }

    fn extract_properties(&self, block: &str, props: &mut HashMap<String, String>) {
        // Simple sequential regex-like scan for standard Aegisub tags
        // \fn[FontName]
        if let Some(pos) = block.find("\\fn") {
            let start = pos + 3;
            let end = block[start..].find('\\').map(|x| start + x).unwrap_or(block.len());
            props.insert("font_name".to_string(), block[start..end].to_string());
        }
        // \fs[FontSize]
        if let Some(pos) = block.find("\\fs") {
            let start = pos + 3;
            let end = block[start..].find('\\').map(|x| start + x).unwrap_or(block.len());
            props.insert("font_size".to_string(), block[start..end].to_string());
        }
        // \b[Bold]
        if let Some(pos) = block.find("\\b") {
            let val = &block[pos + 2..pos + 3];
            props.insert("bold".to_string(), val.to_string());
        }
    }
}

use std::collections::HashMap;

pub struct SigmaMediaEngine {
    pub current_track: Option<AudioTrack>,
    pub state: PlaybackState,
}

impl SigmaMediaEngine {
    pub fn new() -> Self {
        SigmaMediaEngine {
            current_track: None,
            state: PlaybackState::Stopped,
        }
    }

    pub fn load_track(&mut self, name: String, format: MediaFormat, duration: u32) {
        let track = AudioTrack {
            name,
            format,
            duration_secs: duration,
            volume: 0.8,
        };
        self.current_track = Some(track);
        self.state = PlaybackState::Stopped;
    }

    pub fn play(&mut self) -> Result<(), ()> {
        if self.current_track.is_some() {
            self.state = PlaybackState::Playing;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_playback() {
        let mut engine = SigmaMediaEngine::new();
        assert_eq!(engine.state, PlaybackState::Stopped);
        assert!(engine.play().is_err());

        engine.load_track("Symphony-9.mp3".to_string(), MediaFormat::Mp3, 340);
        assert_eq!(engine.state, PlaybackState::Stopped);

        assert!(engine.play().is_ok());
        assert_eq!(engine.state, PlaybackState::Playing);

        engine.pause();
        assert_eq!(engine.state, PlaybackState::Paused);

        engine.stop();
        assert_eq!(engine.state, PlaybackState::Stopped);
    }
}
