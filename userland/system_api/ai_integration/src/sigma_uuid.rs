//! SigmaOS Native UUID Module
//! Replaces uuid dependency with simple UUID v4 generation

#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

/// UUID structure (16 bytes)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SigmaUuid {
    pub data: [u8; 16],
}

impl SigmaUuid {
    /// Create a new random UUID v4
    pub fn new_v4() -> Self {
        let mut uuid = Self { data: [0u8; 16] };
        
        static COUNTER: AtomicU32 = AtomicU32::new(0x12345678);
        let seed = COUNTER.fetch_add(1, Ordering::SeqCst);
        
        let mut state = seed;
        for i in 0..16 {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            uuid.data[i] = (state >> 16) as u8;
        }
        
        uuid.data[6] = (uuid.data[6] & 0x0F) | 0x40;
        uuid.data[8] = (uuid.data[8] & 0x3F) | 0x80;
        
        uuid
    }
    
    /// Create UUID from bytes
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self { data: bytes }
    }
    
    /// Convert UUID to bytes
    pub fn to_bytes(&self) -> [u8; 16] {
        self.data
    }
    
    /// Convert UUID to string (hyphenated format)
    pub fn to_string(&self, buf: &mut [u8]) -> usize {
        let hex = b"0123456789abcdef";
        let mut pos = 0;
        
        for i in 0..16 {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                if pos < buf.len() {
                    buf[pos] = b'-';
                    pos += 1;
                }
            }
            
            let byte = self.data[i];
            if pos + 1 < buf.len() {
                buf[pos] = hex[(byte >> 4) as usize];
                buf[pos + 1] = hex[(byte & 0x0F) as usize];
                pos += 2;
            }
        }
        
        pos
    }
    
    /// Parse UUID from string
    pub fn from_str(s: &str) -> Option<Self> {
        let mut uuid = Self { data: [0u8; 16] };
        let mut byte_idx = 0;
        let mut nibble = 0;
        
        for c in s.bytes() {
            match c {
                b'-' => continue,
                b'0'..=b'9' => {
                    uuid.data[byte_idx] |= (c - b'0') << (4 - nibble * 4);
                }
                b'a'..=b'f' => {
                    uuid.data[byte_idx] |= (c - b'a' + 10) << (4 - nibble * 4);
                }
                b'A'..=b'F' => {
                    uuid.data[byte_idx] |= (c - b'A' + 10) << (4 - nibble * 4);
                }
                _ => return None,
            }
            
            nibble += 1;
            if nibble == 2 {
                nibble = 0;
                byte_idx += 1;
                if byte_idx >= 16 {
                    break;
                }
            }
        }
        
        Some(uuid)
    }
    
    /// Check if UUID is nil (all zeros)
    pub fn is_nil(&self) -> bool {
        self.data.iter().all(|&b| b == 0)
    }
}

impl Default for SigmaUuid {
    fn default() -> Self {
        Self::new_v4()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_v4() {
        let uuid = SigmaUuid::new_v4();
        assert!(!uuid.is_nil());
        assert_eq!(uuid.data[6] & 0xF0, 0x40);
        assert_eq!(uuid.data[8] & 0xC0, 0x80);
    }

    #[test]
    fn test_uuid_roundtrip() {
        let uuid1 = SigmaUuid::new_v4();
        let bytes = uuid1.to_bytes();
        let uuid2 = SigmaUuid::from_bytes(bytes);
        assert_eq!(uuid1, uuid2);
    }

    #[test]
    fn test_uuid_string() {
        let uuid = SigmaUuid::new_v4();
        let mut buf = [0u8; 37];
        let len = uuid.to_string(&mut buf);
        assert!(len == 36);
    }
}
