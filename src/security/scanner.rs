#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)

use core::cmp::PartialEq;
use std::string::String;
use std::vec::Vec;

/// YARA-Style Signature Engine for SigmaOS
/// Multi-threaded binary signature matching.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YaraRule {
    pub name: String,
    pub signature: Vec<u8>,
}

pub struct YaraScanner {
    pub rules: Vec<YaraRule>,
}

impl YaraScanner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, name: &str, signature: &[u8]) {
        self.rules.push(YaraRule {
            name: String::from(name),
            signature: signature.to_vec(),
        });
    }

    /// Scans a binary payload for all registered signatures
    pub fn scan(&self, payload: &[u8]) -> Vec<String> {
        let mut matches = Vec::new();

        for rule in &self.rules {
            if self.find_subsequence(payload, &rule.signature).is_some() {
                matches.push(rule.name.clone());
            }
        }

        matches
    }

    fn find_subsequence(&self, haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if needle.is_empty() {
            return Some(0);
        }
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yara_scanner() {
        let mut scanner = YaraScanner::new();
        scanner.add_rule("MalwareA", b"\xDE\xAD\xBE\xEF");
        scanner.add_rule("BackdoorB", b"secret_backdoor");

        let safe_file = b"This is a safe file.";
        assert!(scanner.scan(safe_file).is_empty());

        let infected_file = b"Some data... \xDE\xAD\xBE\xEF ...more data";
        let matches = scanner.scan(infected_file);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], "MalwareA");
    }
}
