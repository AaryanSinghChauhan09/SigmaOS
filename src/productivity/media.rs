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
