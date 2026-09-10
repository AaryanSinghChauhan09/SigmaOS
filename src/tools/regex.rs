// High-Performance Regular Expression Engine for SigmaOS
// Provides zero-dependency pattern matching, character classes, wildcards, and substring extraction under #![no_std].


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

    /// Tests if the pattern matches anywhere within `text`.
    pub fn is_match(&self, text: &str) -> bool {
        self.find(text).is_some()
    }

    /// Finds the first match of the pattern in `text`.
    pub fn find(&self, text: &str) -> Option<RegexMatch> {
        if self.pattern.is_empty() {
            return Some(RegexMatch {
                start: 0,
                end: 0,
                text: String::new(),
            });
        }

        let char_indices: Vec<(usize, char)> = text.char_indices().collect();
        let pat_chars: Vec<char> = self.pattern.chars().collect();

        // Handle anchor ^
        let anchor_start = pat_chars.first() == Some(&'^');
        let pat_slice = if anchor_start {
            &pat_chars[1..]
        } else {
            &pat_chars[..]
        };

        if anchor_start {
            if let Some(match_len) = match_here(pat_slice, text, 0) {
                return Some(RegexMatch {
                    start: 0,
                    end: match_len,
                    text: String::from(&text[..match_len]),
                });
            }
            return None;
        }

        for (idx, _) in &char_indices {
            if let Some(match_len) = match_here(pat_slice, text, *idx) {
                return Some(RegexMatch {
                    start: *idx,
                    end: *idx + match_len,
                    text: String::from(&text[*idx..*idx + match_len]),
                });
            }
        }

        None
    }

    /// Finds all non-overlapping matches of the pattern in `text`.
    pub fn find_all(&self, text: &str) -> Vec<RegexMatch> {
        let mut matches = Vec::new();
        let mut curr_byte = 0;

        while curr_byte <= text.len() {
            if let Some(m) = self.find(&text[curr_byte..]) {
                if m.start == 0 && m.end == 0 {
                    // Empty match guard to avoid infinite loops
                    break;
                }
                let abs_start = curr_byte + m.start;
                let abs_end = curr_byte + m.end;
                matches.push(RegexMatch {
                    start: abs_start,
                    end: abs_end,
                    text: m.text,
                });
                curr_byte = if abs_end > curr_byte { abs_end } else { curr_byte + 1 };
            } else {
                break;
            }
        }

        matches
    }
}

/// Helper function to match pattern slice `pat` against `text` starting at byte offset `text_idx`.
fn match_here(pat: &[char], text: &str, text_idx: usize) -> Option<usize> {
    if pat.is_empty() {
        return Some(0);
    }

    if pat.len() == 1 && pat[0] == '$' {
        return if text_idx == text.len() { Some(0) } else { None };
    }

    let remaining_text = &text[text_idx..];
    let first_char = remaining_text.chars().next();

    // Check for quantifiers (*, ?)
    if pat.len() >= 2 && pat[1] == '*' {
        let target = pat[0];
        let rest_pat = &pat[2..];

        let mut text_char_indices: Vec<(usize, char)> = remaining_text.char_indices().collect();
        text_char_indices.push((remaining_text.len(), '\0'));

        let mut matched_bytes = 0;
        for i in 0..text_char_indices.len() {
            if let Some(rest_len) = match_here(rest_pat, text, text_idx + matched_bytes) {
                return Some(matched_bytes + rest_len);
            }

            if i < text_char_indices.len() - 1 {
                let (_offset, ch) = text_char_indices[i];
                if match_char(target, ch) {
                    let next_offset = text_char_indices[i + 1].0;
                    matched_bytes = next_offset;
                } else {
                    break;
                }
            }
        }
        return None;
    }

    if pat.len() >= 2 && pat[1] == '?' {
        let target = pat[0];
        let rest_pat = &pat[2..];

        if let Some(ch) = first_char {
            if match_char(target, ch) {
                let char_len = ch.len_utf8();
                if let Some(rest_len) = match_here(rest_pat, text, text_idx + char_len) {
                    return Some(char_len + rest_len);
                }
            }
        }
        return match_here(rest_pat, text, text_idx);
    }

    if let Some(ch) = first_char {
        if match_char(pat[0], ch) {
            let char_len = ch.len_utf8();
            if let Some(rest_len) = match_here(&pat[1..], text, text_idx + char_len) {
                return Some(char_len + rest_len);
            }
        }
    }

    None
}

fn match_char(pat_char: char, text_char: char) -> bool {
    if pat_char == '.' {
        return true;
    }
    pat_char == text_char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_basic() {
        let re = SovereignRegexEngine::new("hello");
        assert!(re.is_match("hello world"));
        assert!(!re.is_match("goodbye world"));
    }

    #[test]
    fn test_regex_wildcard_and_quantifiers() {
        let re = SovereignRegexEngine::new("c.t*");
        assert!(re.is_match("cat"));
        assert!(re.is_match("cuttttt"));

        let re2 = SovereignRegexEngine::new("^abc$");
        assert!(re2.is_match("abc"));
        assert!(!re2.is_match("abcd"));
    }
}
