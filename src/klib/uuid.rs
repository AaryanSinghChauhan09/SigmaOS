use alloc::format;
extern crate alloc;
// Simple UUID implementation for SigmaOS
// Reduces dependency on external uuid crate


use alloc::string::String;
use core::fmt;

/// Simple UUID v4 implementation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uuid {
    pub bytes: [u8; 16],
}

impl Uuid {
    /// Generate a new UUID v4 as alias for new
    pub fn new() -> Self {
        Self::new_v4()
    }

    /// Generate a new UUID v4
    pub fn new_v4() -> Self {
        // Simple pseudo-random UUID v4
        // In a real implementation, this would use a proper CSPRNG
        let mut bytes = [0u8; 16];

        // Use time-based seed for pseudo-randomness
        let seed = Self::get_seed();
        let mut state = seed;

        for i in 0..16 {
            state = Self::xorshift(state);
            bytes[i] = (state & 0xFF) as u8;
        }

        // Set version bits (UUID v4)
        bytes[6] = (bytes[6] & 0x0F) | 0x40;
        // Set variant bits
        bytes[8] = (bytes[8] & 0x3F) | 0x80;

        Self { bytes }
    }

    /// Simple XORShift PRNG for seeding
    fn xorshift(mut state: u64) -> u64 {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state
    }

    /// Get a simple seed based on time (would be replaced with proper RNG)
    fn get_seed() -> u64 {
        // This is a placeholder - in a real kernel, this would use
        // hardware RNG or proper entropy sources
        0x123456789ABCDEF0
    }

    /// Get UUID as hyphenated string
    pub fn to_hyphenated(&self) -> String {
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3],
            self.bytes[4], self.bytes[5],
            self.bytes[6], self.bytes[7],
            self.bytes[8], self.bytes[9],
            self.bytes[10], self.bytes[11], self.bytes[12], self.bytes[13], self.bytes[14], self.bytes[15]
        )
    }

    /// Get UUID as bytes
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hyphenated())
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::new_v4()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_generation() {
        let uuid = Uuid::new_v4();
        assert_eq!(uuid.bytes.len(), 16);
    }

    #[test]
    fn test_uuid_format() {
        let uuid = Uuid::new_v4();
        let formatted = uuid.to_hyphenated();
        assert_eq!(formatted.len(), 36); // 8-4-4-4-12 format
        assert!(formatted.contains('-'));
    }
}
