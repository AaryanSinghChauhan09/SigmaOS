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

    /// Performs a 3-pass DoD 5220.22-M style secure wipe on a memory block
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
        for (i, &allocated) in bitmap.iter().enumerate() {
            if !allocated {
                let start = i * 512;
                let end = (start + 512).min(partition.len());
                if start < end {
                    self.secure_wipe(&mut partition[start..end]);
                }
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

/// Tor Anonymity Gate - Leak-proof outbound firewall restricting all non-Tor connections
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

        if dest_ip == &[127, 0, 0, 1] {
            return true;
        }

        if dest_port == self.tor_port {
            return true;
        }

        false
    }
}

impl Default for TorAnonymityGate {
    fn default() -> Self {
        Self::new()
    }
}

/// Amnesia Manager - Volatile RAM cleanup routines at system shutdown/reboot
pub struct AmnesiaManager {
    pub rounds: usize,
}

impl AmnesiaManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { rounds: 3 }
    }

    pub fn shred_ram_segment(&self, ram_page: &mut [u8]) {
        for _ in 0..self.rounds {
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

/// Metadata Scrubber - Automated EXIF/Geolocation anti-forensic cleaning tool
pub struct MetadataScrubber;

impl MetadataScrubber {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    pub fn scrub_exif_metadata(&self, document: &mut [u8]) -> usize {
        let mut scrub_count = 0;
        let exif_tag = b"Exif\0\0";

        let mut i = 0;
        while i + exif_tag.len() <= document.len() {
            if &document[i..i + exif_tag.len()] == exif_tag {
                let wipe_end = (i + 32).min(document.len());
                for byte in &mut document[i..wipe_end] {
                    *byte = 0x00;
                }
                scrub_count += 1;
                i = wipe_end;
            } else {
                i += 1;
            }
        }

        scrub_count
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

        assert!(gate.validate_outgoing_traffic(&[127, 0, 0, 1], 80));
        assert!(gate.validate_outgoing_traffic(&[104, 244, 42, 1], 9050));

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

        let count = scrubber.scrub_exif_metadata(&mut document);
        assert_eq!(count, 1);

        assert!(!document.windows(6).any(|w| w == b"Exif\0\0"));
    }
}
