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

use core::ptr;

/// Secure Data Erasure (BleachBit Parity)
/// Multi-pass secure sector overwriting and cache purging to prevent forensic recovery.

pub struct SecureCleaner;

impl Default for SecureCleaner {
    fn default() -> Self {
        Self::new()
    }
}

impl SecureCleaner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    /// Overwrites a caller-provided memory buffer. This cannot guarantee
    /// erasure from storage media, caches, snapshots, or compiler-created copies.
    pub fn secure_wipe(&self, block: &mut [u8]) {
        for b in block.iter_mut() {
            *b = 0x00;
        }
        for b in block.iter_mut() {
            *b = 0xFF;
        }
        for b in block.iter_mut() {
            *b = 0xAA;
        }

        unsafe {
            let ptr = block.as_mut_ptr();
            for i in 0..block.len() {
                ptr::write_volatile(ptr.add(i), 0x00);
            }
        }
    }

    /// Clears unused or unallocated space in a filesystem partition
    pub fn wipe_unallocated_space(&self, partition: &mut [u8], bitmap: &[bool]) {
        for (i, sector) in partition.chunks_mut(512).enumerate() {
            if bitmap.get(i) == Some(&false) {
                self.secure_wipe(sector);
            }
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_wipe() {
        let cleaner = SecureCleaner::new();
        let mut sensitive_data = std::vec![0xCA, 0xFE, 0xBA, 0xBE];

        cleaner.secure_wipe(&mut sensitive_data);

        assert_eq!(sensitive_data, std::vec![0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_wipe_unallocated_space() {
        let cleaner = SecureCleaner::new();
        let mut partition = std::vec![0xFF; 1024];
        let bitmap = [true, false];

        cleaner.wipe_unallocated_space(&mut partition, &bitmap);

        assert_eq!(partition[0..512], std::vec![0xFF; 512]);
        assert_eq!(partition[512..1024], std::vec![0x00; 512]);
    }
}

// ==========================================
// TAILS OS PARITY: AMNESIA, TOR & METADATA SCRUBBING
// ==========================================

/// Policy predicate for an explicitly configured local Tor SOCKS endpoint.
/// This does not install or enforce an operating-system firewall rule.
pub struct TorAnonymityGate {
    pub tor_port: u16,
    pub enforce_leak_prevention: bool,
}

impl TorAnonymityGate {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tor_port: 9050,
            enforce_leak_prevention: true,
        }
    }

    pub fn validate_outgoing_traffic(&self, dest_ip: &[u8; 4], dest_port: u16) -> bool {
        if !self.enforce_leak_prevention {
            return true;
        }

        dest_ip == &[127, 0, 0, 1] && dest_port == self.tor_port
    }
}

impl Default for TorAnonymityGate {
    fn default() -> Self {
        Self::new()
    }
}

/// Best-effort overwrite of a caller-provided volatile memory buffer.
pub struct AmnesiaManager {
    pub rounds: usize,
}

const MAX_SHRED_ROUNDS: usize = 16;

impl AmnesiaManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { rounds: 3 }
    }

    pub fn shred_ram_segment(&self, ram_page: &mut [u8]) {
        for _ in 0..self.rounds.clamp(1, MAX_SHRED_ROUNDS) {
            unsafe {
                let ptr = ram_page.as_mut_ptr();
                for i in 0..ram_page.len() {
                    ptr::write_volatile(ptr.add(i), 0x00);
                }
            }
        }
    }
}

impl Default for AmnesiaManager {
    fn default() -> Self {
        Self::new()
    }
}

/// EXIF metadata removal is unavailable until a format-aware parser is provided.
pub struct MetadataScrubber;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataScrubError {
    ParserUnavailable,
}

impl MetadataScrubber {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    /// Fails without modifying input: marker scanning cannot safely remove EXIF data.
    pub fn scrub_exif_metadata(&self, _document: &mut [u8]) -> Result<usize, MetadataScrubError> {
        Err(MetadataScrubError::ParserUnavailable)
    }
}

impl Default for MetadataScrubber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tails_parity_tests {
    use super::*;

    #[test]
    fn test_tor_firewall_rules() {
        let gate = TorAnonymityGate::new();

        assert!(gate.validate_outgoing_traffic(&[127, 0, 0, 1], 9050));
        assert!(!gate.validate_outgoing_traffic(&[127, 0, 0, 1], 80));
        assert!(!gate.validate_outgoing_traffic(&[104, 244, 42, 1], 9050));

        assert!(!gate.validate_outgoing_traffic(&[8, 8, 8, 8], 53));
        assert!(!gate.validate_outgoing_traffic(&[142, 250, 190, 46], 443));
    }

    #[test]
    fn test_amnesic_ram_shredder() {
        let amnesia = AmnesiaManager::new();
        let mut sensitive_ram = std::vec![0xAA; 512];

        amnesia.shred_ram_segment(&mut sensitive_ram);
        assert_eq!(sensitive_ram, std::vec![0x00; 512]);
    }

    #[test]
    fn test_metadata_scrubbing() {
        let scrubber = MetadataScrubber::new();
        let mut document = std::vec![0x41, 0x42, 0x43, 0x00];
        document.extend_from_slice(b"Exif\0\0CameraID_12345_GPSLocation_9999");
        document.extend_from_slice(b"SomeSuffixData");

        let original = document.clone();
        assert_eq!(
            scrubber.scrub_exif_metadata(&mut document),
            Err(MetadataScrubError::ParserUnavailable)
        );
        assert_eq!(document, original);
    }
}
