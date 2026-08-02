#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Custom UUID Generator
// Reduces dependency on predefined libraries by implementing custom UUID generation

// (no_std only applicable at crate root - removed)

use core::sync::atomic::{AtomicU64, Ordering};

/// Simple UUID structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uuid {
    pub data: [u8; 16],
}

impl Uuid {
    /// Generate a new UUID using simple counter-based approach
    /// In production, this should use proper entropy sources
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);

        let mut data = [0u8; 16];

        // Simple deterministic UUID generation based on counter
        // In a real system, this would use hardware entropy
        let bytes = counter.to_le_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            data[i] = byte;
        }

        // Set version bits (UUID v4)
        data[6] = (data[6] & 0x0F) | 0x40; // Version 4
        data[8] = (data[8] & 0x3F) | 0x80; // Variant 1

        Uuid { data }
    }

    /// Create UUID from bytes
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Uuid { data: bytes }
    }

    /// Convert UUID to bytes
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.data
    }

    /// Convert UUID to string representation
    pub fn to_string(&self) -> String {
        // Format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
        let mut result = String::new();

        for i in 0..16 {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                result.push('-');
            }
            result.push(char::from_digit(self.data[i] as u32 >> 4, 16).unwrap_or('0'));
            result.push(char::from_digit(self.data[i] as u32 & 0x0F, 16).unwrap_or('0'));
        }

        result
    }

    /// Parse UUID from string
    pub fn parse_str(s: &str) -> Option<Self> {
        let chars: Vec<char> = s.chars().filter(|c| *c != '-').collect();
        if chars.len() != 32 {
            return None;
        }

        let mut data = [0u8; 16];
        for i in 0..16 {
            let high = char::to_digit(chars[i * 2], 16)? as u8;
            let low = char::to_digit(chars[i * 2 + 1], 16)? as u8;
            data[i] = (high << 4) | low;
        }

        Some(Uuid { data })
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_generation() {
        let uuid1 = Uuid::new();
        let uuid2 = Uuid::new();
        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_uuid_from_bytes() {
        let bytes = [1u8; 16];
        let uuid = Uuid::from_bytes(bytes);
        assert_eq!(uuid.as_bytes(), &bytes);
    }

    #[test]
    fn test_uuid_to_string() {
        let uuid = Uuid::new();
        let s = uuid.to_string();
        assert_eq!(s.len(), 36); // 32 hex chars + 4 hyphens
        assert!(s.contains('-'));
    }

    #[test]
    fn test_uuid_parse_str() {
        let uuid = Uuid::new();
        let s = uuid.to_string();
        let parsed = Uuid::parse_str(&s);
        assert!(parsed.is_some());
        assert_eq!(parsed.unwrap(), uuid);
    }
}
