// SigmaOS Custom Hash Library
// Reduces dependency on predefined hashing functions

#![no_std]

/// Simple hash function for strings (DJB2 algorithm)
pub fn djb2_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in s.bytes() {
        hash = ((hash << 5) + hash) + byte as u64;
    }
    hash
}

/// Simple hash function for byte arrays
pub fn simple_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for &byte in data {
        hash = ((hash << 5) + hash) + byte as u64;
    }
    hash
}

/// XOR-based hash for integers
pub fn xor_hash(value: u64) -> u64 {
    value ^ 0x517cc1b727220a95
}

/// Simple FNV-1a hash implementation
pub fn fnv1a_hash(data: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;

    let mut hash = FNV_OFFSET_BASIS;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Simple hash structure for custom hashing
pub struct SimpleHasher {
    state: u64,
}

impl SimpleHasher {
    pub fn new() -> Self {
        SimpleHasher { state: 5381 }
    }

    pub fn write(&mut self, byte: u8) {
        self.state = self.state.wrapping_shl(5)
            .wrapping_add(self.state)
            .wrapping_add(byte as u64);
    }

    pub fn finish(&self) -> u64 {
        self.state
    }
}

impl Default for SimpleHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl core::hash::Hasher for SimpleHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.wrapping_shl(5)
                .wrapping_add(self.state)
                .wrapping_add(byte as u64);
        }
    }
}

/// Combine two hash values
pub fn combine_hashes(a: u64, b: u64) -> u64 {
    a.wrapping_mul(31).wrapping_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_djb2_hash() {
        let hash1 = djb2_hash("hello");
        let hash2 = djb2_hash("hello");
        assert_eq!(hash1, hash2);

        let hash3 = djb2_hash("world");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_simple_hash() {
        let data = b"test";
        let hash1 = simple_hash(data);
        let hash2 = simple_hash(data);
        assert_eq!(hash1, hash2);

        let data2 = b"test2";
        assert_ne!(hash1, simple_hash(data2));
    }

    #[test]
    fn test_fnv1a_hash() {
        let data = b"test";
        let hash1 = fnv1a_hash(data);
        let hash2 = fnv1a_hash(data);
        assert_eq!(hash1, hash2);

        let data2 = b"test2";
        assert_ne!(hash1, fnv1a_hash(data2));
    }

    #[test]
    fn test_combine_hashes() {
        let hash1 = combine_hashes(100, 200);
        let hash2 = combine_hashes(200, 100);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_simple_hasher() {
        let mut hasher = SimpleHasher::new();
        hasher.write(b't');
        hasher.write(b'e');
        hasher.write(b's');
        hasher.write(b't');
        let hash = hasher.finish();

        let mut hasher2 = SimpleHasher::new();
        hasher2.write(b't');
        hasher2.write(b'e');
        hasher2.write(b's');
        hasher2.write(b't');
        assert_eq!(hash, hasher2.finish());
    }

    #[test]
    fn test_xor_hash() {
        let hash1 = xor_hash(12345);
        let hash2 = xor_hash(12345);
        assert_eq!(hash1, hash2);

        let hash3 = xor_hash(54321);
        assert_ne!(hash1, hash3);
    }
}
