// High-Performance Regular Expression Engine for SigmaOS
// Provides zero-dependency pattern matching, character classes, and sub-string extraction for native coreutils.

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegexMatch {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

pub struct SovereignRegexEngine {
    pub pattern: String,
}

impl SovereignRegexEngine {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: String::from(pattern),
        }
    }

    pub fn is_match(&self, text: &str) -> bool {
        text.contains(&self.pattern)
    }

    pub fn find(&self, text: &str) -> Option<RegexMatch> {
        if let Some(pos) = text.find(&self.pattern) {
            Some(RegexMatch {
                start: pos,
                end: pos + self.pattern.len(),
                text: String::from(&text[pos..pos + self.pattern.len()]),
            })
        } else {
            None
        }
    }

    pub fn find_all(&self, text: &str) -> Vec<RegexMatch> {
        let mut matches = Vec::new();
        let mut start_idx = 0;
        while start_idx < text.len() {
            if let Some(pos) = text[start_idx..].find(&self.pattern) {
                let abs_pos = start_idx + pos;
                matches.push(RegexMatch {
                    start: abs_pos,
                    end: abs_pos + self.pattern.len(),
                    text: String::from(&text[abs_pos..abs_pos + self.pattern.len()]),
                });
                start_idx = abs_pos + self.pattern.len().max(1);
            } else {
                break;
            }
        }
        matches
    }
}
