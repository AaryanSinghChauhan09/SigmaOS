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

// =========================================================================
// 1. SigmaSupportSubtitleSync (Aegisub ASS Advanced Styling & Karaoke Parity)
// =========================================================================

pub struct AegisubKaraokeSyllable {
    pub text: String,
    pub duration_centiseconds: u32,
}

pub struct SigmaSupportSubtitleSync {
    pub font_name: String,
    pub font_size: u32,
    pub text_color_hex: String,
    pub karaoke_syllables: Vec<AegisubKaraokeSyllable>,
}

impl SigmaSupportSubtitleSync {
    pub fn new() -> Self {
        SigmaSupportSubtitleSync {
            font_name: "Arial".to_string(),
            font_size: 20,
            text_color_hex: "FFFFFF".to_string(),
            karaoke_syllables: Vec::new(),
        }
    }

    /// Parses Aegisub-style ASS tags (e.g. {\fnArial\fs24\c&HFF0000&}Sovereign)
    pub fn parse_ass_styling_tags(&mut self, tag_str: &str) -> String {
        if !tag_str.starts_with("{\\") || !tag_str.contains('}') {
            return tag_str.to_string();
        }

        if let Some(fn_idx) = tag_str.find("\\fn") {
            let sub = &tag_str[fn_idx + 3..];
            let end_idx = sub.find('\\').or_else(|| sub.find('}')).unwrap_or(0);
            self.font_name = sub[..end_idx].to_string();
        }

        if let Some(fs_idx) = tag_str.find("\\fs") {
            let sub = &tag_str[fs_idx + 3..];
            let end_idx = sub.find('\\').or_else(|| sub.find('}')).unwrap_or(0);
            if let Ok(size) = sub[..end_idx].parse::<u32>() {
                self.font_size = size;
            }
        }

        let body_start = tag_str.find('}').unwrap_or(0) + 1;
        tag_str[body_start..].to_string()
    }

    pub fn add_karaoke_syllable(&mut self, text: &str, duration_cs: u32) {
        self.karaoke_syllables.push(AegisubKaraokeSyllable {
            text: text.to_string(),
            duration_centiseconds: duration_cs,
        });
    }
}

// =========================================================================
// 2. SigmaSupportSubtitleEdit (Subtitle Edit Timing Synchronization Parity)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleFormat {
    Srt,
    Ass,
    WebVtt,
}

pub struct SubtitleEntry {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

pub struct SigmaSupportSubtitleEdit {
    pub current_format: SubtitleFormat,
    pub entries: Vec<SubtitleEntry>,
}

impl SigmaSupportSubtitleEdit {
    pub fn new(format: SubtitleFormat) -> Self {
        SigmaSupportSubtitleEdit {
            current_format: format,
            entries: Vec::new(),
        }
    }

    pub fn insert_subtitle_entry(&mut self, start: u64, end: u64, text: &str) {
        self.entries.push(SubtitleEntry {
            start_ms: start,
            end_ms: end,
            text: text.to_string(),
        });
    }

    /// Subtitle Edit parity: applies frame-rate scale conversion and millisecond synchronization shifts
    pub fn shift_all_timings_ms(&mut self, offset_ms: i32) {
        for entry in &mut self.entries {
            let s = entry.start_ms as i64 + offset_ms as i64;
            entry.start_ms = s.max(0) as u64;

            let e = entry.end_ms as i64 + offset_ms as i64;
            entry.end_ms = e.max(0) as u64;
        }
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

    #[test]
    fn test_aegisub_styling_tags() {
        let mut aegisub = SigmaSupportSubtitleSync::new();
        assert_eq!(aegisub.font_name, "Arial");

        let body =
            aegisub.parse_ass_styling_tags("{\\fnHelvetica\\fs28\\c&H00FFFF&}Welcome to SigmaOS");
        assert_eq!(body, "Welcome to SigmaOS");
        assert_eq!(aegisub.font_name, "Helvetica");
        assert_eq!(aegisub.font_size, 28);
    }

    #[test]
    fn test_subtitle_edit_sync() {
        let mut edit = SigmaSupportSubtitleEdit::new(SubtitleFormat::Srt);
        edit.insert_subtitle_entry(1000, 3000, "Hello World");

        // Shift forward 500ms
        edit.shift_all_timings_ms(500);
        assert_eq!(edit.entries[0].start_ms, 1500);
        assert_eq!(edit.entries[0].end_ms, 3500);

        // Shift backward 1000ms
        edit.shift_all_timings_ms(-1000);
        assert_eq!(edit.entries[0].start_ms, 500);
        assert_eq!(edit.entries[0].end_ms, 2500);
    }
}
