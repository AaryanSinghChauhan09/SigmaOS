use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// SigmaOS Custom String Parser
// Reduces dependency on std string parsing functions

use core::iter::Iterator;
use core:: String::Chars;

/// Custom string parser for common operations
pub struct StringParser<'a> {
    chars: Chars<'a>,
    current: Option<char>,
}

impl<'a> StringParser<'a> {
    /// Create new string parser
    pub fn new(s: &'a str) -> Self {
        let mut chars = s.chars();
        let current = chars.next();
        Self { chars, current }
    }

    /// Peek at current character
    pub fn peek(&self) -> Option<char> {
        self.current
    }

    /// Consume current character
    pub fn consume(&mut self) -> Option<char> {
        let c = self.current;
        self.current = self.chars.next();
        c
    }

    /// Check if current character matches
    pub fn matches(&mut self, expected: char) -> bool {
        if self.current == Some(expected) {
            self.consume();
            true
        } else {
            false
        }
    }

    /// Skip whitespace
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current {
            if c.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }

    /// Parse until character
    pub fn parse_until(&mut self, delimiter: char) -> String {
        let mut result = String::new();
        while let Some(c) = self.current {
            if c == delimiter {
                break;
            }
            result.push(c);
            self.consume();
        }
        result
    }

    /// Parse while character matches predicate
    pub fn parse_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(c) = self.current {
            if predicate(c) {
                result.push(c);
                self.consume();
            } else {
                break;
            }
        }
        result
    }

    /// Parse digits
    pub fn parse_digits(&mut self) -> String {
        self.parse_while(|c| c.is_ascii_digit())
    }

    /// Parse letters
    pub fn parse_letters(&mut self) -> String {
        self.parse_while(|c| c.is_ascii_alphabetic())
    }

    /// Parse alphanumeric
    pub fn parse_alphanumeric(&mut self) -> String {
        self.parse_while(|c| c.is_ascii_alphanumeric())
    }

    /// Check if at end
    pub fn is_end(&self) -> bool {
        self.current.is_none()
    }
}

/// Custom string splitter
pub fn split_string(s: &str, delimiter: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for c in s.chars() {
        if c == delimiter {
            result.push(current);
            current = String::new();
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

/// Custom string trim
pub fn trim_string(s: &str) -> &str {
    let start = s.chars().take_while(|c| c.is_whitespace()).count();
    let end = s.chars().rev().take_while(|c| c.is_whitespace()).count();
    &s[start..s.len() - end]
}

/// Custom string to lowercase
pub fn to_lowercase(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_lowercase()).collect()
}

/// Custom string to uppercase
pub fn to_uppercase(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}

/// Custom string contains
pub fn contains_string(haystack: &str, needle: &str) -> bool {
    haystack.find(needle).is_some()
}

/// Custom string starts with
pub fn starts_with_string(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

/// Custom string ends with
pub fn ends_with_string(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// Custom string replace
pub fn replace_string(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_parser() {
        let mut parser = StringParser::new("hello world");
        assert_eq!(parser.parse_letters(), "hello");
        parser.skip_whitespace();
        assert_eq!(parser.parse_letters(), "world");
    }

    #[test]
    fn test_split_string() {
        let result = split_string("a,b,c", ',');
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "a");
        assert_eq!(result[1], "b");
        assert_eq!(result[2], "c");
    }

    #[test]
    fn test_trim_string() {
        assert_eq!(trim_string("  hello  "), "hello");
        assert_eq!(trim_string("hello"), "hello");
    }

    #[test]
    fn test_case_conversion() {
        assert_eq!(to_lowercase("HELLO"), "hello");
        assert_eq!(to_uppercase("hello"), "HELLO");
    }

    #[test]
    fn test_string_contains() {
        assert!(contains_string("hello world", "world"));
        assert!(!contains_string("hello world", "xyz"));
    }

    #[test]
    fn test_string_starts_ends() {
        assert!(starts_with_string("hello world", "hello"));
        assert!(ends_with_string("hello world", "world"));
    }
}
