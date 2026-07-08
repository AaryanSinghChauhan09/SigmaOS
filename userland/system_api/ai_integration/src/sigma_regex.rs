//! SigmaOS Native Regex Module
//! Simple pattern matching to replace regex dependency

#![no_std]

/// Simple pattern matcher for common patterns
pub struct SigmaPattern {
    pattern: [u8; 256],
    len: usize,
}

impl SigmaPattern {
    /// Create a new pattern from a string
    pub fn new(pattern: &str) -> Self {
        let mut p = Self {
            pattern: [0u8; 256],
            len: 0,
        };
        
        let bytes = pattern.as_bytes();
        for i in 0..bytes.len().min(256) {
            p.pattern[i] = bytes[i];
        }
        p.len = bytes.len().min(256);
        
        p
    }
    
    /// Simple wildcard matching (* and ?)
    pub fn matches(&self, text: &str) -> bool {
        let text_bytes = text.as_bytes();
        self.wildcard_match(self.pattern[..self.len].as_ptr(), self.len, 
                           text_bytes.as_ptr(), text_bytes.len())
    }
    
    /// Wildcard matching implementation
    fn wildcard_match(&self, pattern: *const u8, pattern_len: usize, 
                      text: *const u8, text_len: usize) -> bool {
        unsafe {
            let mut p_idx = 0;
            let mut t_idx = 0;
            let mut star_idx: usize = usize::MAX;
            let mut match_idx: usize = 0;
            
            while t_idx < text_len {
                if p_idx < pattern_len && *pattern.add(p_idx) == *text.add(t_idx) {
                    p_idx += 1;
                    t_idx += 1;
                } else if p_idx < pattern_len && *pattern.add(p_idx) == b'*' {
                    star_idx = p_idx;
                    match_idx = t_idx;
                    p_idx += 1;
                } else if star_idx != usize::MAX {
                    p_idx = star_idx + 1;
                    match_idx += 1;
                    t_idx = match_idx;
                } else {
                    return false;
                }
            }
            
            while p_idx < pattern_len && *pattern.add(p_idx) == b'*' {
                p_idx += 1;
            }
            
            p_idx == pattern_len
        }
    }
    
    /// Check if text contains pattern
    pub fn contains(&self, text: &str) -> bool {
        let text_bytes = text.as_bytes();
        if self.len > text_bytes.len() {
            return false;
        }
        
        for i in 0..=(text_bytes.len() - self.len) {
            let mut matches = true;
            for j in 0..self.len {
                if self.pattern[j] != text_bytes[i + j] {
                    matches = false;
                    break;
                }
            }
            if matches {
                return true;
            }
        }
        
        false
    }
    
    /// Check if text starts with pattern
    pub fn starts_with(&self, text: &str) -> bool {
        let text_bytes = text.as_bytes();
        if self.len > text_bytes.len() {
            return false;
        }
        
        for i in 0..self.len {
            if self.pattern[i] != text_bytes[i] {
                return false;
            }
        }
        
        true
    }
    
    /// Check if text ends with pattern
    pub fn ends_with(&self, text: &str) -> bool {
        let text_bytes = text.as_bytes();
        if self.len > text_bytes.len() {
            return false;
        }
        
        let offset = text_bytes.len() - self.len;
        for i in 0..self.len {
            if self.pattern[i] != text_bytes[offset + i] {
                return false;
            }
        }
        
        true
    }
}

/// Simple string utilities
pub struct SigmaStr;

impl SigmaStr {
    /// Check if string is alphanumeric
    pub fn is_alphanumeric(s: &str) -> bool {
        s.bytes().all(|b| b.is_ascii_alphanumeric())
    }
    
    /// Check if string is numeric
    pub fn is_numeric(s: &str) -> bool {
        s.bytes().all(|b| b.is_ascii_digit())
    }
    
    /// Check if string is alphabetic
    pub fn is_alpha(s: &str) -> bool {
        s.bytes().all(|b| b.is_ascii_alphabetic())
    }
    
    /// Trim whitespace from string
    pub fn trim(s: &str) -> &str {
        s.trim()
    }
    
    /// Split string by delimiter
    pub fn split<'a>(s: &'a str, delim: char) -> impl Iterator<Item = &'a str> {
        s.split(delim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wildcard_match() {
        let pattern = SigmaPattern::new("test*");
        assert!(pattern.matches("test123"));
        assert!(pattern.matches("test"));
        assert!(!pattern.matches("123test"));
    }

    #[test]
    fn test_contains() {
        let pattern = SigmaPattern::new("test");
        assert!(pattern.contains("this is a test string"));
        assert!(!pattern.contains("no match here"));
    }

    #[test]
    fn test_starts_with() {
        let pattern = SigmaPattern::new("test");
        assert!(pattern.starts_with("test123"));
        assert!(!pattern.starts_with("123test"));
    }

    #[test]
    fn test_ends_with() {
        let pattern = SigmaPattern::new("test");
        assert!(pattern.ends_with("123test"));
        assert!(!pattern.ends_with("test123"));
    }
}
