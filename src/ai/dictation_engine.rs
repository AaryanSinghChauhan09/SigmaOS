// SigmaOS Sovereign Text Dictation & Speech-to-Text Engine
// Zero-dependency Rust #![no_std] / std implementation of local voice dictation & punctuation formatting.

#[cfg(not(test))]
use alloc::string::{String, ToString};
#[cfg(not(test))]
use alloc::vec::Vec;
#[cfg(not(test))]
use alloc::format;

#[cfg(test)]
use std::string::String;

/// Dictation Engine State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictationState {
    Idle,
    Listening,
    Processing,
    PausedForPassword,
}

/// Text Dictation Engine
#[derive(Debug, Clone)]
pub struct SovereignDictationEngine {
    pub state: DictationState,
    pub active_language: String,
    pub auto_punctuation: bool,
    pub silence_timeout_ms: u64,
}

impl SovereignDictationEngine {
    pub fn new() -> Self {
        Self {
            state: DictationState::Idle,
            active_language: String::from("en-US"),
            auto_punctuation: true,
            silence_timeout_ms: 3000,
        }
    }

    pub fn start_listening(&mut self) -> bool {
        if self.state == DictationState::PausedForPassword {
            return false;
        }
        self.state = DictationState::Listening;
        true
    }

    pub fn stop_listening(&mut self) {
        self.state = DictationState::Idle;
    }

    pub fn handle_password_focus(&mut self, is_password_field: bool) {
        if is_password_field {
            self.state = DictationState::PausedForPassword;
        } else if self.state == DictationState::PausedForPassword {
            self.state = DictationState::Idle;
        }
    }

    /// Processes spoken raw tokens and applies voice formatting rules
    pub fn format_dictated_text(&self, raw_transcript: &str) -> String {
        let mut result = String::from(raw_transcript);

        if self.auto_punctuation {
            result = result.replace(" period", ".");
            result = result.replace(" full stop", ".");
            result = result.replace(" comma", ",");
            result = result.replace(" question mark", "?");
            result = result.replace(" exclamation mark", "!");
            result = result.replace(" new line ", "\n");
            result = result.replace(" new line", "\n");
            result = result.replace(" newline ", "\n");
            result = result.replace(" newline", "\n");
            result = result.replace(" new paragraph ", "\n\n");
            result = result.replace(" new paragraph", "\n\n");
        }

        result
    }
}

impl Default for SovereignDictationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictation_engine_formatting_and_state() {
        let mut engine = SovereignDictationEngine::new();

        assert_eq!(engine.state, DictationState::Idle);
        assert!(engine.start_listening());
        assert_eq!(engine.state, DictationState::Listening);

        // Test password safety pause
        engine.handle_password_focus(true);
        assert_eq!(engine.state, DictationState::PausedForPassword);
        assert!(!engine.start_listening());

        engine.handle_password_focus(false);
        assert_eq!(engine.state, DictationState::Idle);

        // Test formatting rules
        let formatted = engine.format_dictated_text("hello world period new line welcome to sigmaos comma enjoy your stay");
        assert_eq!(formatted, "hello world.\nwelcome to sigmaos, enjoy your stay");
    }
}
